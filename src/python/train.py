from sim_wrapper import new_sim
from tokenizer import Tokenizer
from model import Model
import jax
from jax import numpy as jnp
import numpy as np

def train_step (model, sims, sim_steps_per_step, input_modalities, output_modalities, tokenizer, max_in_length, max_out_length):
	all_in_toks = jnp.empty((0, max_in_length), jnp.int32)
	all_out_toks = jnp.empty((0, max_out_length), jnp.int32)
	for _ in range(sim_steps_per_step):
		for simobj in sims:
			in_data = simobj.render(input_modalities)
			simobj.step()
			out_data = simobj.render(output_modalities)
			in_toks = tokenizer(in_data, input_modalities, max_in_length)
			out_toks = tokenizer(out_data, output_modalities, max_out_length)
			all_in_toks = jnp.concatenate((all_in_toks, in_toks))
			all_out_toks = jnp.concatenate((all_out_toks, out_toks))
	loss = model.train(all_in_toks, all_out_toks)
	return loss

def train(sim_steps_per_step, steps_per_sim, total_steps, n_sims, input_modalities, output_modalities, max_in_length, max_out_length):
	model = Model(sim_steps_per_step, input_modalities, output_modalities, max_in_length, max_out_length)
	tokenizer = Tokenizer()
	current_sim_n_steps = 0
	for i in range(total_steps):
		if not sims:
			sims = [new_sim() for _ in range(n_sims)]
		loss = train_step(model, sims, sim_steps_per_step, input_modalities, output_modalities, tokenizer, max_in_length, max_out_length)
		current_sim_n_steps += 1
		if current_sim_n_steps >= steps_per_sim:
			current_sim_n_steps = 0
			sims = False
		print(f"Step {i} loss: {loss}")