mod sim_builder;
mod rendering;
mod agent_feedback;
use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
fn sim(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<Game>()?;
	Ok(())
}

#[pyclass]
struct Sim {
	physics_engine: sim_builder::build_sim::SimEngine,
	past_states: Vec<sim_builder::build_sim::SimState>,
	config: sim_builder::parse_json::Config,
	objects: Vec<sim_builder::parse_json::Object>,
}

#[pymethods]
impl Sim {
	#[new]
	pub fn new(json_str: Option<&Bound<'_, PyString>>) -> Self {
		let (config, objects) = sim_builder::parse_json::parse_json(json_str.unwrap().to_str().unwrap().to_string());
		let physics_engine = sim_builder::build_sim::build_sim(&config, &objects);
		Sim {
			physics_engine,
			past_states: Vec::new(),
			config,
			objects,
		}
	}
}

impl Sim {
	pub fn rs_step(&mut self) -> sim_builder::build_sim::SimState {
		let new_simstate = self.physics_engine.step();
		self.past_states.push(new_simstate.clone());
		new_simstate
	}

	pub fn 
}

#[pyfunction]
fn print(string: Option<&Bound<'_, PyString>>) -> i32 {
	println!("{}", string.unwrap().to_str().unwrap());
	42
}
