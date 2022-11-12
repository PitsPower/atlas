// //! Register components.

// use crate::core::{ChipInternals, Circuit, PinError, PinState, RectangleChip, SimulationMode, TextInfo};

// pub struct Register {
// 	internals: ChipInternals,
// 	sim_mode: SimulationMode,
// 	size: usize,
// 	position: (f64, f64),
// 	text: Option<TextInfo>,
// }

// impl Register {
// 	pub fn new(pos: (f64, f64), size: usize) -> Register {
// 		let mut circuit = Circuit::new();

// 		Register {
// 			internals: ChipInternals {
// 				circuit,
// 				inner_scale: 0.5,
// 			},
// 			sim_mode: SimulationMode::Circuit,
// 			size,
// 			position: pos,
// 			text: Some(TextInfo {
// 				text: format!("{}-bit Register", size),
// 				size: 40,
// 			}),
// 		}
// 	}
// }

// impl RectangleChip for Register {
//     fn get_chip_internals(&self) -> &ChipInternals {
// 		&self.internals
//     }

//     fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
// 		&mut self.internals
//     }

//     fn get_chip_position(&self) -> (f64, f64) {
//         self.position
//     }

// 	fn set_chip_position(&mut self, pos: (f64, f64)) {
// 		self.position = pos;
// 	}

//     fn get_chip_size(&self) -> (f64, f64) {
//         (400.0, 300.0)
//     }

//     fn get_text_info(&self) -> Option<&TextInfo> {
// 		self.text.as_ref()
//     }

//     fn get_mode(&self) -> SimulationMode {
// 		self.sim_mode
//     }

//     fn set_mode(&mut self, mode: SimulationMode) {
// 		// TODO: Let's do this
//     }

//     fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
//         todo!()
//     }

//     fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
//         todo!()
//     }
// }
