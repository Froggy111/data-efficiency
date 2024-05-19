use serde_json;
use std::collections::VecDeque;

#[derive(Default, Copy, Clone)]
pub struct Config {
	pub gravity_multiplier: f64,
	pub env_size: f64,
	pub max_past_states: usize,
}

#[derive(Default, Clone)]
pub struct Object {
	pub coords: Vec<f64>,
	pub orientation: Vec<f64>,
	pub object_type: String,
	pub parameters: std::collections::HashMap<String, f64>,
}

pub fn parse_json (json_str: String)
	-> (Config, VecDeque<Object>) {
	let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

	let mut config: Config = Config::default();
	let mut objects: VecDeque<Object> = VecDeque::new();

	let object_configs = json.get("objects").unwrap();
	for (_k, v) in object_configs.as_object().unwrap() {
		let mut object = Object::default();

		object.coords = v
			.get("coords").unwrap()
			.as_array().unwrap()
			.iter()
			.map(|x| x.as_f64().unwrap())
			.collect();

		object.orientation = v
			.get("orientation").unwrap()
			.as_array().unwrap()
			.iter()
			.map(|x| x.as_f64().unwrap())
			.collect();

		object.object_type = v
			.get("object_type").unwrap()
			.as_str().unwrap()
			.to_string();

		let params = v
			.get("parameters").unwrap()
			.as_object().unwrap()
			.clone();

		for (param_k, param_v) in params {
			object.parameters.insert(param_k, param_v.as_f64().unwrap());
		}

		objects.push_back(object);
	}
	
	(config, objects)
}