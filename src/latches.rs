//! Latch and flip-flop components.

use crate::add;
use crate::core::{
	ChipInternals, Circuit, Junction, Pin, PinError,
	PinState, RectangleChip, SimulationMode, TextInfo,
};
use crate::gates::{AndGate, NorGate, NotGate};
use crate::graphics::WireLayoutCommand;

pub struct SRLatch {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	text: Option<TextInfo>,

	input_reset: PinState,
	input_set: PinState,
	state: PinState,
}

impl SRLatch {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-250.0, -100.0));
		let input2 = add!(circuit, Pin, (-250.0, 100.0));
		
		let nor1 = add!(circuit, NorGate, (0.0, -100.0));
		let nor2 = add!(circuit, NorGate, (0.0, 100.0));
	
		let junction1 = add!(circuit, Junction, (150.0, -100.0), 3);
		let junction2 = add!(circuit, Junction, (150.0, 100.0), 3);
	
		let output1 = add!(circuit, Pin, (250.0, -100.0));
		let output2 = add!(circuit, Pin, (250.0, 100.0));
	
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
	
		// Order matters here. This order ensures that the latch is off by default.
		circuit.connect((junction2, 2), (nor1, 1), vec![
			WireLayoutCommand::MoveVertical(-35.0),
			WireLayoutCommand::Move((-250.0, -100.0)),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((junction1, 2), (nor2, 0), vec![
			WireLayoutCommand::MoveVertical(35.0),
			WireLayoutCommand::Move((-250.0, 100.0)),
			WireLayoutCommand::AlignHorizontal,
		]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.8,
			},
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			text: Some(TextInfo {
				text: String::from("SR Latch"),
				size: 70,
			}),

			input_reset: PinState::Disconnected,
			input_set: PinState::Disconnected,
			state: PinState::Off,
		}
	}
}

impl RectangleChip for SRLatch {
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
		(400.0, 400.0)
    }
	
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

    fn set_mode(&mut self, mode: SimulationMode) {
		match (self.sim_mode, mode) {
			(SimulationMode::HighLevel, SimulationMode::Circuit) => {
				self.internals.circuit.set_pin(0, self.input_reset);
				self.internals.circuit.set_pin(1, self.input_set);

				if !self.input_reset.to_bool() & !self.input_set.to_bool() {
					if self.state.to_bool() {
						self.internals.circuit.set_pin(1, PinState::On);
						self.internals.circuit.set_pin(1, self.input_set);
					} else {
						self.internals.circuit.set_pin(0, PinState::On);
						self.internals.circuit.set_pin(0, self.input_reset);
					}
				}
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				self.input_reset = self.internals.circuit.get_pins()[0].as_ref().get_pin_state(0).unwrap();
				self.input_set = self.internals.circuit.get_pins()[1].as_ref().get_pin_state(0).unwrap();
				self.state = self.internals.circuit.get_pins()[2].as_ref().get_pin_state_external(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 | 1 => Ok(PinState::Disconnected),
			2 => Ok(self.state),
			3 => Ok(self.state.toggle()),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => {
				self.input_reset = state;
				if self.input_reset.to_bool() {
					self.state = PinState::Off;
				}

				Ok(())
			},
			1 => {
				self.input_set = state;
				if self.input_set.to_bool() {
					self.state = PinState::On;
				}

				Ok(())
			},
			2 | 3 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}

    fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
    }
}

pub struct DLatch {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	text: Option<TextInfo>,

	input: PinState,
	clock: PinState,
	state: PinState,
}

impl DLatch {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let offset = 200.0;

		let latch = add!(circuit, SRLatch, (offset, 0.0));
	
		let latch_offset = circuit.get_components()[latch].as_ref().get_pin_positions()[1].1;
	
		let not = add!(circuit, NotGate, (offset - 570.0, -latch_offset));
		let and1 = add!(circuit, AndGate, (offset - 320.0, -latch_offset));
		let and2 = add!(circuit, AndGate, (offset - 320.0, latch_offset));
	
		let and_offset = circuit.get_components()[and1].as_ref().get_pin_positions()[1].1;
	
		let input = add!(circuit, Pin, (offset - 800.0, -200.0));
		let clock = add!(circuit, Pin, (offset - 800.0, 200.0));
	
		let input_junc = add!(circuit, Junction, (offset - 680.0, -latch_offset), 3);
		let clock_junc = add!(circuit, Junction, (offset - 420.0, latch_offset + and_offset), 3);
	
		let output1 = add!(circuit, Pin, (offset + 400.0, -200.0));
		let output2 = add!(circuit, Pin, (offset + 400.0, 200.0));
	
		circuit.connect((input, 0), (input_junc, 0), vec![
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((input_junc, 1), (not, 0), vec![]);
		circuit.connect((not, 1), (and1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input_junc, 2), (and2, 0), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
	
		circuit.connect((clock, 0), (clock_junc, 0), vec![
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((clock_junc, 1), (and1, 1), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock_junc, 2), (and2, 1), vec![]);
		
		circuit.connect((and1, 2), (latch, 0), vec![]);
		circuit.connect((and2, 2), (latch, 1), vec![]);
		
		circuit.connect((latch, 2), (output1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((latch, 3), (output2, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.5,
			},
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			text: Some(TextInfo {
				text: String::from("D Latch"),
				size: 70,
			}),

			input: PinState::Disconnected,
			clock: PinState::Disconnected,
			state: PinState::Off,
		}
	}
}

impl RectangleChip for DLatch {
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
		(600.0, 400.0)
    }
	
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

    fn set_mode(&mut self, mode: SimulationMode) {
		match (self.sim_mode, mode) {
			(SimulationMode::HighLevel, SimulationMode::Circuit) => {
				self.internals.circuit.set_pin(0, self.state);
				self.internals.circuit.set_pin(1, PinState::On);
				self.internals.circuit.set_pin(1, self.clock);
				self.internals.circuit.set_pin(0, self.input);
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				self.input = self.internals.circuit.get_pins()[0].as_ref().get_pin_state(0).unwrap();
				self.clock = self.internals.circuit.get_pins()[1].as_ref().get_pin_state(0).unwrap();
				self.state = self.internals.circuit.get_pins()[2].as_ref().get_pin_state_external(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 | 1 => Ok(PinState::Disconnected),
			2 => Ok(self.state),
			3 => Ok(self.state.toggle()),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => {
				self.input = state;
				if self.clock.to_bool() {
					self.state = self.input;
				}

				Ok(())
			},
			1 => {
				self.clock = state;
				if self.clock.to_bool() {
					self.state = self.input;
				}

				Ok(())
			},
			2 | 3 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}

    fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
    }
}

pub struct DFlipFlop {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	text: Option<TextInfo>,

	input: PinState,
	clock: PinState,
	state: PinState,
}

impl DFlipFlop {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();
		
		let offset = 100.0;
		
		let latch1 = add!(circuit, DLatch, (offset - 400.0, 0.0));
		let latch2 = add!(circuit, DLatch, (offset + 400.0, 0.0));

		let offset = circuit.get_components()[latch1].as_ref().get_pin_positions()[1].1;

		let input = add!(circuit, Pin, (offset - 1100.0, -300.0));
		let clock = add!(circuit, Pin, (offset - 1100.0, 300.0));

		let clock_junc = add!(circuit, Junction, (offset - 965.0, 300.0), 3);

		let not = add!(circuit, NotGate, (offset - 840.0, offset));

		let output1 = add!(circuit, Pin, (offset + 900.0, -300.0));
		let output2 = add!(circuit, Pin, (offset + 900.0, 300.0));

		circuit.connect((input, 0), (latch1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock, 0), (clock_junc, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		
		circuit.connect((clock_junc, 1), (latch2, 1), vec![
			WireLayoutCommand::MoveHorizontal(960.0),
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((clock_junc, 2), (not, 0), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((not, 1), (latch1, 1), vec![]);

		circuit.connect((latch1, 2), (latch2, 0), vec![]);
		
		circuit.connect((latch2, 2), (output1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((latch2, 3), (output2, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.3,
			},
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			text: Some(TextInfo {
				text: String::from("D Flip-Flop"),
				size: 70,
			}),

			input: PinState::Disconnected,
			clock: PinState::Disconnected,
			state: PinState::Off,
		}
	}
}

impl RectangleChip for DFlipFlop {
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
		(600.0, 400.0)
    }
	
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

    fn set_mode(&mut self, mode: SimulationMode) {
		match (self.sim_mode, mode) {
			(SimulationMode::HighLevel, SimulationMode::Circuit) => {
				self.internals.circuit.set_pin(0, self.state);
				self.internals.circuit.set_pin(1, PinState::Off);
				self.internals.circuit.set_pin(1, PinState::On);

				self.internals.circuit.set_pin(1, self.clock);
				self.internals.circuit.set_pin(0, self.input);
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				self.input = self.internals.circuit.get_pins()[0].as_ref().get_pin_state(0).unwrap();
				self.clock = self.internals.circuit.get_pins()[1].as_ref().get_pin_state(0).unwrap();
				self.state = self.internals.circuit.get_pins()[2].as_ref().get_pin_state_external(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 | 1 => Ok(PinState::Disconnected),
			2 => Ok(self.state),
			3 => Ok(self.state.toggle()),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => {
				self.input = state;
				Ok(())
			},
			1 => {
				if state.to_bool() && !self.clock.to_bool() {
					self.state = self.input;
				}

				self.clock = state;
				Ok(())
			},
			2 | 3 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}

    fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
    }
}

pub struct MultiDFlipFlop {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	size: usize,
	text: Option<TextInfo>,

	input: Vec<PinState>,
	clock: PinState,
	state: Vec<PinState>,
}

impl MultiDFlipFlop {
	pub fn new(pos: (f64, f64), size: usize) -> Self {
		let mut circuit = Circuit::new();

		let scale = 0.19;
		let width = 400.0;
		let height = size as f64 * 100.0;
		let wire_spacing = 90.0;

		let flip_flops: Vec<_> = (0..size)
			.map(|i| add!(circuit, DFlipFlop, (0.0, -(i as f64 - (size as f64) * 0.5) * 500.0 - 250.0)))
			.collect();

		let offset = circuit.get_components()[flip_flops[0]].as_ref().get_pin_positions()[1].1;

		let input_group: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (-width * 0.5 / scale, -((i as f64 - (size as f64) * 0.5 + 0.5) * wire_spacing))))
			.collect();

		let clock = add!(circuit, Pin, (0.0, height * 0.5 / scale));

		let clock_juncs: Vec<_> = (0..size - 1)
		.map(|i| add!(circuit, Junction, (-450.0, offset - (i as f64 - (size as f64) * 0.5) * 500.0 - 250.0), 3))
		.collect();

		let output_group: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (width * 0.5 / scale, -((i as f64 - (size as f64) * 0.5 + 0.5) * wire_spacing))))
			.collect();

		circuit.connect((clock_juncs[size-2], 1), (flip_flops[size-1], 1), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock, 0), (clock_juncs[0], 0), vec![
			WireLayoutCommand::MoveVertical(-70.0),
			WireLayoutCommand::AlignVertical,
		]);

		for i in 0..size {
			circuit.connect((input_group[i], 0), (flip_flops[i], 0), vec![
				WireLayoutCommand::CenterHorizontal,
				WireLayoutCommand::MoveHorizontal((if i < size/2 { i } else { size - i - 1 }) as f64 * 30.0),
				WireLayoutCommand::AlignHorizontal,
			]);
			circuit.connect((flip_flops[i], 2), (output_group[i], 0), vec![
				WireLayoutCommand::MoveHorizontal(150.0),
				WireLayoutCommand::MoveHorizontal((if i < size/2 { size - i - 1 } else { i }) as f64 * 30.0),
				WireLayoutCommand::AlignHorizontal,
			]);
		}

		for i in 0..size-1 {
			circuit.connect((clock_juncs[i], 2), (flip_flops[i], 1), vec![]);
		}
		for i in 0..size-2 {
			circuit.connect((clock_juncs[i], 1), (clock_juncs[i+1], 0), vec![]);
		}

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.19,
			},
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			size,
			text: Some(TextInfo {
				text: format!("{}-bit D Flip-Flop", size),
				size: 40,
			}),
			
			input: vec![PinState::Disconnected; size],
			clock: PinState::Disconnected,
			state: vec![PinState::Off; size],
		}
	}
}

impl RectangleChip for MultiDFlipFlop {
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
					self.internals.circuit.set_pin(i, self.state[i]);
				}

				self.internals.circuit.set_pin(self.size, PinState::Off);
				self.internals.circuit.set_pin(self.size, PinState::On);
				self.internals.circuit.set_pin(self.size, self.clock);

				for i in 0..self.size {
					self.internals.circuit.set_pin(i, self.input[i]);
				}
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				for i in 0..self.size {
					self.input[i] = self.internals.circuit.get_pins()[i].as_ref().get_pin_state(0).unwrap();
					self.state[i] = self.internals.circuit.get_pins()[self.size + 1 + i].as_ref()
						.get_pin_state_external(0).unwrap();
				}
				self.clock = self.internals.circuit.get_pins()[self.size].as_ref().get_pin_state(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx <= self.size {
			Ok(PinState::Disconnected)
		} else if idx > self.size && idx <= self.size * 2 {
			Ok(self.state[idx - self.size - 1])
		} else {
			Err(PinError::OutOfRange)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx < self.size {
			self.input[idx] = state;
			Ok(())
		} else if idx == self.size {
			if state.to_bool() && !self.clock.to_bool() {
				for i in 0..self.size {
					self.state[i] = self.input[i];
				}
			}

			self.clock = state;
			Ok(())
		} else if idx > self.size && idx <= self.size * 2 {
			Ok(())
		} else {
			Err(PinError::OutOfRange)
		}
	}

    fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
    }
}
