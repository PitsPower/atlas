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
		for (i, word) in memory.iter().enumerate() {
			self.data[i] = *word;
		}
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
					circuit.update_component_pin(&ExternalPin {
						component_idx: idx,
						pin_idx: i,
					}, *state, true);
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
		for (i, word) in memory.iter().enumerate() {
			self.data[i] = *word;
		}
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

pub fn get_ram_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Memory, (-100.000, 0.000), 15);
	let c1 = add!(circuit, FourWayMultiMultiplexer, (2350.000, 0.000), 16);
	let c2 = add!(circuit, FourWayMultiMultiplexer, (-1300.000, 0.000), 16);
	let c3 = add!(circuit, MultiJunction, (850.000, 0.000), 16);
	let c4 = add!(circuit, MultiSwitch, (0.000, 2450.000), 16);
	let c5 = add!(circuit, MultiJunction, (0.000, 1800.000), 16);
	let c6 = add!(circuit, MultiJunction, (-3050.000, -400.000), 16, true);
	let c7 = add!(circuit, MultiJunction, (850.000, -200.000), 16);
	let c8 = add!(circuit, MultiJunction, (850.000, -800.000), 16);
	let c9 = add!(circuit, MultiTriStateBuffer, (3300.000, 0.000), 16);
	let c10 = add!(circuit, MultiJunction, (-2380.000, 1000.000), 8);
	let c11 = add!(circuit, MultiJunction, (-2380.000, 1920.000), 8);
	let c12 = add!(circuit, MultiSwitch, (0.000, -2350.000), 16);
	let c13 = add!(circuit, Junction, (2500.000, 1050.000), 3);
	let c14 = add!(circuit, Switch, (-4800.000, 100.000));
	let c15 = add!(circuit, Switch, (-4800.000, -100.000));
	let c16 = add!(circuit, Switch, (-4800.000, 300.000));
	let c17 = add!(circuit, Switch, (-4800.000, -300.000));
	let c18 = add!(circuit, AndGate, (-4350.000, 200.000));
	let c19 = add!(circuit, Junction, (-1450.000, 1250.000), 3);
	
	circuit.connect((c2, 66), (c0, 0), &[WireLayoutCommand::MoveTo((-625.000, -112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((-572.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 67), (c0, 1), &[WireLayoutCommand::MoveTo((-625.000, -97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((-587.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 68), (c0, 2), &[WireLayoutCommand::MoveTo((-625.000, -82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((-602.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 69), (c0, 3), &[WireLayoutCommand::MoveTo((-625.000, -67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((-617.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 70), (c0, 4), &[WireLayoutCommand::MoveTo((-625.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((-632.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 71), (c0, 5), &[WireLayoutCommand::MoveTo((-625.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((-647.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 72), (c0, 6), &[WireLayoutCommand::MoveTo((-625.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((-662.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 73), (c0, 7), &[WireLayoutCommand::MoveTo((-625.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((-677.500, -40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 74), (c0, 8), &[WireLayoutCommand::MoveTo((-625.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((-677.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 75), (c0, 9), &[WireLayoutCommand::MoveTo((-625.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((-662.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 76), (c0, 10), &[WireLayoutCommand::MoveTo((-625.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((-647.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 77), (c0, 11), &[WireLayoutCommand::MoveTo((-625.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((-632.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 78), (c0, 12), &[WireLayoutCommand::MoveTo((-625.000, 67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((-617.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 79), (c0, 13), &[WireLayoutCommand::MoveTo((-625.000, 82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((-602.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 80), (c0, 14), &[WireLayoutCommand::MoveTo((-625.000, 97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((-587.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 81), (c0, 15), &[WireLayoutCommand::MoveTo((-625.000, 112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((-572.500, 40.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 32), (c3, 0), &[WireLayoutCommand::MoveTo((500.000, -75.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-35.000, -0.000)), WireLayoutCommand::MoveTo((465.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 33), (c3, 3), &[WireLayoutCommand::MoveTo((500.000, -65.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.000, -0.000)), WireLayoutCommand::MoveTo((475.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 34), (c3, 6), &[WireLayoutCommand::MoveTo((500.000, -55.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, -0.000)), WireLayoutCommand::MoveTo((485.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 35), (c3, 9), &[WireLayoutCommand::MoveTo((500.000, -45.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, -0.000)), WireLayoutCommand::MoveTo((495.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 36), (c3, 12), &[WireLayoutCommand::MoveTo((500.000, -35.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, 0.000)), WireLayoutCommand::MoveTo((505.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 37), (c3, 15), &[WireLayoutCommand::MoveTo((500.000, -25.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, 0.000)), WireLayoutCommand::MoveTo((515.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 38), (c3, 18), &[WireLayoutCommand::MoveTo((500.000, -15.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.000, 0.000)), WireLayoutCommand::MoveTo((525.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 39), (c3, 21), &[WireLayoutCommand::MoveTo((500.000, -5.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((35.000, 0.000)), WireLayoutCommand::MoveTo((535.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 40), (c3, 24), &[WireLayoutCommand::MoveTo((500.000, 5.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((35.000, 0.000)), WireLayoutCommand::MoveTo((535.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 41), (c3, 27), &[WireLayoutCommand::MoveTo((500.000, 15.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.000, 0.000)), WireLayoutCommand::MoveTo((525.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 42), (c3, 30), &[WireLayoutCommand::MoveTo((500.000, 25.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, 0.000)), WireLayoutCommand::MoveTo((515.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 43), (c3, 33), &[WireLayoutCommand::MoveTo((500.000, 35.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, 0.000)), WireLayoutCommand::MoveTo((505.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 44), (c3, 36), &[WireLayoutCommand::MoveTo((500.000, 45.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, -0.000)), WireLayoutCommand::MoveTo((495.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 45), (c3, 39), &[WireLayoutCommand::MoveTo((500.000, 55.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, -0.000)), WireLayoutCommand::MoveTo((485.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 46), (c3, 42), &[WireLayoutCommand::MoveTo((500.000, 65.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.000, -0.000)), WireLayoutCommand::MoveTo((475.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 47), (c3, 45), &[WireLayoutCommand::MoveTo((500.000, 75.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-35.000, -0.000)), WireLayoutCommand::MoveTo((465.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 0), (c5, 0), &[WireLayoutCommand::MoveTo((-375.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -175.000)), WireLayoutCommand::MoveTo((-120.000, 2125.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 1), (c5, 3), &[WireLayoutCommand::MoveTo((-325.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -125.000)), WireLayoutCommand::MoveTo((-120.000, 2175.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 2), (c5, 6), &[WireLayoutCommand::MoveTo((-275.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((-120.000, 2225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 3), (c5, 9), &[WireLayoutCommand::MoveTo((-225.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((-120.000, 2275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 4), (c5, 12), &[WireLayoutCommand::MoveTo((-175.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((-120.000, 2325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 5), (c5, 15), &[WireLayoutCommand::MoveTo((-125.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((-120.000, 2375.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 6), (c5, 18), &[WireLayoutCommand::MoveTo((-75.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 125.000)), WireLayoutCommand::MoveTo((-120.000, 2425.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 7), (c5, 21), &[WireLayoutCommand::MoveTo((-25.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 175.000)), WireLayoutCommand::MoveTo((-120.000, 2475.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 8), (c5, 24), &[WireLayoutCommand::MoveTo((25.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 175.000)), WireLayoutCommand::MoveTo((120.000, 2475.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 9), (c5, 27), &[WireLayoutCommand::MoveTo((75.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 125.000)), WireLayoutCommand::MoveTo((120.000, 2425.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 10), (c5, 30), &[WireLayoutCommand::MoveTo((125.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((120.000, 2375.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 11), (c5, 33), &[WireLayoutCommand::MoveTo((175.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((120.000, 2325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 12), (c5, 36), &[WireLayoutCommand::MoveTo((225.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((120.000, 2275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 13), (c5, 39), &[WireLayoutCommand::MoveTo((275.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((120.000, 2225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 14), (c5, 42), &[WireLayoutCommand::MoveTo((325.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -125.000)), WireLayoutCommand::MoveTo((120.000, 2175.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 15), (c5, 45), &[WireLayoutCommand::MoveTo((375.000, 2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -175.000)), WireLayoutCommand::MoveTo((120.000, 2125.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 0), (c6, 45), &[WireLayoutCommand::MoveTo((-2250.000, -712.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, -0.000)), WireLayoutCommand::MoveTo((-2137.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -112.500)), WireLayoutCommand::MoveTo((-3300.000, -1162.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 1), (c6, 42), &[WireLayoutCommand::MoveTo((-2250.000, -697.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, -0.000)), WireLayoutCommand::MoveTo((-2152.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -97.500)), WireLayoutCommand::MoveTo((-3300.000, -1147.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 2), (c6, 39), &[WireLayoutCommand::MoveTo((-2250.000, -682.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, -0.000)), WireLayoutCommand::MoveTo((-2167.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -82.500)), WireLayoutCommand::MoveTo((-3300.000, -1132.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 3), (c6, 36), &[WireLayoutCommand::MoveTo((-2250.000, -667.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, -0.000)), WireLayoutCommand::MoveTo((-2182.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -67.500)), WireLayoutCommand::MoveTo((-3300.000, -1117.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 4), (c6, 33), &[WireLayoutCommand::MoveTo((-2250.000, -652.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((-2197.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -52.500)), WireLayoutCommand::MoveTo((-3300.000, -1102.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 5), (c6, 30), &[WireLayoutCommand::MoveTo((-2250.000, -637.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((-2212.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -37.500)), WireLayoutCommand::MoveTo((-3300.000, -1087.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 6), (c6, 27), &[WireLayoutCommand::MoveTo((-2250.000, -622.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((-2227.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -22.500)), WireLayoutCommand::MoveTo((-3300.000, -1072.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 7), (c6, 24), &[WireLayoutCommand::MoveTo((-2250.000, -607.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((-2242.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -7.500)), WireLayoutCommand::MoveTo((-3300.000, -1057.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 8), (c6, 21), &[WireLayoutCommand::MoveTo((-2250.000, -592.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((-2257.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 7.500)), WireLayoutCommand::MoveTo((-3300.000, -1042.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 9), (c6, 18), &[WireLayoutCommand::MoveTo((-2250.000, -577.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((-2272.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 22.500)), WireLayoutCommand::MoveTo((-3300.000, -1027.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 10), (c6, 15), &[WireLayoutCommand::MoveTo((-2250.000, -562.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((-2287.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 37.500)), WireLayoutCommand::MoveTo((-3300.000, -1012.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 11), (c6, 12), &[WireLayoutCommand::MoveTo((-2250.000, -547.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((-2302.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 52.500)), WireLayoutCommand::MoveTo((-3300.000, -997.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 12), (c6, 9), &[WireLayoutCommand::MoveTo((-2250.000, -532.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, 0.000)), WireLayoutCommand::MoveTo((-2317.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 67.500)), WireLayoutCommand::MoveTo((-3300.000, -982.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 13), (c6, 6), &[WireLayoutCommand::MoveTo((-2250.000, -517.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, 0.000)), WireLayoutCommand::MoveTo((-2332.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 82.500)), WireLayoutCommand::MoveTo((-3300.000, -967.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 14), (c6, 3), &[WireLayoutCommand::MoveTo((-2250.000, -502.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, 0.000)), WireLayoutCommand::MoveTo((-2347.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 97.500)), WireLayoutCommand::MoveTo((-3300.000, -952.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 15), (c6, 0), &[WireLayoutCommand::MoveTo((-2250.000, -487.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, 0.000)), WireLayoutCommand::MoveTo((-2362.500, -1050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 112.500)), WireLayoutCommand::MoveTo((-3300.000, -937.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 16), (c6, 46), &[WireLayoutCommand::MoveTo((-2600.000, -312.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, -0.000)), WireLayoutCommand::MoveTo((-2487.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 17), (c6, 43), &[WireLayoutCommand::MoveTo((-2600.000, -297.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, -0.000)), WireLayoutCommand::MoveTo((-2502.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 18), (c6, 40), &[WireLayoutCommand::MoveTo((-2600.000, -282.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, -0.000)), WireLayoutCommand::MoveTo((-2517.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 19), (c6, 37), &[WireLayoutCommand::MoveTo((-2600.000, -267.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, -0.000)), WireLayoutCommand::MoveTo((-2532.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 20), (c6, 34), &[WireLayoutCommand::MoveTo((-2600.000, -252.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((-2547.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 21), (c6, 31), &[WireLayoutCommand::MoveTo((-2600.000, -237.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((-2562.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 22), (c6, 28), &[WireLayoutCommand::MoveTo((-2600.000, -222.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((-2577.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 23), (c6, 25), &[WireLayoutCommand::MoveTo((-2600.000, -207.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((-2592.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 24), (c6, 22), &[WireLayoutCommand::MoveTo((-2600.000, -192.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((-2607.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 25), (c6, 19), &[WireLayoutCommand::MoveTo((-2600.000, -177.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((-2622.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 26), (c6, 16), &[WireLayoutCommand::MoveTo((-2600.000, -162.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((-2637.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 27), (c6, 13), &[WireLayoutCommand::MoveTo((-2600.000, -147.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((-2652.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 28), (c6, 10), &[WireLayoutCommand::MoveTo((-2600.000, -132.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, 0.000)), WireLayoutCommand::MoveTo((-2667.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 29), (c6, 7), &[WireLayoutCommand::MoveTo((-2600.000, -117.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, 0.000)), WireLayoutCommand::MoveTo((-2682.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 30), (c6, 4), &[WireLayoutCommand::MoveTo((-2600.000, -102.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, 0.000)), WireLayoutCommand::MoveTo((-2697.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 31), (c6, 1), &[WireLayoutCommand::MoveTo((-2600.000, -87.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, 0.000)), WireLayoutCommand::MoveTo((-2712.500, -400.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 1), (c7, 0), &[]);
	circuit.connect((c3, 4), (c7, 3), &[]);
	circuit.connect((c3, 7), (c7, 6), &[]);
	circuit.connect((c3, 10), (c7, 9), &[]);
	circuit.connect((c3, 13), (c7, 12), &[]);
	circuit.connect((c3, 16), (c7, 15), &[]);
	circuit.connect((c3, 19), (c7, 18), &[]);
	circuit.connect((c3, 22), (c7, 21), &[]);
	circuit.connect((c3, 25), (c7, 24), &[]);
	circuit.connect((c3, 28), (c7, 27), &[]);
	circuit.connect((c3, 31), (c7, 30), &[]);
	circuit.connect((c3, 34), (c7, 33), &[]);
	circuit.connect((c3, 37), (c7, 36), &[]);
	circuit.connect((c3, 40), (c7, 39), &[]);
	circuit.connect((c3, 43), (c7, 42), &[]);
	circuit.connect((c3, 46), (c7, 45), &[]);
	circuit.connect((c7, 1), (c8, 0), &[]);
	circuit.connect((c7, 4), (c8, 3), &[]);
	circuit.connect((c7, 7), (c8, 6), &[]);
	circuit.connect((c7, 10), (c8, 9), &[]);
	circuit.connect((c7, 13), (c8, 12), &[]);
	circuit.connect((c7, 16), (c8, 15), &[]);
	circuit.connect((c7, 19), (c8, 18), &[]);
	circuit.connect((c7, 22), (c8, 21), &[]);
	circuit.connect((c7, 25), (c8, 24), &[]);
	circuit.connect((c7, 28), (c8, 27), &[]);
	circuit.connect((c7, 31), (c8, 30), &[]);
	circuit.connect((c7, 34), (c8, 33), &[]);
	circuit.connect((c7, 37), (c8, 36), &[]);
	circuit.connect((c7, 40), (c8, 39), &[]);
	circuit.connect((c7, 43), (c8, 42), &[]);
	circuit.connect((c7, 46), (c8, 45), &[]);
	circuit.connect((c1, 16), (c7, 2), &[WireLayoutCommand::MoveTo((1550.000, -312.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((1602.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 17), (c7, 5), &[WireLayoutCommand::MoveTo((1550.000, -297.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((1587.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 18), (c7, 8), &[WireLayoutCommand::MoveTo((1550.000, -282.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((1572.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 19), (c7, 11), &[WireLayoutCommand::MoveTo((1550.000, -267.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((1557.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 20), (c7, 14), &[WireLayoutCommand::MoveTo((1550.000, -252.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((1542.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 21), (c7, 17), &[WireLayoutCommand::MoveTo((1550.000, -237.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((1527.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 22), (c7, 20), &[WireLayoutCommand::MoveTo((1550.000, -222.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((1512.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 23), (c7, 23), &[WireLayoutCommand::MoveTo((1550.000, -207.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((1497.500, -320.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 24), (c7, 26), &[WireLayoutCommand::MoveTo((1550.000, -192.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((1497.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 25), (c7, 29), &[WireLayoutCommand::MoveTo((1550.000, -177.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((1512.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 26), (c7, 32), &[WireLayoutCommand::MoveTo((1550.000, -162.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((1527.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 27), (c7, 35), &[WireLayoutCommand::MoveTo((1550.000, -147.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((1542.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 28), (c7, 38), &[WireLayoutCommand::MoveTo((1550.000, -132.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((1557.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 29), (c7, 41), &[WireLayoutCommand::MoveTo((1550.000, -117.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((1572.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 30), (c7, 44), &[WireLayoutCommand::MoveTo((1550.000, -102.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((1587.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 31), (c7, 47), &[WireLayoutCommand::MoveTo((1550.000, -87.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((1602.500, -80.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 0), (c8, 1), &[WireLayoutCommand::MoveTo((1500.000, -712.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, -0.000)), WireLayoutCommand::MoveTo((1612.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 1), (c8, 4), &[WireLayoutCommand::MoveTo((1500.000, -697.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, -0.000)), WireLayoutCommand::MoveTo((1597.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 2), (c8, 7), &[WireLayoutCommand::MoveTo((1500.000, -682.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, -0.000)), WireLayoutCommand::MoveTo((1582.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 3), (c8, 10), &[WireLayoutCommand::MoveTo((1500.000, -667.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, -0.000)), WireLayoutCommand::MoveTo((1567.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 4), (c8, 13), &[WireLayoutCommand::MoveTo((1500.000, -652.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((1552.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 5), (c8, 16), &[WireLayoutCommand::MoveTo((1500.000, -637.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((1537.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 6), (c8, 19), &[WireLayoutCommand::MoveTo((1500.000, -622.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((1522.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 7), (c8, 22), &[WireLayoutCommand::MoveTo((1500.000, -607.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((1507.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 8), (c8, 25), &[WireLayoutCommand::MoveTo((1500.000, -592.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((1492.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 9), (c8, 28), &[WireLayoutCommand::MoveTo((1500.000, -577.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((1477.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 10), (c8, 31), &[WireLayoutCommand::MoveTo((1500.000, -562.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((1462.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 11), (c8, 34), &[WireLayoutCommand::MoveTo((1500.000, -547.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((1447.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 12), (c8, 37), &[WireLayoutCommand::MoveTo((1500.000, -532.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, 0.000)), WireLayoutCommand::MoveTo((1432.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 13), (c8, 40), &[WireLayoutCommand::MoveTo((1500.000, -517.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, 0.000)), WireLayoutCommand::MoveTo((1417.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 14), (c8, 43), &[WireLayoutCommand::MoveTo((1500.000, -502.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, 0.000)), WireLayoutCommand::MoveTo((1402.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 15), (c8, 46), &[WireLayoutCommand::MoveTo((1500.000, -487.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, 0.000)), WireLayoutCommand::MoveTo((1387.500, -800.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 63), (c3, 47), &[WireLayoutCommand::MoveTo((970.000, 712.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 62), (c3, 44), &[WireLayoutCommand::MoveTo((970.000, 697.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 61), (c3, 41), &[WireLayoutCommand::MoveTo((970.000, 682.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 60), (c3, 38), &[WireLayoutCommand::MoveTo((970.000, 667.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 59), (c3, 35), &[WireLayoutCommand::MoveTo((970.000, 652.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 58), (c3, 32), &[WireLayoutCommand::MoveTo((970.000, 637.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 57), (c3, 29), &[WireLayoutCommand::MoveTo((970.000, 622.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 56), (c3, 26), &[WireLayoutCommand::MoveTo((970.000, 607.350)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 47), (c3, 23), &[WireLayoutCommand::MoveTo((1600.000, 312.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((1652.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 52.500)), WireLayoutCommand::MoveTo((730.000, 452.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 46), (c3, 20), &[WireLayoutCommand::MoveTo((1600.000, 297.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((1637.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 37.500)), WireLayoutCommand::MoveTo((730.000, 437.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 45), (c3, 17), &[WireLayoutCommand::MoveTo((1600.000, 282.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((1622.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 22.500)), WireLayoutCommand::MoveTo((730.000, 422.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 44), (c3, 14), &[WireLayoutCommand::MoveTo((1600.000, 267.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((1607.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 7.500)), WireLayoutCommand::MoveTo((730.000, 407.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 43), (c3, 11), &[WireLayoutCommand::MoveTo((1600.000, 252.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((1592.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -7.500)), WireLayoutCommand::MoveTo((730.000, 392.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 42), (c3, 8), &[WireLayoutCommand::MoveTo((1600.000, 237.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((1577.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -22.500)), WireLayoutCommand::MoveTo((730.000, 377.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 41), (c3, 5), &[WireLayoutCommand::MoveTo((1600.000, 222.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((1562.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -37.500)), WireLayoutCommand::MoveTo((730.000, 362.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 40), (c3, 2), &[WireLayoutCommand::MoveTo((1600.000, 207.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((1547.500, 400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -52.500)), WireLayoutCommand::MoveTo((730.000, 347.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 66), (c9, 0), &[]);
	circuit.connect((c1, 67), (c9, 1), &[]);
	circuit.connect((c1, 68), (c9, 2), &[]);
	circuit.connect((c1, 69), (c9, 3), &[]);
	circuit.connect((c1, 70), (c9, 4), &[]);
	circuit.connect((c1, 71), (c9, 5), &[]);
	circuit.connect((c1, 72), (c9, 6), &[]);
	circuit.connect((c1, 73), (c9, 7), &[]);
	circuit.connect((c1, 74), (c9, 8), &[]);
	circuit.connect((c1, 75), (c9, 9), &[]);
	circuit.connect((c1, 76), (c9, 10), &[]);
	circuit.connect((c1, 77), (c9, 11), &[]);
	circuit.connect((c1, 78), (c9, 12), &[]);
	circuit.connect((c1, 79), (c9, 13), &[]);
	circuit.connect((c1, 80), (c9, 14), &[]);
	circuit.connect((c1, 81), (c9, 15), &[]);
	circuit.connect((c5, 2), (c9, 17), &[WireLayoutCommand::MoveTo((3950.000, 1575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-225.000, -0.000)), WireLayoutCommand::MoveTo((3725.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 5), (c9, 18), &[WireLayoutCommand::MoveTo((3950.000, 1605.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-195.000, -0.000)), WireLayoutCommand::MoveTo((3755.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 8), (c9, 19), &[WireLayoutCommand::MoveTo((3950.000, 1635.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-165.000, -0.000)), WireLayoutCommand::MoveTo((3785.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 11), (c9, 20), &[WireLayoutCommand::MoveTo((3950.000, 1665.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-135.000, -0.000)), WireLayoutCommand::MoveTo((3815.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 14), (c9, 21), &[WireLayoutCommand::MoveTo((3950.000, 1695.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-105.000, -0.000)), WireLayoutCommand::MoveTo((3845.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 17), (c9, 22), &[WireLayoutCommand::MoveTo((3950.000, 1725.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-75.000, -0.000)), WireLayoutCommand::MoveTo((3875.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 20), (c9, 23), &[WireLayoutCommand::MoveTo((3950.000, 1755.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-45.000, -0.000)), WireLayoutCommand::MoveTo((3905.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 23), (c9, 24), &[WireLayoutCommand::MoveTo((3950.000, 1785.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, -0.000)), WireLayoutCommand::MoveTo((3935.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 26), (c9, 25), &[WireLayoutCommand::MoveTo((3950.000, 1815.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, 0.000)), WireLayoutCommand::MoveTo((3965.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 29), (c9, 26), &[WireLayoutCommand::MoveTo((3950.000, 1845.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((45.000, 0.000)), WireLayoutCommand::MoveTo((3995.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 32), (c9, 27), &[WireLayoutCommand::MoveTo((3950.000, 1875.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((75.000, 0.000)), WireLayoutCommand::MoveTo((4025.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 35), (c9, 28), &[WireLayoutCommand::MoveTo((3950.000, 1905.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((105.000, 0.000)), WireLayoutCommand::MoveTo((4055.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 38), (c9, 29), &[WireLayoutCommand::MoveTo((3950.000, 1935.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((135.000, 0.000)), WireLayoutCommand::MoveTo((4085.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 41), (c9, 30), &[WireLayoutCommand::MoveTo((3950.000, 1965.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((165.000, 0.000)), WireLayoutCommand::MoveTo((4115.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 44), (c9, 31), &[WireLayoutCommand::MoveTo((3950.000, 1995.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((195.000, 0.000)), WireLayoutCommand::MoveTo((4145.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 47), (c9, 32), &[WireLayoutCommand::MoveTo((3950.000, 2025.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((225.000, 0.000)), WireLayoutCommand::MoveTo((4175.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 40), (c8, 26), &[WireLayoutCommand::MoveTo((-2100.000, 207.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 207.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((-3497.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 52.500)), WireLayoutCommand::MoveTo((970.000, -1397.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 41), (c8, 29), &[WireLayoutCommand::MoveTo((-2100.000, 222.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 222.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((-3512.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 37.500)), WireLayoutCommand::MoveTo((970.000, -1412.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 42), (c8, 32), &[WireLayoutCommand::MoveTo((-2100.000, 237.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 237.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((-3527.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 22.500)), WireLayoutCommand::MoveTo((970.000, -1427.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 43), (c8, 35), &[WireLayoutCommand::MoveTo((-2100.000, 252.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 252.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((-3542.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 7.500)), WireLayoutCommand::MoveTo((970.000, -1442.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 44), (c8, 38), &[WireLayoutCommand::MoveTo((-2100.000, 267.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 267.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((-3557.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -7.500)), WireLayoutCommand::MoveTo((970.000, -1457.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 45), (c8, 41), &[WireLayoutCommand::MoveTo((-2100.000, 282.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 282.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((-3572.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -22.500)), WireLayoutCommand::MoveTo((970.000, -1472.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 46), (c8, 44), &[WireLayoutCommand::MoveTo((-2100.000, 297.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 297.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((-3587.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -37.500)), WireLayoutCommand::MoveTo((970.000, -1487.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 47), (c8, 47), &[WireLayoutCommand::MoveTo((-2100.000, 312.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-3550.000, 312.450)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((-3602.500, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -52.500)), WireLayoutCommand::MoveTo((970.000, -1502.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 48), (c8, 2), &[WireLayoutCommand::MoveTo((-3800.000, 487.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((-3747.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 52.500)), WireLayoutCommand::MoveTo((730.000, -1647.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 49), (c8, 5), &[WireLayoutCommand::MoveTo((-3800.000, 502.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((-3762.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 37.500)), WireLayoutCommand::MoveTo((730.000, -1662.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 50), (c8, 8), &[WireLayoutCommand::MoveTo((-3800.000, 517.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((-3777.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 22.500)), WireLayoutCommand::MoveTo((730.000, -1677.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 51), (c8, 11), &[WireLayoutCommand::MoveTo((-3800.000, 532.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((-3792.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 7.500)), WireLayoutCommand::MoveTo((730.000, -1692.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 52), (c8, 14), &[WireLayoutCommand::MoveTo((-3800.000, 547.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((-3807.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -7.500)), WireLayoutCommand::MoveTo((730.000, -1707.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 53), (c8, 17), &[WireLayoutCommand::MoveTo((-3800.000, 562.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((-3822.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -22.500)), WireLayoutCommand::MoveTo((730.000, -1722.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 54), (c8, 20), &[WireLayoutCommand::MoveTo((-3800.000, 577.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((-3837.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -37.500)), WireLayoutCommand::MoveTo((730.000, -1737.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 55), (c8, 23), &[WireLayoutCommand::MoveTo((-3800.000, 592.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((-3852.500, -1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -52.500)), WireLayoutCommand::MoveTo((730.000, -1752.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 46), (c11, 21), &[]);
	circuit.connect((c5, 43), (c11, 18), &[]);
	circuit.connect((c5, 40), (c11, 15), &[]);
	circuit.connect((c5, 37), (c11, 12), &[]);
	circuit.connect((c5, 34), (c11, 9), &[]);
	circuit.connect((c5, 31), (c11, 6), &[]);
	circuit.connect((c5, 28), (c11, 3), &[]);
	circuit.connect((c5, 25), (c11, 0), &[]);
	circuit.connect((c11, 22), (c6, 2), &[WireLayoutCommand::MoveTo((-3170.000, 2025.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 19), (c6, 5), &[WireLayoutCommand::MoveTo((-3170.000, 1995.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 16), (c6, 8), &[WireLayoutCommand::MoveTo((-3170.000, 1965.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 13), (c6, 11), &[WireLayoutCommand::MoveTo((-3170.000, 1935.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 10), (c6, 14), &[WireLayoutCommand::MoveTo((-3170.000, 1905.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 7), (c6, 17), &[WireLayoutCommand::MoveTo((-3170.000, 1875.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 4), (c6, 20), &[WireLayoutCommand::MoveTo((-3170.000, 1845.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 1), (c6, 23), &[WireLayoutCommand::MoveTo((-3170.000, 1815.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 1), (c6, 47), &[WireLayoutCommand::MoveTo((-2930.000, 1575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 4), (c6, 44), &[WireLayoutCommand::MoveTo((-2930.000, 1605.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 7), (c6, 41), &[WireLayoutCommand::MoveTo((-2930.000, 1635.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 10), (c6, 38), &[WireLayoutCommand::MoveTo((-2930.000, 1665.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 13), (c6, 35), &[WireLayoutCommand::MoveTo((-2930.000, 1695.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 16), (c6, 32), &[WireLayoutCommand::MoveTo((-2930.000, 1725.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 19), (c6, 29), &[WireLayoutCommand::MoveTo((-2930.000, 1755.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 22), (c6, 26), &[WireLayoutCommand::MoveTo((-2930.000, 1785.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 2), (c10, 0), &[]);
	circuit.connect((c11, 5), (c10, 3), &[]);
	circuit.connect((c11, 8), (c10, 6), &[]);
	circuit.connect((c11, 11), (c10, 9), &[]);
	circuit.connect((c11, 14), (c10, 12), &[]);
	circuit.connect((c11, 17), (c10, 15), &[]);
	circuit.connect((c11, 20), (c10, 18), &[]);
	circuit.connect((c11, 23), (c10, 21), &[]);
	circuit.connect((c2, 56), (c10, 1), &[WireLayoutCommand::MoveTo((-2050.000, 607.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((-2102.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 57), (c10, 4), &[WireLayoutCommand::MoveTo((-2050.000, 622.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((-2087.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 58), (c10, 7), &[WireLayoutCommand::MoveTo((-2050.000, 637.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((-2072.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 59), (c10, 10), &[WireLayoutCommand::MoveTo((-2050.000, 652.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((-2057.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 60), (c10, 13), &[WireLayoutCommand::MoveTo((-2050.000, 667.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((-2042.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 61), (c10, 16), &[WireLayoutCommand::MoveTo((-2050.000, 682.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((-2027.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 62), (c10, 19), &[WireLayoutCommand::MoveTo((-2050.000, 697.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((-2012.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 63), (c10, 22), &[WireLayoutCommand::MoveTo((-2050.000, 712.350)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((-1997.500, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 32), (c10, 2), &[WireLayoutCommand::MoveTo((-2380.000, 87.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 33), (c10, 5), &[WireLayoutCommand::MoveTo((-2380.000, 102.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 34), (c10, 8), &[WireLayoutCommand::MoveTo((-2380.000, 117.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 35), (c10, 11), &[WireLayoutCommand::MoveTo((-2380.000, 132.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 36), (c10, 14), &[WireLayoutCommand::MoveTo((-2380.000, 147.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 37), (c10, 17), &[WireLayoutCommand::MoveTo((-2380.000, 162.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 38), (c10, 20), &[WireLayoutCommand::MoveTo((-2380.000, 177.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 39), (c10, 23), &[WireLayoutCommand::MoveTo((-2380.000, 192.450)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 16), (c12, 0), &[WireLayoutCommand::MoveTo((-320.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 70.000)), WireLayoutCommand::MoveTo((4500.000, 1020.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((70.000, 0.000)), WireLayoutCommand::MoveTo((4570.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -70.000)), WireLayoutCommand::MoveTo((-25.000, -2020.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 17), (c12, 1), &[WireLayoutCommand::MoveTo((-310.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 60.000)), WireLayoutCommand::MoveTo((4500.000, 1010.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((60.000, 0.000)), WireLayoutCommand::MoveTo((4560.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -60.000)), WireLayoutCommand::MoveTo((-25.000, -2010.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 18), (c12, 2), &[WireLayoutCommand::MoveTo((-300.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 50.000)), WireLayoutCommand::MoveTo((4500.000, 1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((50.000, 0.000)), WireLayoutCommand::MoveTo((4550.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -50.000)), WireLayoutCommand::MoveTo((-25.000, -2000.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 19), (c12, 3), &[WireLayoutCommand::MoveTo((-290.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 40.000)), WireLayoutCommand::MoveTo((4500.000, 990.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((40.000, 0.000)), WireLayoutCommand::MoveTo((4540.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -40.000)), WireLayoutCommand::MoveTo((-25.000, -1990.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 20), (c12, 4), &[WireLayoutCommand::MoveTo((-280.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 30.000)), WireLayoutCommand::MoveTo((4500.000, 980.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((30.000, 0.000)), WireLayoutCommand::MoveTo((4530.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -30.000)), WireLayoutCommand::MoveTo((-25.000, -1980.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 21), (c12, 5), &[WireLayoutCommand::MoveTo((-270.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 20.000)), WireLayoutCommand::MoveTo((4500.000, 970.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((20.000, 0.000)), WireLayoutCommand::MoveTo((4520.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -20.000)), WireLayoutCommand::MoveTo((-25.000, -1970.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 22), (c12, 6), &[WireLayoutCommand::MoveTo((-260.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 10.000)), WireLayoutCommand::MoveTo((4500.000, 960.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((10.000, 0.000)), WireLayoutCommand::MoveTo((4510.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -10.000)), WireLayoutCommand::MoveTo((-25.000, -1960.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 23), (c12, 7), &[WireLayoutCommand::MoveTo((-250.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4500.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4500.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-25.000, -1950.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 24), (c12, 8), &[WireLayoutCommand::MoveTo((-240.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -10.000)), WireLayoutCommand::MoveTo((4500.000, 940.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-10.000, -0.000)), WireLayoutCommand::MoveTo((4490.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 10.000)), WireLayoutCommand::MoveTo((-25.000, -1940.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 25), (c12, 9), &[WireLayoutCommand::MoveTo((-230.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -20.000)), WireLayoutCommand::MoveTo((4500.000, 930.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-20.000, -0.000)), WireLayoutCommand::MoveTo((4480.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 20.000)), WireLayoutCommand::MoveTo((-25.000, -1930.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 26), (c12, 10), &[WireLayoutCommand::MoveTo((-220.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -30.000)), WireLayoutCommand::MoveTo((4500.000, 920.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-30.000, -0.000)), WireLayoutCommand::MoveTo((4470.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 30.000)), WireLayoutCommand::MoveTo((-25.000, -1920.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 27), (c12, 11), &[WireLayoutCommand::MoveTo((-210.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -40.000)), WireLayoutCommand::MoveTo((4500.000, 910.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-40.000, -0.000)), WireLayoutCommand::MoveTo((4460.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 40.000)), WireLayoutCommand::MoveTo((-25.000, -1910.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 28), (c12, 12), &[WireLayoutCommand::MoveTo((-200.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -50.000)), WireLayoutCommand::MoveTo((4500.000, 900.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-50.000, -0.000)), WireLayoutCommand::MoveTo((4450.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 50.000)), WireLayoutCommand::MoveTo((-25.000, -1900.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 29), (c12, 13), &[WireLayoutCommand::MoveTo((-190.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -60.000)), WireLayoutCommand::MoveTo((4500.000, 890.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-60.000, -0.000)), WireLayoutCommand::MoveTo((4440.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 60.000)), WireLayoutCommand::MoveTo((-25.000, -1890.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 30), (c12, 14), &[WireLayoutCommand::MoveTo((-180.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -70.000)), WireLayoutCommand::MoveTo((4500.000, 880.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-70.000, -0.000)), WireLayoutCommand::MoveTo((4430.000, -1950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 70.000)), WireLayoutCommand::MoveTo((-25.000, -1880.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 15), (c13, 0), &[WireLayoutCommand::MoveTo((375.000, -2050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4600.000, -2050.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4600.000, 1050.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c13, 1), (c1, 65), &[]);
	circuit.connect((c13, 2), (c2, 65), &[WireLayoutCommand::MoveTo((-1150.000, 1050.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c14, 0), (c18, 0), &[WireLayoutCommand::MoveTo((-4502.750, 100.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-4502.750, 170.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 0), (c18, 1), &[WireLayoutCommand::MoveTo((-4502.750, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-4502.750, 230.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c18, 2), (c0, 31), &[WireLayoutCommand::MoveTo((-4200.000, 200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-4200.000, 1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((50.000, 1400.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c15, 0), (c9, 16), &[WireLayoutCommand::MoveTo((-4250.000, -100.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-4200.000, -100.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-4200.000, -1300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((3300.000, -1300.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c17, 0), (c19, 0), &[WireLayoutCommand::MoveTo((-4050.000, -300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-4050.000, 1250.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c19, 1), (c2, 64), &[]);
	circuit.connect((c19, 2), (c1, 64), &[WireLayoutCommand::MoveTo((2200.000, 1250.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	
	circuit.pinify(&mut [c4, c12, c14, c15, c16, c17]);

	circuit
}
