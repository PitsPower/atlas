//! Component used in the ALU.

use itertools::Itertools;

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

pub fn get_conditional_inverter_circuit(size: usize) -> Circuit {
	let mut circuit = Circuit::new();

	let input: Vec<_> = get_pin_coords(0.0, size, 50.0).iter()
		.map(|y| add!(circuit, Pin, (-1500.0, *y)))
		.collect();

	let enable = add!(circuit, Pin, (0.0, 2500.0));

	let xors: Vec<_> = get_pin_coords(0.0, size, 250.0).iter()
		.map(|y| add!(circuit, XorGate, (0.0, *y)))
		.collect();

	let junctions: Vec<_> = xors[1..].iter()
		.map(|c| {
			let pos = (
				circuit.components[*c].position.0 + circuit.components[*c].get_pin_positions()[1].0,
				circuit.components[*c].position.1 + circuit.components[*c].get_pin_positions()[1].1,
			);
			add!(circuit, Junction, (pos.0 - 150.0, pos.1), 3)
		})
		.collect();

	let output: Vec<_> = get_pin_coords(0.0, size, 50.0).iter()
		.map(|y| add!(circuit, Pin, (1500.0, *y)))
		.collect();

	circuit.connect_groups(
		&input[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&input[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	for i in 0..size-1 {
		circuit.connect((junctions[i], 2), (xors[i+1], 1), &[]);
	}
	for i in 0..size-2 {
		circuit.connect((junctions[i], 1), (junctions[i+1], 0), &[]);
	}

	circuit.connect((enable, 0), (junctions[size-2], 1), &[
		WireLayoutCommand::CenterVertical,
		WireLayoutCommand::AlignVertical,
	]);
	circuit.connect((junctions[0], 0), (xors[0], 1), &[
		WireLayoutCommand::AlignHorizontal,
	]);
	
	circuit.connect_groups(
		&output[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[..8].iter().map(|c| (*c, 2)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&output[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[8..].iter().map(|c| (*c, 2)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit
}
