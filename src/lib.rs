#![feature(test)]
#[allow(dead_code)]

extern crate test;

mod core;
mod gates;
mod graphics;
mod transistor;
mod utils;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use crate::core::{Bulb, RectangleChip, ChipInternals, Circuit, Junction, Switch};
use crate::gates::{AndGate, NandGate, NorGate, NotGate, OrGate, XorGate};
use crate::graphics::WireLayoutCommand;
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
			let chip1 = RectangleChip {
				internals: ChipInternals {
					circuit: iter(n-1),
					inner_scale: 0.4,
				},
		
				position: (-180.0, 0.0),
				size: (300.0, 300.0),
			};
			let chip2 = RectangleChip {
				internals: ChipInternals {
					circuit: iter(n-1),
					inner_scale: 0.4,
				},
		
				position: (180.0, 0.0),
				size: (300.0, 300.0),
			};

			result.add(Box::new(chip1));
			result.add(Box::new(chip2));
		}

		result
	}

	let chip = RectangleChip {
		internals: ChipInternals {
			circuit: iter(n),
			inner_scale: 1.0,
		},

		position: (0.0, 10.0),
		size: (850.0, 500.0),
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
	
	let input = add!(circuit, Switch, (-300.0, 0.0));
	let not_gate_1 = add!(circuit, NotGate, (0.0, -150.0));
	let not_gate_2 = add!(circuit, NotGate, (0.0, 150.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input, 0), (not_gate_1, 0), vec![
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((not_gate_1, 1), (not_gate_2, 0), vec![
		WireLayoutCommand::MoveHorizontal(100.0),
		WireLayoutCommand::CenterVertical,
		WireLayoutCommand::MoveHorizontal(-300.0),
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((not_gate_2, 1), (output, 0), vec![
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}

#[wasm_bindgen]
pub fn nor_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, NorGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn or_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, OrGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn nand_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, NandGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn and_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, AndGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn xor_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, XorGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn nor_latch_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	
	let nor1 = add!(circuit, NorGate, (0.0, -100.0));
	let nor2 = add!(circuit, NorGate, (0.0, 100.0));

	let junction1 = add!(circuit, Junction, (150.0, -100.0), 3);
	let junction2 = add!(circuit, Junction, (150.0, 100.0), 3);

	let output1 = add!(circuit, Bulb, (300.0, -100.0));
	let output2 = add!(circuit, Bulb, (300.0, 100.0));

	circuit.connect((input1, 0), (nor1, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (nor2, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	
	circuit.connect((nor1, 2), (junction1, 0), vec![]);
	circuit.connect((junction1, 1), (output1, 0), vec![]);
	
	circuit.connect((nor2, 2), (junction2, 0), vec![]);
	circuit.connect((junction2, 1), (output2, 0), vec![]);

	circuit.connect((junction1, 2), (nor2, 0), vec![
		WireLayoutCommand::MoveVertical(35.0),
		WireLayoutCommand::Move((-250.0, 100.0)),
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((junction2, 2), (nor1, 1), vec![
		WireLayoutCommand::MoveVertical(-35.0),
		WireLayoutCommand::Move((-250.0, -100.0)),
		WireLayoutCommand::AlignHorizontal,
	]);

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

	#[test]
	fn test() {
		let mut circuit = bidirectional_example();
		circuit.toggle_switch(0);
		circuit.toggle_switch(1);
		circuit.toggle_switch(0);
	}

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
