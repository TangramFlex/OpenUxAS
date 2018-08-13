use lmcp::afrl::cmasi::line_search_task::LineSearchTask;

pub struct LineSearch {
    plan_both_directions: bool,
}

/// called after the lifetime thread receives a CMASI LineSearchTask
/// message; represents the lifecycle of an individual task instance
pub async fn run_task(task_msg: LineSearchTask, mut task: LineSearch) {
    task.configure(&task_msg);
}

impl LineSearch {
    fn configure(&mut self, task_msg: &LineSearchTask) {
        // aspirational assignment; currently this option is set in
        // the XML configuration of the `TaskServiceManager`, but it
        // really should be part of the messages that spin up tasks

        // self.plan_both_directions = task_msg.plan_both_directions
    }
}
