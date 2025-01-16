mod build;
mod dbg;
mod display;
mod start;
mod wait;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ServiceUtil {
    dbg: bool,
    root_path: &'static str,
    binaries: Vec<&'static str>,
}
