//! Memory components (e.g. ROM, RAM, etc.).

use crate::add;
use crate::bus::BusLayoutCommand;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;
use crate::utils::get_pin_coords;

static mut TEST: usize = 0;

pub fn get_rom_circuit(address_size: usize, inner_scale: f64) -> Circuit {
	let chip_width = 500.0;
	let spacing = 10.0 / inner_scale;

	let mut circuit = Circuit::new();

	// The address
	let inputs: Vec<_> = get_pin_coords(0.0, address_size, spacing).iter()
		.map(|y| add!(circuit, Pin, (-chip_width * 0.5 / inner_scale, *y)))
		.collect();

	let outputs: Vec<_> = get_pin_coords(0.0, 16, spacing).iter()
		.map(|y| add!(circuit, Pin, (chip_width * 0.5 / inner_scale, *y)))
		.collect();

	if address_size == 0 {
		let multi_switch = add!(circuit, MultiSwitch, (0.0, -150.0 / inner_scale), 16);

		unsafe {
			circuit.toggle_switch(TEST);
			TEST += 1;
		}

		for (i, output) in outputs.iter().enumerate() {
			circuit.connect((multi_switch, i), (*output, 0), &[WireLayoutCommand::AlignHorizontal]);
		}
	} else {
		let junction = add!(circuit, MultiJunction, (-200.0 / inner_scale, 0.0), address_size - 1);
		let rom1 = add!(circuit, Rom, (-120.0 / inner_scale, -100.0 / inner_scale), address_size - 1);
		let rom2 = add!(circuit, Rom, (-120.0 / inner_scale, 100.0 / inner_scale), address_size - 1);
		let multiplexer = add!(circuit, MultiMultiplexer, (70.0 / inner_scale, 0.0), 16);

		let junction_pins_1: Vec<_> = (0..address_size-1).map(|i| (junction, i * 3)).collect();
		let junction_pins_2: Vec<_> = (0..address_size-1).map(|i| (junction, i * 3 + 1)).collect();
		let junction_pins_3: Vec<_> = (0..address_size-1).map(|i| (junction, i * 3 + 2)).collect();

		let rom_inputs: Vec<_> = inputs.iter().map(|i| (*i, 0)).collect();
		let rom_outputs: Vec<_> = outputs.iter().map(|i| (*i, 0)).collect();

		let rom1_inputs: Vec<_> = (0..address_size-1).map(|i| (rom1, i)).collect();
		let rom2_inputs: Vec<_> = (0..address_size-1).map(|i| (rom2, i)).collect();

		let rom1_outputs: Vec<_> = (address_size-1..address_size-1+16).map(|i| (rom1, i)).collect();
		let rom2_outputs: Vec<_> = (address_size-1..address_size-1+16).map(|i| (rom2, i)).collect();
		
		let mult_inputs_1: Vec<_> = (0..16).map(|i| (multiplexer, i)).collect();
		let mult_inputs_2: Vec<_> = (16..32).map(|i| (multiplexer, i)).collect();

		let mult_outputs: Vec<_> = (33..33+16).map(|i| (multiplexer, i)).collect();

		circuit.connect_groups(&rom_inputs[1..], &junction_pins_1, &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((inputs[0], 0), (multiplexer, 32), &[
			WireLayoutCommand::MoveHorizontal(200.0),
			WireLayoutCommand::MoveVertical(1000.0),
			WireLayoutCommand::AlignVertical,
		]);

		circuit.connect_groups(&junction_pins_2, &rom1_inputs, &[
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&junction_pins_3, &rom2_inputs, &[
			BusLayoutCommand::AlignHorizontal,
		]);

		circuit.connect_groups(&rom1_outputs, &mult_inputs_1, &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&rom2_outputs, &mult_inputs_2, &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&mult_outputs, &rom_outputs, &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&mult_outputs[..8], &rom_outputs[..8], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&mult_outputs[8..], &rom_outputs[8..], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
	}

	circuit
}
