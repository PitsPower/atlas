//! Adder components.

use crate::add;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;

pub fn get_half_adder_circuit() -> Circuit {
	let mut circuit = Circuit::new();
		
	let input1 = add!(circuit, Pin, (-250.0, -150.0));
	let input2 = add!(circuit, Pin, (-250.0, 150.0));

	let junc1 = add!(circuit, Junction, (-200.0, -150.0), 3);
	let junc2 = add!(circuit, Junction, (-150.0, 150.0), 3);

	let and = add!(circuit, AndGate, (0.0, -100.0));
	let xor = add!(circuit, XorGate, (0.0, 100.0));
	
	let output1 = add!(circuit, Pin, (250.0, -150.0));
	let output2 = add!(circuit, Pin, (250.0, 150.0));

	circuit.connect((input1, 0), (junc1, 0), vec![]);
	circuit.connect((junc1, 1), (and, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((junc1, 2), (xor, 0), vec![
		WireLayoutCommand::AlignHorizontal,
	]);
	
	circuit.connect((input2, 0), (junc2, 0), vec![]);
	circuit.connect((junc2, 1), (and, 1), vec![
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((junc2, 2), (xor, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((and, 2), (output1, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((xor, 2), (output2, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit
}

pub fn get_full_adder_circuit() -> Circuit {
	let mut circuit = Circuit::new();
		
	let half_adder_1 = add!(circuit, HalfAdder, (-200.0, 100.0));
	let half_adder_2 = add!(circuit, HalfAdder, (200.0, 100.0));
	let or_gate = add!(circuit, OrGate, (0.0, -125.0));

	let input_offset = circuit.components[half_adder_1].position.1 +
		circuit.components[half_adder_1].get_pin_positions()[1].1;

	let input1 = add!(circuit, Pin, (-500.0, -input_offset));
	let input2 = add!(circuit, Pin, (-500.0, input_offset));
	let carry_in = add!(circuit, Pin, (0.0, 250.0));

	let output = add!(circuit, Pin, (500.0, 0.0));
	let carry_out = add!(circuit, Pin, (0.0, -250.0));

	circuit.connect((input1, 0), (half_adder_1, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (half_adder_1, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((half_adder_1, 3), (half_adder_2, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((carry_in, 0), (half_adder_2, 1), vec![
		WireLayoutCommand::MoveVertical(-50.0),
		WireLayoutCommand::MoveHorizontal(50.0),
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((half_adder_2, 3), (output, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((half_adder_1, 2), (or_gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((half_adder_2, 2), (or_gate, 0), vec![
		WireLayoutCommand::MoveHorizontal(50.0),
		WireLayoutCommand::MoveVertical(-80.0),
		WireLayoutCommand::MoveHorizontal(-500.0),
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((or_gate, 2), (carry_out, 0), vec![
		WireLayoutCommand::MoveHorizontal(50.0),
		WireLayoutCommand::MoveVertical(-80.0),
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}

pub fn get_adder_circuit(size: usize) -> Circuit {
	let mut circuit = Circuit::new();

	let chip_width = 400.0;
	let scale = 0.3;

	let adders: Vec<_> = (0..size)
		.map(|i| add!(circuit, FullAdder, (100.0, -(i as f64 - (size as f64) * 0.5) * 300.0 - 150.0)))
		.collect();

	let input_group_1: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (-chip_width * 0.5 / scale, -600.0 - (i as f64 * 50.0))))
		.collect();

	let input_group_2: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (-chip_width * 0.5 / scale, 600.0 + size as f64 * 50.0 - (i as f64 * 50.0))))
		.collect();
		
	let output_group: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (chip_width * 0.5 / scale, -(i as f64 - (size as f64) * 0.5) * 50.0 - 25.0)))
		.collect();

	for i in 0..size {
		circuit.connect((input_group_1[i as usize], 0), (adders[i as usize], 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::MoveHorizontal(i as f64 * 30.0),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input_group_2[i as usize], 0), (adders[i as usize], 1), vec![
			WireLayoutCommand::MoveHorizontal(10.0),
			WireLayoutCommand::MoveHorizontal((size as f64 - i as f64) * 30.0),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((adders[i as usize], 3), (output_group[i as usize], 0), vec![
			WireLayoutCommand::MoveHorizontal(30.0),
			WireLayoutCommand::MoveHorizontal(((i as f64 - size as f64 * 0.5) * 30.0 + 15.0).abs()),
			WireLayoutCommand::AlignHorizontal,
		]);
	}

	for i in 0..size-1 {
		circuit.connect((adders[i as usize], 4), (adders[(i+1) as usize], 2), vec![]);
	}

	circuit
}
