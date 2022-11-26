//! Multiplexer components.

use crate::add;
use crate::core::{
	ChipInternals, Circuit, ExternalPin, Junction, Pin, PinError,
	PinState, RectangleChip, SimulationMode, TextInfo,
};
use crate::gates::{AndGate, NotGate, OrGate};
use crate::graphics::WireLayoutCommand;

pub struct Multiplexer {
	internals: ChipInternals,
	sim_mode: SimulationMode,
	position: (f64, f64),
	chip_size: (f64, f64),
	text: Option<TextInfo>,

	input1: PinState,
	input2: PinState,
	selector: PinState,
}

impl Multiplexer {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let c0 = add!(circuit, Pin, (0.000, 450.000));
		let c1 = add!(circuit, Pin, (-350.000, 150.000));
		let c2 = add!(circuit, Pin, (-350.000, -151.670));
		let c3 = add!(circuit, Pin, (350.000, 0.000));
		let c4 = add!(circuit, AndGate, (-50.000, 100.000));
		let c5 = add!(circuit, AndGate, (-50.000, -350.000));
		let c6 = add!(circuit, OrGate, (150.000, 0.000));
		let c7 = add!(circuit, Junction, (0.000, 350.000), 3);
		let c8 = add!(circuit, NotGate, (150.000, 350.000));
		
		circuit.connect((c6, 2), (c3, 0), vec![]);
		circuit.connect((c1, 0), (c4, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c2, 0), (c5, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c4, 2), (c6, 1), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c5, 2), (c6, 0), vec![WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal, WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c0, 0), (c7, 0), vec![]);
		circuit.connect((c7, 1), (c8, 0), vec![]);
		circuit.connect((c7, 2), (c4, 1), vec![WireLayoutCommand::MoveTo((-150.000, 350.000)), WireLayoutCommand::AlignHorizontal]);
		circuit.connect((c8, 1), (c5, 1), vec![WireLayoutCommand::MoveTo((250.000, 350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((250.000, -200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-150.000, -200.000)), WireLayoutCommand::AlignHorizontal]);
		
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
			sim_mode: SimulationMode::HighLevel,
			position: pos,
			chip_size,
			text: Some(TextInfo {
				text: String::from("Multiplexer"),
				size: 27,
			}),

			input1: PinState::Disconnected,
			input2: PinState::Disconnected,
			selector: PinState::Disconnected,
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
