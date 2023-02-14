//! Component used in the ALU.

use crate::add;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::utils::get_pin_coords;

pub fn get_zero_tester_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let input: Vec<_> = get_pin_coords(0.0, 16, 300.0).iter()
		.map(|y| add!(circuit, Pin, (-500.0, *y)))
		.collect();

	circuit
}
