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
    root_path: &'static str, // root_path basically remains constant after initialization
    binaries: Vec<&'static str>, // After verification, we're only reading from the Vec, thus lock-free
    binary_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}
