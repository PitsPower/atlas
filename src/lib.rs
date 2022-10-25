#![feature(test)]
#[allow(dead_code)]

extern crate test;

mod core;
mod graphics;
mod transistor;
mod utils;

use graphics::WireLayoutCommand;
use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use crate::core::{Bulb, Chip, Circuit, Junction, Switch};
// use crate::graphics::WireLayoutCommand;
use crate::transistor::{NTransistor, PTransistor};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		web_sys::console::log_1(&format!($($arg)*).into());
	};
}

#[macro_export]
macro_rules! add {
	($circ:expr, $type:ident, $($arg:tt)*) => {
		$circ.add(Box::new($type::new($($arg)*)))
	};
}

#[wasm_bindgen]
pub fn example1(n: u32) -> Circuit {
	fn iter(n: u32) -> Circuit {
		let mut result = Circuit::new();
		
		if n > 0 {
			let chip1 = Chip {
				circuit: iter(n-1),
		
				position: (-180.0, 0.0),
				size: (300.0, 300.0),
				inner_scale: 0.4,
			};
			let chip2 = Chip {
				circuit: iter(n-1),
		
				position: (180.0, 0.0),
				size: (300.0, 300.0),
				inner_scale: 0.4,
			};

			result.add(Box::new(chip1));
			result.add(Box::new(chip2));
		}

		result
	}

	let chip = Chip {
		circuit: iter(n),

		position: (0.0, 10.0),
		size: (850.0, 500.0),
		inner_scale: 1.0,
	};

	let mut circuit = Circuit::new();
	circuit.add(Box::new(chip));
	circuit
}

#[wasm_bindgen]
pub fn example2() -> Circuit {
	let mut circuit = Circuit::new();

	let switch = circuit.add(Box::new(Switch::new((-200.0, 0.0))));
	let junction = circuit.add(Box::new(Junction::new((0.0, 0.0), 3)));
	let bulb1 = circuit.add(Box::new(Bulb::new((200.0, 0.0))));
	let bulb2 = circuit.add(Box::new(Bulb::new((0.0, -200.0))));

	circuit.connect((switch, 0), (junction, 0), vec![]);
	circuit.connect((junction, 1), (bulb1, 0), vec![]);
	circuit.connect((junction, 2), (bulb2, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn transistor_example() -> Circuit {
	let mut circuit = Circuit::new();

	let transistor = circuit.add(Box::new(NTransistor::new((0.0, 0.0))));
	
	let gate = circuit.add(Box::new(Switch::new((-200.0, 0.0))));

	let offset = circuit.get_components()[transistor].get_pin_positions()[1].0;

	let source = circuit.add(Box::new(Switch::new((offset, 200.0))));
	let drain = circuit.add(Box::new(Bulb::new((offset, -200.0))));

	circuit.connect((gate, 0), (transistor, 0), vec![]);
	circuit.connect((source, 0), (transistor, 1), vec![]);
	circuit.connect((transistor, 2), (drain, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn transistor_example2() -> Circuit {
	let mut circuit = Circuit::new();

	let transistor = circuit.add(Box::new(PTransistor::new((0.0, 0.0))));
	
	let gate = circuit.add(Box::new(Switch::new((-200.0, 0.0))));

	let offset = circuit.get_components()[transistor].get_pin_positions()[1].0;

	let source = circuit.add(Box::new(Switch::new((offset, -200.0))));
	let drain = circuit.add(Box::new(Bulb::new((offset, 200.0))));

	circuit.connect((gate, 0), (transistor, 0), vec![]);
	circuit.connect((source, 0), (transistor, 1), vec![]);
	circuit.connect((transistor, 2), (drain, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn bidirectional_example() -> Circuit {
	let mut circuit = Circuit::new();

	let transistor1 = add!(circuit, NTransistor, (-400.0, 200.0));
	let transistor2 = add!(circuit, NTransistor, (400.0, 200.0));

	let switch1 = add!(circuit, Switch, (-600.0, 200.0));
	let switch2 = add!(circuit, Switch, (200.0, 200.0));

	let offset = circuit.get_components()[transistor1].get_pin_positions()[1].0;

	let source1 = add!(circuit, Switch, (-400.0 + offset, 400.0));
	let source2 = add!(circuit, Switch, (400.0 + offset, 400.0));

	circuit.toggle_switch(2);
	circuit.toggle_switch(3);

	let junction1 = add!(circuit, Junction, (-400.0 + offset, 0.0), 3);
	let junction2 = add!(circuit, Junction, (400.0 + offset, 0.0), 3);

	let bulb1 = add!(circuit, Bulb, (-400.0 + offset, -200.0));
	let bulb2 = add!(circuit, Bulb, (400.0 + offset, -200.0));

	circuit.connect((switch1, 0), (transistor1, 0), vec![]);
	circuit.connect((source1, 0), (transistor1, 1), vec![]);
	circuit.connect((transistor1, 2), (junction1, 0), vec![]);
	circuit.connect((junction1, 1), (bulb1, 0), vec![]);

	circuit.connect((switch2, 0), (transistor2, 0), vec![]);
	circuit.connect((source2, 0), (transistor2, 1), vec![]);
	circuit.connect((transistor2, 2), (junction2, 0), vec![]);
	circuit.connect((junction2, 1), (bulb2, 0), vec![]);
	
	circuit.connect((junction1, 2), (junction2, 2), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn not_gate_example() -> Circuit {
	let mut circuit = Circuit::new();
	
	let input = add!(circuit, Switch, (-400.0, 0.0));
	let input_junc = add!(circuit, Junction, (-230.0, 0.0), 3);

	let n_transistor = add!(circuit, NTransistor, (0.0, 200.0));
	let p_transistor = add!(circuit, PTransistor, (0.0, -200.0));

	let offset = circuit.get_components()[n_transistor].get_pin_positions()[1].0;

	let on_source = add!(circuit, Switch, (offset, -400.0));
	let off_source = add!(circuit, Switch, (offset, 400.0));

	circuit.toggle_switch(1);
	
	let output_junc = add!(circuit, Junction, (230.0, 0.0), 3);
	let output = add!(circuit, Bulb, (400.0, 0.0));

	circuit.connect((input, 0), (input_junc, 0), vec![]);
	circuit.connect((input_junc, 1), (n_transistor, 0), vec![WireLayoutCommand::AlignHorizontal]);
	circuit.connect((input_junc, 2), (p_transistor, 0), vec![WireLayoutCommand::AlignHorizontal]);

	circuit.connect((on_source, 0), (p_transistor, 1), vec![]);
	circuit.connect((off_source, 0), (n_transistor, 1), vec![]);

	circuit.connect((n_transistor, 2), (output_junc, 1), vec![WireLayoutCommand::AlignVertical]);
	circuit.connect((p_transistor, 2), (output_junc, 2), vec![WireLayoutCommand::AlignVertical]);
	
	circuit.connect((output_junc, 0), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen(start)]
pub fn start() {
	log!("Stuff has started!");
	set_panic_hook();
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[bench]
	fn bench_simple_switch_circuit(b: &mut Bencher) {
		let mut circuit = example2();
		b.iter(|| circuit.toggle_switch(0));
	}

	#[bench]
	fn bench_not_gate(b: &mut Bencher) {
		let mut circuit = not_gate_example();
		b.iter(|| circuit.toggle_switch(0));
	}
}
