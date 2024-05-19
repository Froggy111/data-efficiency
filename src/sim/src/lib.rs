mod sim_builder;
mod rendering;
mod agent_feedback;
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::VecDeque;

#[pymodule]
fn sim(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<Sim>()?;
	Ok(())
}

#[pyclass]
struct Sim {
	physics_engine: sim_builder::build_sim::SimEngine,
	pub past_states: VecDeque<sim_builder::build_sim::SimState>,
	pub config: sim_builder::parse_json::Config,
	pub objects: VecDeque<sim_builder::parse_json::Object>,
}

#[pymethods]
impl Sim {
	#[new]
	pub fn new(json_str: Option<&Bound<'_, PyString>>) -> Self {
		let (config, objects) = sim_builder::parse_json::parse_json(json_str.unwrap().to_str().unwrap().to_string());
		let physics_engine = sim_builder::build_sim::build_sim(&config, &objects);
		Sim {
			physics_engine,
			past_states: VecDeque::new(),
			config,
			objects,
		}
	}

	pub fn step(&mut self) {
		let new_simstate = self.physics_engine.step();
		self.past_states.push_back(new_simstate);
		if self.past_states.len() > self.config.max_past_states {
			self.past_states.pop_front();
		}
	}

	pub fn get_past_states(&self) -> Vec<sim_builder::build_sim::SimState> {
		self.past_states.get()
	}
}