use super::build_object::build_object;
use super::parse_json;
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::VecDeque;


pub fn build_sim (configs: &parse_json::Config, objects: &VecDeque<parse_json::Object>) -> SimEngine {
	for object in objects.iter() {
		build_object(&object);
	}

	SimEngine::default()
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

#[derive(Default)]
pub struct SimState {

}

impl SimState {
	pub fn rs_to_py (&self) -> () {

	}
}