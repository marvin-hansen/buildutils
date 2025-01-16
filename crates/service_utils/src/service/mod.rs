use std::collections::HashMap;

mod build;
mod dbg;
mod display;
mod start;
mod wait;
mod verify;
mod stop;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ServiceUtil {
    dbg: bool,
    root_path: &'static str,
    binaries: Vec<&'static str>,
    bin_pid_map: HashMap<String, String>,
}
