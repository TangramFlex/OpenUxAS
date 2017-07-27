//! # PlanBuilderService
//!
//! A component that constructs plans from assignments.
//!
//! 1. For each assigned task option, request calculation of final waypoint plan in order
//!
//! 2. Construct resulting waypoint plans and send automation response.
//!
//! MESSAGES
//!
//! - ==> TaskAssignmentSummary
//!
//! FOR EVERY TASK
//!
//! - <== TaskImplementationRequest
//! - ==> TaskImplementationResponse
//! - <== AutomationResponse
//!
//! ## Configuration String
//! `<Service Type="PlanBuilderService" AssignmentStartPointLead_m="50.0" />`
//!
//! ## Options
//! - AssignmentStartPointLead_m
//!
//! ## Subscribed Messages
//! - uxas::messages::task::UniqueAutomationRequest
//! - uxas::messages::task::TaskAssignmentSummary
//! - uxas::messages::task::TaskImplementationResponse
//! - afrl::cmasi::AirVehicleState
//! - afrl::impact::GroundVehicleState
//! - afrl::impact::SurfaceVehicleState
//!
//! ## Sent Messages
//! - afrl::cmasi::ServiceStatus
//! - uxas::messages::task::TaskImplementationRequest
//! - uxas::messages::task::UniqueAutomationResponse

use lmcp::*;
use lmcp::afrl::cmasi::air_vehicle_state::*;
use lmcp::afrl::cmasi::entity_state::*;
use lmcp::afrl::cmasi::key_value_pair::*;
use lmcp::afrl::cmasi::location3d::*;
use lmcp::afrl::cmasi::mission_command::*;
use lmcp::afrl::cmasi::service_status::*;
use lmcp::afrl::cmasi::service_status_type::*;
use lmcp::afrl::impact::ground_vehicle_state::*;
use lmcp::afrl::impact::surface_vehicle_state::*;
use lmcp::avtas::lmcp::*;
use lmcp::uxas::messages::task::planning_state::*;
use lmcp::uxas::messages::task::task_assignment::*;
use lmcp::uxas::messages::task::task_assignment_summary::*;
use lmcp::uxas::messages::task::task_implementation_request::*;
use lmcp::uxas::messages::task::task_implementation_response::*;
use lmcp::uxas::messages::task::unique_automation_request::*;
use lmcp::uxas::messages::task::unique_automation_response::*;

use client::{Client};
use unit_conversions::*;

use std::collections::*;
use std::thread;

macro_rules! debug_println {
    () =>
        (if cfg!(debug_assertions) { println!("[plan_builder]\n") });
    ($fmt:expr) =>
        (if cfg!(debug_assertions) { println!(concat!("[plan_builder] ", $fmt)) });
    ($fmt:expr, $($arg:tt)*) =>
        (if cfg!(debug_assertions) { println!(concat!("[plan_builder] ", $fmt), $($arg)*) });
}

/// Opaque pointer to the C++ PlanBuilderService object
pub enum PlanBuilderService {}

pub struct PlanBuilder {
    /// Unique automation requests keyed by unique automation request ID
    unique_automation_requests: HashMap<i64, UniqueAutomationRequest>,

    /// In-progress unique automation responses keyed by unique automation request ID
    in_progress_response: HashMap<i64, UniqueAutomationResponse>,

    /// Task assignment summaries keyed by unique automation request ID
    assignment_summaries: HashMap<i64, TaskAssignmentSummary>,

    /// Projected entity states keyed by corresponding unique automation request ID
    projected_entity_states: HashMap<i64, Vec<ProjectedState>>,

    /// Assignments yet to be completed, keyed by corresponding unique automation request ID
    remaining_assignments: HashMap<i64, VecDeque<Box<TaskAssignmentT>>>,

    /// Unique automation request IDs keyed by pending task implementation request ID
    expected_response_ids: HashMap<i64, i64>,

    /// Latest entity states, used to get starting heading, position, and time; keyed by entity ID
    entity_states: HashMap<i64, Box<EntityStateT>>,

    /// The last TaskImplementationId (and StartingWaypointId) sent out. It is incremented by 1 for
    /// each new ID. It is reset to zero each time a new TaskAssignmentSummary is received.
    task_implementation_id: i64,

    /// CommandId used in the last mission command. Incremented by 1 for each new mission
    /// command. Assume this ID will be unique during the lifetime run of the PlanBuilder
    command_id: i64,

    /// Distance in meters to add to the position of the vehicle in the direction the vehicle is
    /// heading to calculate the starting point for new plans
    assignment_start_point_lead_m: f64,

    /// Unit conversion state initialized with the scenario's initial latitude and longitude
    unit_converter: UnitConverter,
}

#[derive(Clone, Debug)]
pub struct ProjectedState {
    state: PlanningState,
    final_waypoint_id: i64,
    /// ms since epoch
    time: i64,
}

impl Default for PlanBuilder {
    fn default() -> PlanBuilder {
        PlanBuilder {
            unique_automation_requests: HashMap::default(),
            in_progress_response: HashMap::default(),
            assignment_summaries: HashMap::default(),
            projected_entity_states: HashMap::default(),
            remaining_assignments: HashMap::default(),
            expected_response_ids: HashMap::default(),
            entity_states: HashMap::default(),
            task_implementation_id: 1,
            command_id: 1,
            assignment_start_point_lead_m: 50.0,
            unit_converter: UnitConverter::default(),
        }
    }
}

impl PlanBuilder {
    pub fn run(client: Client, lead: f64, init_lat_deg: f64, init_long_deg: f64) -> thread::JoinHandle<()> { thread::spawn(move|| {
        let mut pb = PlanBuilder::default();

        pb.assignment_start_point_lead_m = lead;
        pb.unit_converter = UnitConverter::new(init_lat_deg.to_radians(), init_long_deg.to_radians());

        let subs = [
            UniqueAutomationRequest::subscription(),
            TaskAssignmentSummary::subscription(),
            TaskImplementationResponse::subscription(),
            AirVehicleState::subscription(),
            GroundVehicleState::subscription(),
            SurfaceVehicleState::subscription(),
        ];
        for sub in subs.iter() {
            client.add_subscription(sub.as_bytes()).unwrap();
        }

        loop {
            let msg = client.receive().unwrap();
            match msg {
                LmcpType::TaskAssignmentSummary(tas) => {
                    debug_println!("Received message! {:?}", tas);
                    
                    match pb.process_task_assignment_summary(tas) {
                        Ok(msgs) => {
                            for msg in &msgs {
                                client.broadcast(msg).unwrap();
                            }
                        },
                        Err(msg) => {
                            debug_println!("plan_builder: process_task_assignment_summary failed");
                            client.send_error(msg);
                        },
                    };
                }
                LmcpType::TaskImplementationResponse(tir) => {
                    debug_println!("Received message! {:?}", tir);
                    
                    match pb.process_task_implementation_response(tir) {
                        Ok(msgs) => {
                            for msg in &msgs {
                                client.broadcast(msg).unwrap();
                            }
                        },
                        Err(()) => {
                            debug_println!(
                                "plan_builder: process_task_implementation_response failed");
                        },
                    };
                }
                LmcpType::AirVehicleState(vs) => {
                    pb.entity_states.insert(vs.id, Box::new(vs));
                }
                LmcpType::GroundVehicleState(vs) => {
                    pb.entity_states.insert(vs.id, Box::new(vs));
                }
                LmcpType::SurfaceVehicleState(vs) => {
                    pb.entity_states.insert(vs.id, Box::new(vs));
                }
                LmcpType::UniqueAutomationRequest(uar) => {
                    debug_println!("Received message! {:?}", uar);
                    let id = uar.request_id;
                    pb.unique_automation_requests.insert(id, uar);
                    // re-initialize state maps, possibly halting completion of an overridden
                    // automation request
                    pb.assignment_summaries.insert(id, TaskAssignmentSummary::default());
                    pb.projected_entity_states.insert(id, Vec::new());
                    pb.remaining_assignments.insert(id, VecDeque::new());
                    pb.in_progress_response.insert(id, UniqueAutomationResponse::default());
                }
                _ => debug_println!("Unhandled LMCP message {:?}", msg),
            }
        }
    })}
}

impl PlanBuilder {
    fn process_task_assignment_summary(
        &mut self,
        tas: TaskAssignmentSummary,
    ) -> Result<Vec<LmcpType>, String> {
        let err_pfx = "ERROR::process_task_assignment_summary:";
        let car_id = tas.corresponding_automation_request_id;

        {
            // validate that this summary corresponds to an existing unique automation request
            let err_msg = format!(
                "{} Corresponding Unique Automation Request ID [{}] not found!",
                err_pfx, car_id
            );
            let car = self.unique_automation_requests.get(&car_id).ok_or(err_msg)?;

            // ensure that a valid state for each vehicle in the request has been received
            for v in car.original_request.entity_list() {
                if let None = self.entity_states.get(&v) {
                    return Err(format!(
                        "{} Corresponding Unique Automation Request included vehicle ID [{}] \
                         which does not have a corresponding current state!",
                        err_pfx, v
                    ));
                }
            }

            // queue up all task assignments to be made
            self.remaining_assignments.insert(
                car_id, VecDeque::from(tas.task_list.clone()),
            );

            // initialize state tracking maps with the current request IDs
            self.assignment_summaries.insert(car_id, tas);
            self.in_progress_response.insert(
                car_id, UniqueAutomationResponse { response_id: car_id, ..Default::default() }
            );

            // project states and save them
            let pes = self.project_entity_states(&car);
            self.projected_entity_states.insert(car_id, pes);
        }

        // send the next request, or send no message if the next request fails
        self.send_next_task_implementation_request(car_id).or(Ok(Vec::new()))
    }

    fn project_entity_states(&self, car: &UniqueAutomationRequest) -> Vec<ProjectedState> {
        // list all participating vehicles in the assignment
        let participating_vehicles: Vec<i64> = if car.original_request.entity_list().is_empty() {
            self.entity_states.keys().map(|&x| x).collect()
        } else {
            car.original_request.entity_list().clone()
        };

        // project states
        let participating_vehicle_states = participating_vehicles.iter().map(|&x| {
            self.entity_states.get(&x)
        });
        participating_vehicles
            .iter()
            .zip(participating_vehicle_states)
            .map(|(v, oes)| {
                let entity_state = oes.expect("ensured to exist by above validation");
                let ps_state =
                    if let Some(ps) = car.planning_states.iter().find(|&ps| v == &ps.entity_id()) {
                        ps.as_planning_state().expect("no subtypes of PlanningState").clone()
                    } else {
                        // add in the assignment start point lead distance
                        let pos0 = entity_state.location();
                        let hdg = entity_state.heading();
                        let NorthEastM(north_m0, east_m0) =
                            LatLongDeg(pos0.latitude(), pos0.longitude()).to_north_east_m(&self.unit_converter);
                        let north_m = north_m0 +
                            self.assignment_start_point_lead_m * (hdg as f64).to_radians().cos();
                        let east_m = east_m0 +
                            self.assignment_start_point_lead_m * (hdg as f64).to_radians().sin();
                        let LatLongDeg(lat_deg, long_deg) =
                            NorthEastM(north_m, east_m).to_lat_long_deg(&self.unit_converter);
                        let position = Location3D {
                            latitude: lat_deg,
                            longitude: long_deg,
                            altitude: pos0.altitude(),
                            altitude_type: pos0.altitude_type(),
                        };
                        PlanningState {
                            entity_id: *v,
                            planning_heading: hdg,
                            planning_position: Box::new(position),
                        }
                    };
                ProjectedState {
                    final_waypoint_id: 0,
                    time: entity_state.time(),
                    state: ps_state,
                }
            })
            .collect()
    }

    fn send_next_task_implementation_request(
        &mut self,
        id: i64,
    ) -> Result<Vec<LmcpType>, ()> {
        debug_println!("entering send_next_task_implementation_request");
        let tir = {
            let uar = self.unique_automation_requests.get(&id).ok_or(())?;
            let ra = self.remaining_assignments.get_mut(&id).ok_or(())?;
            let pes = self.projected_entity_states.get(&id).ok_or(())?;

            let ta = ra.pop_front().ok_or(())?;

            let ps = pes.iter().find(|&ps| {
                ps.state.entity_id() == ta.assigned_vehicle()
            }).ok_or(())?;

            self.expected_response_ids.insert(self.task_implementation_id, id);

            let neighbors = pes.iter().filter_map(|ref neighbor| {
                if neighbor.state.entity_id() != ps.state.entity_id() {
                    Some(Box::new(neighbor.state.clone()) as Box<PlanningStateT>)
                } else {
                    None
                }
            }).collect();
            TaskImplementationRequest {
                corresponding_automation_request_id: id,
                request_id: self.task_implementation_id,
                starting_waypoint_id: ps.final_waypoint_id + 1,
                vehicle_id: ta.assigned_vehicle(),
                task_id: ta.task_id(),
                option_id: ta.option_id(),
                region_id: uar.original_request.operating_region(),
                time_threshold: ta.time_threshold(),
                start_heading: ps.state.planning_heading,
                start_position: ps.state.planning_position.clone(),
                start_time: ps.time,
                neighbor_locations: neighbors,
            }
        };

        // do this outside of the block above in order to satisfy the borrow checker :(
        self.next_implementation_id();

        // send the message
        Ok(vec![LmcpType::TaskImplementationRequest(tir)])
    }
}

impl PlanBuilder {
    fn process_task_implementation_response(
        &mut self,
        tiresp: TaskImplementationResponse,
    ) -> Result<Vec<LmcpType>, ()> {
        debug_println!(
            "entering process_task_implementation_response with id {}", tiresp.response_id);
        // check response ID
        let unique_request_id = *self.expected_response_ids.get(&tiresp.response_id).ok_or(())?;

        if tiresp.task_waypoints.is_empty() {
            // task cannot be completed (e.g. inside a no-fly zone)
            let err = mk_error(format!(
                "Task [{}] option [{}] assigned to vehicle [{}] \
                 reported an empty waypoint list for implementation!",
                tiresp.task_id, tiresp.option_id, tiresp.vehicle_id
            ));
            // legacy: still try to complete the request, just skipping this task
            let mut msgs = self.check_next_task_implementation_request(unique_request_id);
            msgs.push(err);
            return Ok(msgs);
        }

        // cache response (waypoints in self.in_progress_response)
        let mut ipr = self.in_progress_response.remove(&unique_request_id).ok_or(())?;
        let mut found_mish = false;

        let next_wp = tiresp.task_waypoints.first().ok_or(())?.number();

        for mut mish in ipr.original_response.mission_command_list_mut() {
            if mish.vehicle_id() == tiresp.vehicle_id {
                found_mish = true;
                if let Some(back) = mish.waypoint_list_mut().last_mut() {
                    *back.next_waypoint_mut() = next_wp;
                }
                for wp in &tiresp.task_waypoints {
                    mish.waypoint_list_mut().push(wp.clone());
                }
                break;
            }
        }

        if !found_mish {
            let mish = MissionCommand {
                command_id: self.next_command_id(),
                vehicle_id: tiresp.vehicle_id,
                first_waypoint: next_wp,
                waypoint_list: tiresp.task_waypoints.clone(),
                ..Default::default()
            };
            ipr.original_response.mission_command_list_mut().push(Box::new(mish));
        }

        self.in_progress_response.insert(unique_request_id, ipr);

        // update projected state
        if let Some(mut pes) = self.projected_entity_states.remove(&unique_request_id) {
            for st in &mut pes {
                if st.state.entity_id == tiresp.vehicle_id {
                    st.final_waypoint_id = tiresp.task_waypoints.last().ok_or(())?.number();
                    st.time = tiresp.final_time;
                    st.state.planning_position = tiresp.final_location.clone();
                    st.state.planning_heading = tiresp.final_heading;
                    break;
                }
            }
            self.projected_entity_states.insert(unique_request_id, pes);
        }

        Ok(self.check_next_task_implementation_request(unique_request_id))
    }

    fn check_next_task_implementation_request(&mut self, unique_request_id: i64) -> Vec<LmcpType> {
        debug_println!("entering check_next_task_implementation_request");
        // check to see if there are any more in the queue
        //    yes --> send_next_task_implementation_request
        //    no --> send self.in_progress_response[unique_request_id], then clear it out
        if let Some(ra) = self.remaining_assignments.get(&unique_request_id) {
            if ra.is_empty() {
                let mut msgs = Vec::new();
                // add FinalStates (which are the 'projected' states in the planning process)
                if let Some(pes) = self.projected_entity_states.get(&unique_request_id) {
                    if let Some(mut ipr) = self.in_progress_response.remove(&unique_request_id) {
                        for e in pes {
                            ipr.final_states.push(Box::new(e.state.clone()));
                        }
                        msgs.push(LmcpType::UniqueAutomationResponse(ipr));

                        let kv = KeyValuePair {
                            key: format!(
                                "UniqueAutomationResponse[{}] - sent\0", unique_request_id
                            ).into_bytes(),
                            value: String::from("\0").into_bytes(),
                        };
                        let ss = ServiceStatus {
                            status_type: ServiceStatusType::Information,
                            info: vec![Box::new(kv)],
                            ..Default::default()
                        };
                        msgs.push(LmcpType::ServiceStatus(ss));
                    }
                }
                return msgs;
            }
        }
        // control flow awkward for the sake of the borrow checker: consider this the `else` branch
        // of `if ra.is_empty()`:
        self.send_next_task_implementation_request(unique_request_id).unwrap_or(Vec::new())
    }
}

impl PlanBuilder {
    fn next_implementation_id(&mut self) -> i64 {
        match self.task_implementation_id.checked_add(1) {
            None => panic!("next_implementation_id overflowed!"),
            Some(x) => {
                self.task_implementation_id = x;
                x
            }
        }
    }

    fn next_command_id(&mut self) -> i64 {
        match self.command_id.checked_add(1) {
            None => panic!("next_command_id overflowed!"),
            Some(x) => {
                self.command_id = x;
                x
            }
        }
    }
}

impl Client {
    fn send_error(&self, msg: String) {
        let ss = mk_error(msg);
        debug_println!("sending error {:?}", ss);
        self.broadcast(&ss).unwrap();
    }
}

fn mk_error(mut msg: String) -> LmcpType {
    msg.push('\0');
    let kv = KeyValuePair {
        key: String::from("No UniqueAutomationResponse\0").into_bytes(),
        value: msg.into_bytes(),
    };
    let ss = ServiceStatus {
        status_type: ServiceStatusType::Error,
        info: vec![Box::new(kv)],
        percent_complete: 0.0,
    };
    LmcpType::ServiceStatus(ss)
}
