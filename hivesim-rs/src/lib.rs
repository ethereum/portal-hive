#![allow(dead_code)]
#![warn(clippy::unwrap_used)]
mod macros;
mod simulation;
mod testapi;
mod testmatch;
pub mod types;
pub mod utils;

pub use simulation::Simulation;
pub use testapi::{
    Client, ClientTestSpec, NClientTestSpec, Suite, Test, TestSpec, TwoClientTestSpec,
};
pub use testmatch::TestMatcher;
