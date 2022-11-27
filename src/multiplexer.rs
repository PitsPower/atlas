//! Multiplexer components.

use crate::add;
use crate::core::{
	ChipInternals, Circuit, Junction, Pin, PinError,
	PinState, RectangleChip, SimulationMode, TextInfo,
};
use crate::gates::{AndGate, NotGate, OrGate};
use crate::graphics::WireLayoutCommand;

pub struct Multiplexer {
	internals: ChipInternals,
	position: (f64, f64),
	chip_size: (f64, f64),
	text: Option<TextInfo>,
}

impl Multiplexer {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let c0 = add!(circuit, AndGate, (0.000, -100.000));
		let c1 = add!(circuit, AndGate, (0.000, 100.000));
		let c2 = add!(circuit, NotGate, (-200.000, 0.000));
		let c3 = add!(circuit, Junction, (-300.000, 100.000), 3);
		let c4 = add!(circuit, Pin, (-400.000, -200.000));
		let c5 = add!(circuit, Pin, (-400.000, 200.000));
		let c6 = add!(circuit, Pin, (400.000, 0.000));
		let c7 = add!(circuit, OrGate, (200.000, 0.000));
		let c8 = add!(circuit, Pin, (0.000, 350.000));
		
		circuit.connect((c2, 1), (c0, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c1, 0), (c3, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c3, 1), (c2, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c5, 0), (c1, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c4, 0), (c0, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c0, 2), (c7, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c1, 2), (c7, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c7, 2), (c6, 0), vec![]);
		circuit.connect((c8, 0), (c3, 2), vec![WireLayoutCommand::MoveTo((0.000, 231.313)), WireLayoutCommand::AlignVertical]);
		
		let inner_scale = 0.4;

		let mut min_x = 9999999.9;
		let mut min_y = 9999999.9;
		let mut max_x = -9999999.9;
		let mut max_y = -9999999.9;

		for component in circuit.get_components() {
			if component.is_pin() {
				let pos = component.get_position();
				
				if min_x > pos.0 {
					min_x = pos.0;
				}
				if min_y > pos.1 {
					min_y = pos.1;
				}
				if max_x < pos.0 {
					max_x = pos.0;
				}
				if max_y < pos.1 {
					max_y = pos.1;
				}
			}
		}

		let mut min_side_y = 9999999.9;
		let mut max_side_y = -9999999.9;

		for component in circuit.get_components() {
			if component.is_pin() {
				let pos = component.get_position();

				if pos.0 == min_x || pos.0 == max_x {
					if min_side_y > pos.1 {
						min_side_y = pos.1;
					}
					if max_side_y < pos.1 {
						max_side_y = pos.1;
					}
				}
			}
		}

		let top_pad = min_y - min_side_y;
		let bottom_pad = max_y - max_side_y;

		let mut chip_size = (
			(max_x - min_x) * inner_scale,
			(max_y - min_y) * inner_scale,
		);

		let mut circuit_offset = (
			-(max_x - min_x) * 0.5 - min_x,
			-(max_y - min_y) * 0.5 - min_y,
		);

		if bottom_pad > top_pad {
			let pad_diff = bottom_pad - top_pad;
			chip_size.1 += pad_diff * inner_scale;
			circuit_offset.1 += pad_diff * 0.5;
		}

		for component in circuit.get_components_mut() {
			component.translate((circuit_offset.0, circuit_offset.1));
		}

		for wire in circuit.get_wires_mut() {
			for i in 0..wire.layout_commands.len() {
				match wire.layout_commands[i] {
					WireLayoutCommand::MoveXTo(x) => {
						wire.layout_commands[i] = WireLayoutCommand::MoveXTo(x + circuit_offset.0);
					},
					WireLayoutCommand::MoveYTo(y) => {
						wire.layout_commands[i] = WireLayoutCommand::MoveYTo(y + circuit_offset.1);
					},
					WireLayoutCommand::MoveTo((x, y)) => {
						wire.layout_commands[i] = WireLayoutCommand::MoveTo((
							x + circuit_offset.0,
							y + circuit_offset.1,
						));
					},
					_ => {},
				};
			}
		}

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale,
			},
			position: pos,
			chip_size,
			text: Some(TextInfo {
				text: String::from("Multiplexer"),
				size: 27,
			}),
		}
	}
}

impl RectangleChip for Multiplexer {
    fn get_chip_name(&self) -> String {
		String::from("Multiplexer")
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
		self.chip_size
    }

	fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
	}

    fn get_mode(&self) -> SimulationMode {
		SimulationMode::Circuit
    }

    fn set_mode(&mut self, mode: SimulationMode) {
		
    }

    fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
        todo!()
    }

    fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
        todo!()
    }
}

pub struct TwoBitMultiplexer {
	internals: ChipInternals,
	position: (f64, f64),
	chip_size: (f64, f64),
	text: Option<TextInfo>,
}

impl TwoBitMultiplexer {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let c0 = add!(circuit, Multiplexer, (-250.000, -200.000));
		let c1 = add!(circuit, Multiplexer, (-250.000, 200.000));
		let c2 = add!(circuit, Multiplexer, (250.000, 0.000));
		let c3 = add!(circuit, Pin, (-550.000, -100.000));
		let c4 = add!(circuit, Pin, (-550.000, 100.000));
		let c5 = add!(circuit, Pin, (-550.000, -300.000));
		let c6 = add!(circuit, Pin, (-550.000, 300.000));
		let c7 = add!(circuit, Pin, (550.000, 0.000));
		let c8 = add!(circuit, Pin, (100.000, 500.000));
		let c9 = add!(circuit, Pin, (-100.000, 500.000));
		let c10 = add!(circuit, Junction, (50.000, 400.000), 3);
		
		circuit.connect((c0, 2), (c2, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c1, 2), (c2, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c5, 0), (c0, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c3, 0), (c0, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c4, 0), (c1, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c6, 0), (c1, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c2, 2), (c7, 0), vec![]);
		circuit.connect((c8, 0), (c10, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c10, 1), (c1, 3), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((c10, 2), (c0, 3), vec![WireLayoutCommand::MoveTo((50.000, 0.000)), WireLayoutCommand::AlignVertical]);
		circuit.connect((c9, 0), (c2, 3), vec![WireLayoutCommand::MoveTo((-100.000, 450.000)), WireLayoutCommand::AlignVertical]);
		
		let inner_scale = 0.4;

		let mut min_x = f64::INFINITY;
		let mut min_y = f64::INFINITY;
		let mut max_x = f64::NEG_INFINITY;
		let mut max_y = f64::NEG_INFINITY;

		for component in circuit.get_components() {
			if component.is_pin() {
				let pos = component.get_position();
				
				if min_x > pos.0 {
					min_x = pos.0;
				}
				if min_y > pos.1 {
					min_y = pos.1;
				}
				if max_x < pos.0 {
					max_x = pos.0;
				}
				if max_y < pos.1 {
					max_y = pos.1;
				}
			}
		}

		let mut min_side_y = f64::INFINITY;
		let mut max_side_y = f64::NEG_INFINITY;

		for component in circuit.get_components() {
			if component.is_pin() {
				let pos = component.get_position();

				if pos.0 == min_x || pos.0 == max_x {
					if min_side_y > pos.1 {
						min_side_y = pos.1;
					}
					if max_side_y < pos.1 {
						max_side_y = pos.1;
					}
				}
			}
		}

		let top_pad = min_y - min_side_y;
		let bottom_pad = max_y - max_side_y;

		let mut chip_size = (
			(max_x - min_x) * inner_scale,
			(max_y - min_y) * inner_scale,
		);

		let mut circuit_offset = (
			-(max_x - min_x) * 0.5 - min_x,
			-(max_y - min_y) * 0.5 - min_y,
		);

		if bottom_pad > top_pad {
			let pad_diff = bottom_pad - top_pad;
			chip_size.1 += pad_diff * inner_scale;
			circuit_offset.1 += pad_diff * 0.5;
		}

		for component in circuit.get_components_mut() {
			component.translate((circuit_offset.0, circuit_offset.1));
		}

		for wire in circuit.get_wires_mut() {
			for i in 0..wire.layout_commands.len() {
				match wire.layout_commands[i] {
					WireLayoutCommand::MoveXTo(x) => {
						wire.layout_commands[i] = WireLayoutCommand::MoveXTo(x + circuit_offset.0);
					},
					WireLayoutCommand::MoveYTo(y) => {
						wire.layout_commands[i] = WireLayoutCommand::MoveYTo(y + circuit_offset.1);
					},
					WireLayoutCommand::MoveTo((x, y)) => {
						wire.layout_commands[i] = WireLayoutCommand::MoveTo((
							x + circuit_offset.0,
							y + circuit_offset.1,
						));
					},
					_ => {},
				};
			}
		}

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale,
			},
			position: pos,
			chip_size,
			text: Some(TextInfo {
				text: String::from("2-bit Multiplexer"),
				size: 27,
			}),
		}
	}
}

impl RectangleChip for TwoBitMultiplexer {
    fn get_chip_name(&self) -> String {
		String::from("TwoBitMultiplexer")
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
		self.chip_size
    }

	fn get_text_info(&self) -> Option<&TextInfo> {
		self.text.as_ref()
	}

    fn get_mode(&self) -> SimulationMode {
		SimulationMode::Circuit
    }

    fn set_mode(&mut self, mode: SimulationMode) {
		
    }

    fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
        todo!()
    }

    fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
        todo!()
    }
}
