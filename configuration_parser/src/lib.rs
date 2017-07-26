extern crate xml;

use std::io::Read;
use std::path;
use std::str::{FromStr};
use std::time;

use xml::attribute::{OwnedAttribute};
use xml::name::{OwnedName};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub struct UxAS {
    pub entity_start_time_since_epoch: time::Duration,
    pub entity_id: u32,
    pub entity_type: String,
    pub console_logger_severity_level: LogSeverityLevel,
    pub main_file_logger_severity_level: LogSeverityLevel,
    pub is_logging_thread_id: bool,
    pub is_zeromq_multipart_message: bool,
    pub run_duration: time::Duration,
    pub serial_port_wait_time: time::Duration,
    pub start_delay: time::Duration,
    pub zeromq_receive_socket_poll_wait_time: time::Duration,
    pub root_data_work_directory: path::PathBuf,
    pub root_data_in_directory: path::PathBuf,
    pub root_data_ref_directory: path::PathBuf,
}

impl Default for UxAS {
    fn default() -> UxAS {
        UxAS {
            entity_start_time_since_epoch: time::Duration::from_millis(0),
            entity_id: 0,
            entity_type: String::from(""),
            console_logger_severity_level: LogSeverityLevel::DEBUG,
            main_file_logger_severity_level: LogSeverityLevel::DEBUG,
            is_logging_thread_id: true,
            is_zeromq_multipart_message: false,
            run_duration: time::Duration::from_secs(u32::max_value() as u64),
            serial_port_wait_time: time::Duration::from_millis(50),
            start_delay: time::Duration::from_millis(0),
            zeromq_receive_socket_poll_wait_time: time::Duration::from_millis(100),
            root_data_work_directory: path::PathBuf::from("datain/"),
            root_data_in_directory: path::PathBuf::from("dataref/"),
            root_data_ref_directory: path::PathBuf::from("datawork/"),
        }
    }
}

impl UxAS {
    pub fn from_xml<R: Read>(source: R) -> UxAS {
        let mut out = UxAS::default();

        let now = time::SystemTime::now();
        let since_epoch = now.duration_since(time::UNIX_EPOCH)
                             .expect("Creating a new UxAS before the UNIX epoch");
        out.entity_start_time_since_epoch = since_epoch;

        let parser = EventReader::new(source);
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name: OwnedName { local_name, .. },
                    attributes, ..
                }) => {
                    if local_name == "UxAS" {
                        println!("found UxAS node!");
                        for OwnedAttribute {
                                name: OwnedName { local_name: attr_name, .. },
                                value
                            }
                        in attributes {
                            match attr_name.as_str() {
                                "EntityID" => {
                                    let id = u32::from_str(value.as_str()).expect("Valid uint32 literal");
                                    out.entity_id = id;
                                },
                                "EntityType" => { out.entity_type = value },
                                "ConsoleLoggerSeverityLevel" => {
                                    let l = LogSeverityLevel::from_str(value.as_str()).expect("Valid logger severity level");
                                    out.console_logger_severity_level = l;
                                },
                                "MainFileLoggerSeverityLevel" => {
                                    let l = LogSeverityLevel::from_str(value.as_str()).expect("Valid logger severity level");
                                    out.main_file_logger_severity_level = l;
                                },
                                "StartDelay_ms" => {
                                    let ms = u64::from_str(value.as_str()).expect("Valid uint32 literal");
                                    out.start_delay = time::Duration::from_millis(ms);
                                },
                                "RunDuration_s" => {
                                    let s = u64::from_str(value.as_str()).expect("Valid uint32 literal");
                                    out.start_delay = time::Duration::from_secs(s);
                                },
                                "isLoggingThreadId" => {
                                    let b = bool::from_str(value.as_str()).expect("`true` or `false`");
                                    out.is_logging_thread_id = b;
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        out
    }
}

#[derive(Debug, PartialEq)]
pub enum LogSeverityLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl Default for LogSeverityLevel {
    fn default() -> LogSeverityLevel { LogSeverityLevel::DEBUG }
}

impl FromStr for LogSeverityLevel {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, ()> {
        match src {
            "DEBUG" => Ok(LogSeverityLevel::DEBUG),
            "INFO" => Ok(LogSeverityLevel::INFO),
            "WARN" => Ok(LogSeverityLevel::WARN),
            "ERROR" => Ok(LogSeverityLevel::ERROR),
            _ => Err(())
        }
    }
}
