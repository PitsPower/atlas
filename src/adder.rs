//! Adder components.

use crate::add;
use crate::core::{ChipInternals, Circuit, Junction, Pin, PinError, PinState, RectangleChip, SimulationMode, TextInfo, ExternalPin};
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
	pub fn new(pos: (f64, f64)) -> HalfAdder {
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

		HalfAdder {
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
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		&mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
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
				self.internals.circuit.update_component(&ExternalPin {
					component_idx: 0,
					pin_idx: 0,
				}, self.input1, true);
				self.internals.circuit.update_component(&ExternalPin {
					component_idx: 1,
					pin_idx: 0,
				}, self.input2, true);
			},
			(SimulationMode::Circuit, SimulationMode::HighLevel) => {
				self.input1 = self.internals.circuit.get_components()[0].as_ref().get_pin_state(0).unwrap();
				self.input2 = self.internals.circuit.get_components()[1].as_ref().get_pin_state(0).unwrap();
			},
			_ => { },
		}
		
		self.sim_mode = mode;
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 | 1 => Ok(PinState::Disconnected),
			2 => Ok(PinState::from_binary(self.input1.to_binary() && self.input2.to_binary())),
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
	position: (f64, f64),
	text: Option<TextInfo>,
}

impl FullAdder {
	pub fn new(pos: (f64, f64)) -> FullAdder {
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

		FullAdder {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.4,
			},
			position: pos,
			text: Some(TextInfo {
				text: String::from("Full Adder"),
				size: 50,
			}),
		}
	}
}

impl RectangleChip for FullAdder {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		&mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
		(400.0, 200.0)
    }
	
	fn get_mode(&self) -> SimulationMode {
		SimulationMode::Circuit
	}

    fn set_mode(&mut self, mode: SimulationMode) {
        // todo!()
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		todo!()
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		todo!()
	}

    fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
    }
}

pub struct Adder {
	internals: ChipInternals,
	size: usize,
	position: (f64, f64),
	text: Option<TextInfo>,
}

impl Adder {
	pub fn new(pos: (f64, f64), size: usize) -> Adder {
		let mut circuit = Circuit::new();

		let adders: Vec<_> = (0..size)
			.map(|i| add!(circuit, FullAdder, (100.0, -(i as f64 - (size as f64) * 0.5) * 300.0 - 150.0)))
			.collect();

		let input_group_1: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (-666.0, -600.0 - (i as f64 * 50.0))))
			.collect();

		let input_group_2: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (-666.0, 600.0 + size as f64 * 50.0 - (i as f64 * 50.0))))
			.collect();
			
		let output_group: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (666.0, -(i as f64 - (size as f64) * 0.5) * 50.0 - 25.0)))
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

		Adder {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.3,
			},
			position: pos,
			size,
			text: Some(TextInfo {
				text: format!("{}-bit Adder", size),
				size: 50,
			}),
		}
	}
}

impl RectangleChip for Adder {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		&mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
		(400.0, self.size as f64 * 100.0)
    }
	
	fn get_mode(&self) -> SimulationMode {
		SimulationMode::Circuit
	}

    fn set_mode(&mut self, mode: SimulationMode) {
        // todo!()
    }

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		todo!()
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		todo!()
	}

    fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
    }
}
