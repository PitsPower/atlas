//! Adder components.

use crate::add;
use crate::core::{
	ChipInternals, Circuit, Junction, num_to_states, Pin, PinError,
	PinState, RectangleChip, SimulationMode, states_to_num, TextInfo,
};
use crate::gates::{AndGate, OrGate, XorGate};
use crate::graphics::WireLayoutCommand;

pub struct HalfAdder {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	text: Option<TextInfo>,

	input1: PinState,
	input2: PinState,
}

impl HalfAdder {
	pub fn new(pos: (f64, f64)) -> Self {
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

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.4,
			},
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			text: Some(TextInfo {
				text: String::from("Half Adder"),
				size: 27,
			}),

			input1: PinState::Disconnected,
			input2: PinState::Disconnected,
		}
	}
}

impl RectangleChip for HalfAdder {
	fn get_chip_name(&self) -> String {
		String::from("HalfAdder")
	}

	fn get_chip_internals(&self) -> &ChipInternals {
		&self.internals
	}

	fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		&mut self.internals
	}

	fn get_chip_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_chip_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_chip_size(&self) -> (f64, f64) {
		(200.0, 200.0)
	}
	
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

	fn set_mode(&mut self, mode: SimulationMode) {
		match (self.sim_mode, mode) {
			(SimulationMode::HighLevel, SimulationMode::Circuit) => {
				self.internals.circuit.set_pin(0, self.input1);
				self.internals.circuit.set_pin(1, self.input2);
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				self.input1 = self.internals.circuit.get_pins()[0].get_pin_state(0).unwrap();
				self.input2 = self.internals.circuit.get_pins()[1].get_pin_state(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
	}

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 | 1 => Ok(PinState::Disconnected),
			2 => Ok(PinState::from_bool(self.input1.to_bool() && self.input2.to_bool())),
			3 => Ok(self.input1.xor(self.input2)),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => { self.input1 = state; Ok(()) },
			1 => { self.input2 = state; Ok(()) },
			2 | 3 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
	}
}

pub struct FullAdder {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	text: Option<TextInfo>,

	input1: PinState,
	input2: PinState,
	carry_in: PinState,
}

impl FullAdder {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();
		
		let half_adder_1 = add!(circuit, HalfAdder, (-200.0, 100.0));
		let half_adder_2 = add!(circuit, HalfAdder, (200.0, 100.0));
		let or_gate = add!(circuit, OrGate, (0.0, -125.0));

		let input_offset = circuit.get_components()[half_adder_1].as_ref().get_position().1 +
			circuit.get_components()[half_adder_1].as_ref().get_pin_positions()[1].1;

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

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.4,
			},
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			text: Some(TextInfo {
				text: String::from("Full Adder"),
				size: 50,
			}),

			input1: PinState::Disconnected,
			input2: PinState::Disconnected,
			carry_in: PinState::Disconnected,
		}
	}
}

impl RectangleChip for FullAdder {
	fn get_chip_name(&self) -> String {
		String::from("FullAdder")
	}

	fn get_chip_internals(&self) -> &ChipInternals {
		&self.internals
	}

	fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		&mut self.internals
	}

	fn get_chip_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_chip_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_chip_size(&self) -> (f64, f64) {
		(400.0, 200.0)
	}
	
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

	fn set_mode(&mut self, mode: SimulationMode) {
		match (self.sim_mode, mode) {
			(SimulationMode::HighLevel, SimulationMode::Circuit) => {
				self.internals.circuit.set_pin(0, self.input1);
				self.internals.circuit.set_pin(1, self.input2);
				self.internals.circuit.set_pin(2, self.carry_in);
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				self.input1 = self.internals.circuit.get_pins()[0].get_pin_state(0).unwrap();
				self.input2 = self.internals.circuit.get_pins()[1].get_pin_state(0).unwrap();
				self.carry_in = self.internals.circuit.get_pins()[2].get_pin_state(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
	}

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 | 1 | 2 => Ok(PinState::Disconnected),
			3 => Ok(self.input1.xor(self.input2).xor(self.carry_in)),
			4 => {
				let i1 = self.input1.to_bool();
				let i2 = self.input2.to_bool();
				let c = self.carry_in.to_bool();

				let result = (i1 && (i2 || c)) || (i2 && c);

				Ok(PinState::from_bool(result))
			},
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => { self.input1 = state; Ok(()) },
			1 => { self.input2 = state; Ok(()) },
			2 => { self.carry_in = state; Ok(()) },
			3 | 4 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
	}
}

pub struct Adder {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	size: usize,
	position: (f64, f64),
	text: Option<TextInfo>,

	input1: Vec<PinState>,
	input2: Vec<PinState>,
}

impl Adder {
	pub fn new(pos: (f64, f64), size: usize) -> Self {
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

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: scale,
			},
			sim_mode: SimulationMode::HighLevel,
			size,
			position: pos,
			text: Some(TextInfo {
				text: format!("{}-bit Adder", size),
				size: 50,
			}),

			input1: vec![PinState::Disconnected; size],
			input2: vec![PinState::Disconnected; size],
		}
	}
}

impl RectangleChip for Adder {
	fn get_chip_name(&self) -> String {
		String::from("Adder")
	}

	fn get_chip_internals(&self) -> &ChipInternals {
		&self.internals
	}

	fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		&mut self.internals
	}

	fn get_chip_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_chip_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_chip_size(&self) -> (f64, f64) {
		(400.0, self.size as f64 * 100.0)
	}
	
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

	fn set_mode(&mut self, mode: SimulationMode) {
		match (self.sim_mode, mode) {
			(SimulationMode::HighLevel, SimulationMode::Circuit) => {
				for i in 0..self.size {
					self.internals.circuit.set_pin(i, self.input1[self.size - i - 1]);
					self.internals.circuit.set_pin(self.size + i, self.input2[self.size - i - 1]);
				}
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				for i in 0..self.size {
					self.input1[self.size - i - 1] =
						self.internals.circuit.get_pins()[i]
							.get_pin_state(0).unwrap();
					self.input2[self.size - i - 1] =
						self.internals.circuit.get_pins()[self.size + i]
							.get_pin_state(0).unwrap();
				}
			},
			_ => { },
		}
		
		self.sim_mode = mode;
	}

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		let i1 = states_to_num(&self.input1);
		let i2 = states_to_num(&self.input2);

		// TODO: Fix this for 32-bit adders maybe
		let result = num_to_states(i1 + i2);

		if idx < self.size * 2 {
			Ok(PinState::Disconnected)
		} else if idx >= self.size * 2 && idx < self.size * 3 {
			let i = idx - self.size * 2;
			Ok(*result.get(result.len() - i - 1).unwrap_or(&PinState::Off))
		} else {
			Err(PinError::OutOfRange)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx < self.size {
			self.input1[self.size - idx - 1] = state;
			Ok(())
		} else if idx >= self.size && idx < self.size * 2 {
			self.input2[self.size - (idx - self.size) - 1] = state;
			Ok(())
		} else {
			Err(PinError::OutOfRange)
		}
		
	}

	fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
	}
}
