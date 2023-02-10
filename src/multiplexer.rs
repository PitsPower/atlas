//! Multiplexer components.

use crate::add;
use crate::bus::*;
use crate::core::{Circuit, ComponentOptions, ComponentSimulator, ComponentType, PinError, PinState};
use crate::graphics::WireLayoutCommand;
use crate::utils::get_pin_coords;

pub fn get_multiplexer_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, AndGate, (0.000, -100.000));
	let c1 = add!(circuit, AndGate, (0.000, 100.000));
	let c2 = add!(circuit, NotGate, (-200.000, 0.000));
	let c3 = add!(circuit, Junction, (-300.000, 100.000), 3);
	let c4 = add!(circuit, Pin, (-400.000, -200.000));
	let c5 = add!(circuit, Pin, (-400.000, 200.000));
	let c8 = add!(circuit, Pin, (0.000, 350.000));
	let c6 = add!(circuit, Pin, (400.000, 0.000));
	let c7 = add!(circuit, OrGate, (200.000, 0.000));
	
	circuit.connect((c2, 1), (c0, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 0), (c3, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 1), (c2, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 0), (c1, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 0), (c0, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 2), (c7, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 2), (c7, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c7, 2), (c6, 0), &[]);
	circuit.connect((c8, 0), (c3, 2), &[WireLayoutCommand::MoveTo((0.000, 231.313)), WireLayoutCommand::AlignVertical]);

	circuit
}

pub fn get_multi_multiplexer_circuit(size: usize) -> Circuit {
	let mut circuit = Circuit::new();

	let chip_width = 4000.0;

	let pin_spacing = 70.0;

	let input_group_1: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (-chip_width * 0.5, -700.0 - (i as f64 * pin_spacing))))
		.rev()
		.collect();

	let input_group_2: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (-chip_width * 0.5, 700.0 + (size - i) as f64 * pin_spacing)))
		.rev()
		.collect();

	let enable = add!(circuit, Pin, (0.0, 220.0 * size as f64));
		
	let junctions: Vec<_> = (1..size)
		.map(|i| add!(circuit, Junction, (300.0, (i as f64 - size as f64 * 0.5 + 0.5) * 400.0 + 200.0), 3))
		.collect();
		
	let multiplexers: Vec<_> = (0..size)
		.map(|i| add!(circuit, Multiplexer, (0.0, (i as f64 - size as f64 * 0.5 + 0.5) * 400.0)))
		.collect();

	let output_group: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (chip_width * 0.5, -(i as f64 - (size as f64) * 0.5) * pin_spacing)))
		.rev()
		.collect();

	let input_1_positions: Vec<_> = input_group_1.iter().map(|i| circuit.components[*i].position).collect();
	let input_2_positions: Vec<_> = input_group_2.iter().map(|i| circuit.components[*i].position).collect();

	let multiplexer_input_1_positions: Vec<_> = multiplexers.iter()
		.map(|i| {
			let pos = circuit.components[*i].position;
			let pin_pos = circuit.components[*i].get_pin_positions()[0];
			(pos.0 + pin_pos.0, pos.1 + pin_pos.1)
		})
		.collect();

	let multiplexer_input_2_positions: Vec<_> = multiplexers.iter()
		.map(|i| {
			let pos = circuit.components[*i].position;
			let pin_pos = circuit.components[*i].get_pin_positions()[1];
			(pos.0 + pin_pos.0, pos.1 + pin_pos.1)
		})
		.collect();

	let multiplexer_output_positions: Vec<_> = multiplexers.iter()
		.map(|i| {
			let pos = circuit.components[*i].position;
			let pin_pos = circuit.components[*i].get_pin_positions()[3];
			(pos.0 + pin_pos.0, pos.1 + pin_pos.1)
		})
		.collect();

	let output_positions: Vec<_> = output_group.iter().map(|i| circuit.components[*i].position).collect();
		
	let input_1_commands = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&input_1_positions[..],
		&multiplexer_input_1_positions[..],
	);
	let input_2_commands = compute_wire_commands(
		&[
			BusLayoutCommand::MoveHorizontal(200.0),
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&input_2_positions[..],
		&multiplexer_input_2_positions[..],
	);

	for idx in 0..size {
		circuit.connect((input_group_1[idx], 0), (multiplexers[idx], 0), &input_1_commands[idx]);
		circuit.connect((input_group_2[idx], 0), (multiplexers[idx], 1), &input_2_commands[idx]);
	}
	
	let output_commands_1 = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&output_positions[..size/2],
		&multiplexer_output_positions[..size/2],
	);
	let output_commands_2 = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&output_positions[size/2..],
		&multiplexer_output_positions[size/2..],
	);

	for idx in 0..size/2 {
		circuit.connect((output_group[idx], 0), (multiplexers[idx], 3), &output_commands_1[idx]);
	}
	for idx in size/2..size {
		circuit.connect((output_group[idx], 0), (multiplexers[idx], 3), &output_commands_2[idx - size/2]);
	}

	for idx in 0..size-2 {
		circuit.connect((junctions[idx], 1), (junctions[idx + 1], 0), &[]);
		circuit.connect((junctions[idx], 2), (multiplexers[idx + 1], 2), &[WireLayoutCommand::AlignVertical]);
	}
	
	circuit.connect((enable, 0), (junctions[size-2], 1), &[
		WireLayoutCommand::CenterVertical,
		WireLayoutCommand::AlignVertical,
	]);
	circuit.connect((junctions[size-2], 2), (multiplexers[size-1], 2), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((junctions[0], 0), (multiplexers[0], 2), &[
		WireLayoutCommand::MoveVertical(-400.0),
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}

pub struct MultiMultiplexerSimulator {
	size: usize,

	input1: Vec<PinState>,
	input2: Vec<PinState>,
	enable: PinState,
}

impl MultiMultiplexerSimulator {
	pub fn new(size: usize) -> Self {
		Self {
			size,
			input1: vec![PinState::Disconnected; size],
			input2: vec![PinState::Disconnected; size],
			enable: PinState::Disconnected,
		}
	}
}

impl ComponentSimulator for MultiMultiplexerSimulator {
	fn set_mode_to_high_level(&mut self, circuit: &Circuit) {
		for i in 0..self.size*2+1 {
			let state = circuit.get_pin(i).unwrap().get_pin_state(0).unwrap();
			self.set_pin_state_high_level(i, state).unwrap();
		}
	}

	fn set_mode_to_circuit(&mut self, circuit: &mut Circuit) {
		for (i, state) in self.input1.iter().enumerate() {
			circuit.set_pin(i, *state);
		}
		for (i, state) in self.input2.iter().enumerate() {
			circuit.set_pin(i + self.size, *state);
		}
		circuit.set_pin(self.size * 2, self.enable);
	}

    fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
        if idx <= self.size * 2 {
			Ok(PinState::Disconnected)
		} else if idx > self.size * 3 {
			Err(PinError::OutOfRange)
		} else if self.enable == PinState::On {
			Ok(self.input2[idx - self.size * 2 - 1].or(PinState::Off))
		} else {
			Ok(self.input1[idx - self.size * 2 - 1].or(PinState::Off))
		}
    }

    fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
        if idx < self.size {
			self.input1[idx] = state;
			Ok(())
		} else if idx < self.size * 2 {
			self.input2[idx - self.size] = state;
			Ok(())
		} else if idx == self.size * 2 {
			self.enable = state;
			Ok(())
		} else if idx <= self.size * 3 {
			Ok(())
		} else {
			Err(PinError::OutOfRange)
		}
    }
}

pub fn get_four_way_circuit(size: usize) -> Circuit {
	let mut circuit = Circuit::new();

	let chip_width = 4000.0;
	let chip_height = 5000.0;
	let group_spacing = 1333.0;
	let pin_spacing = 50.0;

	let group_positions = get_pin_coords(0.0, 4, group_spacing);

	let input_groups: Vec<Vec<_>> = group_positions.iter()
		.map(|y| {
			get_pin_coords(*y, size, pin_spacing).iter()
				.map(|y| add!(circuit, Pin, (-chip_width * 0.5, *y)))
				.collect()
		})
		.collect();

	let selector_pin_1 = add!(circuit, Pin, (-500.0, chip_height * 0.5));
	let selector_pin_2 = add!(circuit, Pin, (500.0, chip_height * 0.5));

	let output_pins: Vec<_> = get_pin_coords(0.0, size, pin_spacing).iter()
		.map(|y| add!(circuit, Pin, (chip_width * 0.5, *y)))
		.collect();

	let mult1 = add!(circuit, MultiMultiplexer, (-700.0, (group_positions[0] + group_positions[1]) * 0.5), size);
	let mult2 = add!(circuit, MultiMultiplexer, (-700.0, (group_positions[2] + group_positions[3]) * 0.5), size);
	let mult3 = add!(circuit, MultiMultiplexer, (700.0, 0.0), size);

	let pos = circuit.components[selector_pin_2].position;
	let junc = add!(circuit, Junction, (pos.0, pos.1 - 300.0), 3);

	circuit.connect_groups(
		&(0..size).map(|i| (mult1, i)).collect::<Vec<_>>(),
		&input_groups[0].iter().map(|i| (*i, 0usize)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(0..size).map(|i| (mult1, size + i)).collect::<Vec<_>>(),
		&input_groups[1].iter().map(|i| (*i, 0usize)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(0..size).map(|i| (mult2, i)).collect::<Vec<_>>(),
		&input_groups[2].iter().map(|i| (*i, 0usize)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(0..size).map(|i| (mult2, size + i)).collect::<Vec<_>>(),
		&input_groups[3].iter().map(|i| (*i, 0usize)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit.connect_groups(
		&(0..size).map(|i| (mult1, size * 2 + 1 + i)).collect::<Vec<_>>(),
		&(0..size).map(|i| (mult3, i)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(0..size).map(|i| (mult2, size * 2 + 1 + i)).collect::<Vec<_>>(),
		&(0..size).map(|i| (mult3, size + i)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit.connect_groups(
		&(0..size/2).map(|i| (mult3, size * 2 + 1 + i)).collect::<Vec<_>>(),
		&output_pins[..size/2].iter().map(|i| (*i, 0usize)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(size/2..size).map(|i| (mult3, size * 2 + 1 + i)).collect::<Vec<_>>(),
		&output_pins[size/2..].iter().map(|i| (*i, 0usize)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit.connect((selector_pin_1, 0), (mult3, size * 2), &[
		WireLayoutCommand::MoveVertical(-200.0),
		WireLayoutCommand::AlignVertical,
	]);

	circuit.connect((selector_pin_2, 0), (junc, 0), &[]);
	circuit.connect((junc, 1), (mult2, size * 2), &[
		WireLayoutCommand::AlignVertical,
	]);
	circuit.connect((junc, 2), (mult1, size * 2), &[
		WireLayoutCommand::MoveVertical(-500.0),
		WireLayoutCommand::MoveHorizontal(-700.0),
		WireLayoutCommand::MoveVertical(-1500.0),
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}
