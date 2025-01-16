use std::collections::HashMap;
use std::sync::RwLock;
use tokio::task::JoinHandle;

mod build;
mod dbg;
mod display;
mod getters;
mod start;
mod stop;
mod verify;
mod wait;

#[derive(Debug)]
pub struct ServiceUtil {
    dbg: bool,
    root_path: &'static str,
    binaries: Vec<&'static str>,
    binary_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}
