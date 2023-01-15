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

pub fn get_memory_circuit(address_size: usize, inner_scale: f64) -> Circuit {
	let chip_width = 900.0;
	let chip_height = 700.0;
	let spacing = 10.0 / inner_scale;

	let mut circuit = Circuit::new();

	let inputs: Vec<_> = get_pin_coords(0.0, 16, spacing).iter()
		.map(|y| add!(circuit, Pin, (-chip_width * 0.5 / inner_scale, *y)))
		.collect();

	let address: Vec<_> = get_pin_coords(0.0, address_size, spacing).iter()
		.map(|x| add!(circuit, Pin, (*x - 150.0 / inner_scale, chip_height * 0.5 / inner_scale)))
		.collect();

	let clock = add!(circuit, Pin, (150.0 / inner_scale, chip_height * 0.5 / inner_scale));

	let outputs: Vec<_> = get_pin_coords(0.0, 16, spacing).iter()
		.map(|y| add!(circuit, Pin, (chip_width * 0.5 / inner_scale, *y)))
		.collect();

	let input_pins: Vec<_> = inputs.iter().map(|i| (*i, 0)).collect();
	let address_pins: Vec<_> = address.iter().map(|i| (*i, 0)).collect();
	let output_pins: Vec<_> = outputs.iter().map(|i| (*i, 0)).collect();

	if address_size == 0 {
		let flipflop = add!(circuit, MultiDFlipFlop, (0.0, 0.0), 16);
		
		let flipflop_input_pins: Vec<_> = (0..16).map(|i| (flipflop, i)).rev().collect();
		let flipflop_output_pins: Vec<_> = (17..17+16).map(|i| (flipflop, i)).rev().collect();

		circuit.connect_groups(&input_pins[..8], &flipflop_input_pins[..8], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&input_pins[8..], &flipflop_input_pins[8..], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);

		circuit.connect_groups(&flipflop_output_pins[..8], &output_pins[..8], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&flipflop_output_pins[8..], &output_pins[8..], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((clock, 0), (flipflop, 16), &[
			WireLayoutCommand::CenterVertical,
			WireLayoutCommand::AlignVertical,
		]);
	} else {
		let input_junction = add!(circuit, MultiJunction, (-290.0 / inner_scale, 0.0), 16);
		let address_junction = add!(circuit, MultiJunction, (-150.0 / inner_scale, 240.0 / inner_scale), address_size - 1);

		let address_bit_junction_1_x = circuit.components[address[0]].position.0;
		let address_bit_junction_1 = add!(circuit, Junction, (address_bit_junction_1_x, 300.0 / inner_scale), 3); 

		let memory1 = add!(circuit, Memory, (-120.0 / inner_scale, -100.0 / inner_scale), address_size - 1);
		let memory2 = add!(circuit, Memory, (-120.0 / inner_scale, 100.0 / inner_scale), address_size - 1);
		let multiplexer = add!(circuit, MultiMultiplexer, (120.0 / inner_scale, 0.0), 16);

		let not_gate = add!(circuit, NotGate, (-350.0 / inner_scale, 200.0 / inner_scale));
		let and_gate_1 = add!(circuit, AndGate, (-300.0 / inner_scale, 200.0 / inner_scale));
		let and_gate_2 = add!(circuit, AndGate, (-300.0 / inner_scale, 240.0 / inner_scale));

		let and_gate_1_pin_y = circuit.components[and_gate_2].position.1
			+ circuit.components[and_gate_2].get_pin_positions()[0].1;
		let address_bit_junction_2 = add!(circuit, Junction, (-370.0 / inner_scale, and_gate_1_pin_y), 3);

		let and_gate_2_pin_y = circuit.components[and_gate_2].position.1
			+ circuit.components[and_gate_2].get_pin_positions()[1].1;
		let clock_junction = add!(circuit, Junction, (-320.0 / inner_scale, and_gate_2_pin_y), 3);
		
		let input_junction_pins_1: Vec<_> = (0..16).map(|i| (input_junction, i * 3)).collect();
		let input_junction_pins_2: Vec<_> = (0..16).map(|i| (input_junction, i * 3 + 1)).collect();
		let input_junction_pins_3: Vec<_> = (0..16).map(|i| (input_junction, i * 3 + 2)).collect();
		
		let address_junction_pins_1: Vec<_> = (0..address_size-1).map(|i| (address_junction, i * 3)).collect();
		let address_junction_pins_2: Vec<_> = (0..address_size-1).map(|i| (address_junction, i * 3 + 1)).collect();
		let address_junction_pins_3: Vec<_> = (0..address_size-1).map(|i| (address_junction, i * 3 + 2)).collect();

		let memory1_input_pins: Vec<_> = (0..16).map(|i| (memory1, i)).collect();
		let memory2_input_pins: Vec<_> = (0..16).map(|i| (memory2, i)).collect();

		let memory1_address_pins: Vec<_> = (16..16+address_size-1).map(|i| (memory1, i)).collect();
		let memory2_address_pins: Vec<_> = (16..16+address_size-1).map(|i| (memory2, i)).collect();

		let memory1_output_pins: Vec<_> = (16+(address_size-1)+1..16+(address_size-1)+1+16).map(|i| (memory1, i)).collect();
		let memory2_output_pins: Vec<_> = (16+(address_size-1)+1..16+(address_size-1)+1+16).map(|i| (memory2, i)).collect();

		let mult_inputs_1: Vec<_> = (0..16).map(|i| (multiplexer, i)).collect();
		let mult_inputs_2: Vec<_> = (16..32).map(|i| (multiplexer, i)).collect();

		let mult_outputs: Vec<_> = (33..33+16).map(|i| (multiplexer, i)).collect();

		circuit.connect_groups(&input_pins[..8], &input_junction_pins_1[..8], &[
			BusLayoutCommand::MoveHorizontal(300.0),
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&input_pins[8..], &input_junction_pins_1[8..], &[
			BusLayoutCommand::MoveHorizontal(300.0),
			BusLayoutCommand::AlignHorizontal,
		]);
		
		circuit.connect_groups(&input_junction_pins_2, &memory1_input_pins, &[
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&input_junction_pins_3, &memory2_input_pins, &[
			BusLayoutCommand::AlignHorizontal,
		]);
		
		circuit.connect_groups(&memory1_output_pins, &mult_inputs_1, &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&memory2_output_pins, &mult_inputs_2, &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		
		circuit.connect_groups(&mult_outputs[..8], &output_pins[..8], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);
		circuit.connect_groups(&mult_outputs[8..], &output_pins[8..], &[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		]);

		if address_size > 1 {
			circuit.connect_groups(&address_junction_pins_1[..address_size/2-1], &address_pins[1..address_size/2], &[
				BusLayoutCommand::CenterVertical,
				BusLayoutCommand::AlignVertical,
			]);
			circuit.connect_groups(&address_junction_pins_1[address_size/2-1..], &address_pins[address_size/2..], &[
				BusLayoutCommand::CenterVertical,
				BusLayoutCommand::AlignVertical,
			]);
			
			circuit.connect_groups(&memory2_address_pins[..address_size/2-1], &address_junction_pins_2[..address_size/2-1], &[
				BusLayoutCommand::CenterVertical,
				BusLayoutCommand::AlignVertical,
			]);
			circuit.connect_groups(&memory2_address_pins[address_size/2-1..], &address_junction_pins_2[address_size/2-1..], &[
				BusLayoutCommand::CenterVertical,
				BusLayoutCommand::AlignVertical,
			]);
		}

		circuit.connect_groups(&memory1_address_pins, &address_junction_pins_3, &[
			BusLayoutCommand::MoveVertical(150.0),
			BusLayoutCommand::MoveHorizontal(750.0),
			BusLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((address[0], 0), (address_bit_junction_1, 0), &[]);
		circuit.connect((address_bit_junction_1, 1), (multiplexer, 32), &[
			WireLayoutCommand::AlignVertical,
		]);

		circuit.connect((address_bit_junction_1, 2), (address_bit_junction_2, 0), &[
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((address_bit_junction_2, 1), (not_gate, 0), &[
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((address_bit_junction_2, 2), (and_gate_2, 0), &[]);

		circuit.connect((clock, 0), (clock_junction, 0), &[
			WireLayoutCommand::MoveVertical(-70.0),
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((clock_junction, 1), (and_gate_1, 1), &[
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock_junction, 2), (and_gate_2, 1), &[]);
		
		circuit.connect((not_gate, 1), (and_gate_1, 0), &[
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((and_gate_1, 2), (memory1, 16 + address_size - 1), &[
			WireLayoutCommand::MoveHorizontal(50.0),
			WireLayoutCommand::MoveVertical(-150.0),
			WireLayoutCommand::MoveHorizontal(250.0),
			WireLayoutCommand::MoveVertical(-800.0),
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((and_gate_2, 2), (memory2, 16 + address_size - 1), &[
			WireLayoutCommand::MoveHorizontal(100.0),
			WireLayoutCommand::MoveVertical(-300.0),
			WireLayoutCommand::AlignVertical,
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

		// TODO: Use `self.size` instead of being DUMB

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
}

/// A [`ComponentSimulator`] for memory.
pub struct MemorySimulator {
	/// The size of the address.
	size: usize,

	/// The states of the input pins.
	input_states: Vec<PinState>,
	/// The current input.
	input: u16,

	/// The states of the address pins.
	address_states: Vec<PinState>,
	/// The current address.
	address: usize,

	/// The current clock state.
	clock_state: PinState,

	/// The data stored in memory (stored in 16-bit words).
	data: Vec<u16>,
}

impl MemorySimulator {
	/// Returns a new [`MemorySimulator`].
	pub fn new(size: usize) -> Self {
		Self {
			size,

			input_states: vec![PinState::Disconnected; 16],
			input: 0x0000,

			address_states: vec![PinState::Disconnected; size],
			address: 0,

			clock_state: PinState::Disconnected,

			data: vec![0x0000; 2_usize.pow(size as u32)],
		}
	}
}

impl ComponentSimulator for MemorySimulator {
    fn give_memory(&mut self, memory: &[u16]) {
		self.data = memory.to_vec();
	}

    fn take_memory(&self) -> &[u16] {
		&self.data
	}

    fn set_mode_to_high_level(&mut self, circuit: &Circuit) {
		if self.size == 0 {
			let flipflop = circuit.components.iter()
				.find(|c| c.get_type() == ComponentType::MultiDFlipFlop)
				.unwrap();

			let states = (17..17+16).map(|i| flipflop.get_pin_state(i).unwrap()).rev().collect();
			let value = states_to_num(&states) as u16;
			self.data[0] = value;
		} else {
			let mut memories = circuit.components.iter()
				.filter(|c| c.get_type() == ComponentType::Memory);

			let memory1 = memories.next().unwrap();
			let memory2 = memories.next().unwrap();

			let data_size = self.data.len();

			for i in 0..data_size/2 {
				self.data[i] = memory1.simulator.as_ref().unwrap().take_memory()[i];
			}
			for i in data_size/2..data_size {
				self.data[i] = memory2.simulator.as_ref().unwrap().take_memory()[i - data_size / 2];
			}
		}

		for i in 0..16+self.size+1 {
			let state = circuit.get_pin(i).unwrap().get_pin_state(0).unwrap();
			self.set_pin_state_high_level(i, state).unwrap();
		}
	}

    fn set_mode_to_circuit(&mut self, circuit: &mut Circuit) {
		if self.size == 0 {
			let states = num_to_states(self.data[0] as u32, 16);

			circuit.set_pin(16, PinState::Off);

			for (i, state) in states.iter().enumerate() {
				circuit.set_pin(i, *state);
			}

			circuit.set_pin(16, PinState::On);
		} else {
			let mut memories = circuit.components.iter()
				.enumerate()
				.filter(|(_, c)| c.get_type() == ComponentType::Memory);

			let memory1 = memories.next().unwrap().0;
			let memory2 = memories.next().unwrap().0;

			let data_size = self.data.len();

			circuit.set_memory(memory1, &self.data[..data_size / 2]);
			circuit.set_memory(memory2, &self.data[data_size / 2..]);
		}

		// Update input, address, and clock pins

		for (i, state) in self.input_states.iter().enumerate() {
			circuit.set_pin(i, *state);
		}
		for (i, state) in self.address_states.iter().enumerate() {
			circuit.set_pin(16 + i, *state);
		}
		
		circuit.set_pin(16 + self.size, self.clock_state);
	}

    fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= 16 + self.size + 1 + 16 {
			Err(PinError::OutOfRange)
		} else if idx < 16 + self.size + 1 {
			Ok(PinState::Disconnected)
		} else {
			let result = self.data[self.address];
			let states = num_to_states(result as u32, 16);
			Ok(states[idx - (16 + self.size + 1)])
		}
	}

    fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= 16 + self.size + 1 + 16 {
			Err(PinError::OutOfRange)
		} else if idx < 16 {
			self.input_states[idx] = state;
			self.input = states_to_num(&self.input_states) as u16;
			Ok(())
		} else if idx < 16 + self.size {
			self.address_states[idx - 16] = state;
			self.address = states_to_num(&self.address_states) as usize;
			Ok(())
		} else if idx == 16 + self.size {
			if self.clock_state != PinState::On && state == PinState::On {
				self.data[self.address] = self.input;
			}

			self.clock_state = state;
			Ok(())
		} else {
			Ok(())
		}
	}
}
