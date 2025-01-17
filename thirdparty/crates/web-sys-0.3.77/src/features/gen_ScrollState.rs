#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ScrollState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScrollState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollState {
    Started = "started",
    Stopped = "stopped",
}
