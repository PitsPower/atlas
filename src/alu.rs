//! Component used in the ALU.

use crate::add;
use crate::bus::BusLayoutCommand;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;
use crate::utils::get_pin_coords;

pub fn get_zero_tester_circuit() -> Circuit {
	let width = 3000.0;
	let layer_xs = get_pin_coords(0.0, 4, 500.0);

	let mut circuit = Circuit::new();

	let input: Vec<_> = get_pin_coords(0.0, 16, 50.0).iter()
		.map(|y| add!(circuit, Pin, (-width * 0.5, *y)))
		.collect();

	let output = add!(circuit, Pin, (width * 0.5, 0.0));

	let layer1: Vec<_> = get_pin_coords(0.0, 8, 300.0).iter()
		.map(|y| add!(circuit, NorGate, (layer_xs[0], *y)))
		.collect();

	let layer2: Vec<_> = get_pin_coords(0.0, 4, 300.0).iter()
		.map(|y| add!(circuit, NandGate, (layer_xs[1], *y)))
		.collect();

	let layer3: Vec<_> = get_pin_coords(0.0, 2, 300.0).iter()
		.map(|y| add!(circuit, NorGate, (layer_xs[2], *y)))
		.collect();

	let layer4: Vec<_> = get_pin_coords(0.0, 1, 300.0).iter()
		.map(|y| add!(circuit, AndGate, (layer_xs[3], *y)))
		.collect();
	
	circuit.connect_groups(
		&input[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&layer1[..4].iter().flat_map(|c| [(*c, 0), (*c, 1)]).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&input[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&layer1[4..].iter().flat_map(|c| [(*c, 0), (*c, 1)]).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	let wire_coords = get_pin_coords((layer_xs[0] + layer_xs[1]) * 0.5, 4, 40.0);

	for i in 0..4 {
		circuit.connect((layer1[i], 2), (layer2[i/2], i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[3-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((layer1[7-i], 2), (layer2[3-i/2], 1 - i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[3-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
	}
	
	let wire_coords = get_pin_coords((layer_xs[1] + layer_xs[2]) * 0.5, 2, 40.0);

	for i in 0..2 {
		circuit.connect((layer2[i], 2), (layer3[i/2], i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[1-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((layer2[3-i], 2), (layer3[1-i/2], 1 - i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[1-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
	}

	circuit.connect((layer3[0], 2), (layer4[0], 0), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((layer3[1], 2), (layer4[0], 1), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((layer4[0], 2), (output, 0), &[]);

	circuit
}
