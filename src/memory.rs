//! Memory components (e.g. ROM, RAM, etc.).

use crate::add;
use crate::bus::BusLayoutCommand;
use crate::core::{Circuit, ComponentOptions, ComponentSimulator, ComponentType, ExternalPin, PinError, PinState};
use crate::graphics::WireLayoutCommand;
use crate::utils::{get_pin_coords, num_to_states, states_to_num};

pub fn get_rom_circuit(address_size: usize, inner_scale: f64) -> Circuit {
	let chip_width = 700.0;
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

		for (i, output) in outputs.iter().enumerate() {
			circuit.connect((multi_switch, i), (*output, 0), &[WireLayoutCommand::AlignHorizontal]);
		}
	} else {
		let junction = add!(circuit, MultiJunction, (-250.0 / inner_scale, 0.0), address_size - 1);
		let rom1 = add!(circuit, Rom, (-120.0 / inner_scale, -100.0 / inner_scale), address_size - 1);
		let rom2 = add!(circuit, Rom, (-120.0 / inner_scale, 100.0 / inner_scale), address_size - 1);
		let multiplexer = add!(circuit, MultiMultiplexer, (120.0 / inner_scale, 0.0), 16);

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

		circuit.connect_groups(&rom_inputs[1..(address_size-1)/2+1], &junction_pins_1[..(address_size-1)/2], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&rom_inputs[(address_size-1)/2+1..], &junction_pins_1[(address_size-1)/2..], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((inputs[0], 0), (multiplexer, 32), &[
			WireLayoutCommand::MoveHorizontal(200.0),
			WireLayoutCommand::MoveYTo(1000.0),
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

/// A [`ComponentSimulator`] for read only memory.
pub struct RomSimulator {
	/// The size of the address.
	size: usize,
	/// The states of the address pins.
	address_states: Vec<PinState>,
	/// The current address.
	address: usize,
	/// The data stored in memory (stored in 16-bit words).
	data: Vec<u16>,
}

impl RomSimulator {
	/// Returns a new [`RomSimulator`].
	pub fn new(size: usize) -> Self {
		Self {
			size,
			address_states: vec![PinState::Disconnected; size],
			address: 0,
			data: vec![0x0000; 2_usize.pow(size as u32)],
		}
	}
}

impl ComponentSimulator for RomSimulator {
	fn give_memory(&mut self, memory: &[u16]) {
		self.data = memory.to_vec();
	}

	fn set_mode_to_high_level(&mut self, circuit: &Circuit) {
		for i in 0..self.size {
			let state = circuit.get_pin(i).unwrap().get_pin_state(0).unwrap();
			self.set_pin_state_high_level(i, state).unwrap();
		}
	}

    fn set_mode_to_circuit(&mut self, circuit: &mut Circuit) {
		// Either set the multi-switch to the correct value
		// or give half of the memory to each ROM

		let (idx, comp) = circuit.components.iter()
			.enumerate()
			.find(|(_, c)| c.get_type() != ComponentType::Pin)
			.unwrap();

		if comp.get_type() == ComponentType::MultiSwitch {
			let states = num_to_states(self.data[0] as u32, 16);

			for (i, state) in states.iter().enumerate() {
				if *state == PinState::On {
					circuit.update_component(&ExternalPin { component_idx: idx, pin_idx: i }, *state, true);
				}
			}
		} else {
			let data_size = self.data.len();

			circuit.set_memory(idx + 1, &self.data[..data_size / 2]);
			circuit.set_memory(idx + 2, &self.data[data_size / 2..]);
		}

		// Update address pins
		for (i, state) in self.address_states.iter().enumerate() {
			circuit.set_pin(i, *state);
		}
	}

    fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.size + 16 {
			Err(PinError::OutOfRange)
		} else if idx < self.size {
			Ok(PinState::Disconnected)
		} else {
			let result = self.data[self.address];
			let states = num_to_states(result as u32, 16);
			Ok(states[idx - self.size])
		}
	}

    fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.size + 16 {
			Err(PinError::OutOfRange)
		} else if idx < self.size {
			self.address_states[idx] = state;
			self.address = states_to_num(&self.address_states) as usize;
			Ok(())
		} else {
			Ok(())
		}
	}

    fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		self.get_pin_state_high_level(idx)
	}

    fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		self.set_pin_state_high_level(idx, state)
	}
}
