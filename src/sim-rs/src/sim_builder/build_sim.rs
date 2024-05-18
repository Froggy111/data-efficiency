use crate::sim_builder::build_object::build_object;
use crate::sim_builder::parse_json::parse_json;
use pyo3::prelude::*;
use pyo3::types::*;

use super::parse_json;

pub fn build_sim (configs: &parse_json::Config, objects: &Vec<parse_json::Object>) -> SimEngine {
	for object in objects.iter() {
		build_object();
	}
}

pub fn build_engine (config: &parse_json::Config) -> () {

}

#[derive(Default)]
pub struct SimEngine {
	
}

impl SimEngine {
	pub fn step (&mut self) -> SimState {
		SimState {}
	}
}

#[derive(Default, Clone)]
pub struct SimState {

}

impl SimState {
	pub fn rs_to_py (&self) -> () {

	}
}