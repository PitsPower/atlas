use std::f64::consts::PI;

use crate::core::{Component, PinError, PinState};
use crate::graphics::{Drawable, Viewport};

const WIDTH: f64 = 67.0;
const HEIGHT: f64 = 110.0;
const RADIUS: f64 = 11.5;

/// An N-type transistor. This kind of transistor lets a current through when
/// the gate is high.
pub struct NTransistor {
	position: (f64, f64),
	source_state: PinState,
	gate_state: PinState,
}

impl NTransistor {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			source_state: PinState::Disconnected,
			gate_state: PinState::Disconnected,
		}
	}
}

impl Drawable for NTransistor {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: Viewport) {
		ctx.set_line_width(7.0);
		ctx.set_line_cap("square");
		ctx.set_stroke_style(&"#fff".into());

		ctx.begin_path();
		ctx.move_to(WIDTH * 0.5, HEIGHT * 0.5);
		ctx.line_to(-WIDTH * 0.5, HEIGHT * 0.5);
		ctx.line_to(-WIDTH * 0.5, -HEIGHT * 0.5);
		ctx.line_to(WIDTH * 0.5, -HEIGHT * 0.5);
		ctx.stroke();

		ctx.begin_path();
		ctx.move_to(-WIDTH * 0.5 - 15.0, HEIGHT * 0.5);
		ctx.line_to(-WIDTH * 0.5 - 15.0, -HEIGHT * 0.5);
		ctx.stroke();
	}
}

impl Component for NTransistor {
	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![
			(-WIDTH * 0.5 - 15.0, 0.0),
			(WIDTH * 0.5, HEIGHT * 0.5),
			(WIDTH * 0.5, -HEIGHT * 0.5),
		]
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 => Ok(PinState::Disconnected),
			1 => Ok(PinState::Disconnected),
			2 => {
				if self.gate_state == PinState::On {
					Ok(self.source_state)
				} else {
					Ok(PinState::Disconnected)
				}
			},
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => { self.gate_state = state; Ok(()) },
			1 => { self.source_state = state; Ok(()) },
			2 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}

	fn get_position(&self) -> (f64, f64) {
		self.position
	}
}

/// A P-type transistor. This kind of transistor lets a current through when
/// the gate is low.
pub struct PTransistor {
	position: (f64, f64),
	source_state: PinState,
	gate_state: PinState,
}

impl PTransistor {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			source_state: PinState::Disconnected,
			gate_state: PinState::Disconnected,
		}
	}
}

impl Drawable for PTransistor {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: Viewport) {
		ctx.set_line_width(7.0);
		ctx.set_line_cap("square");
		ctx.set_stroke_style(&"#fff".into());

		ctx.begin_path();
		ctx.move_to(WIDTH * 0.5, HEIGHT * 0.5);
		ctx.line_to(-WIDTH * 0.5, HEIGHT * 0.5);
		ctx.line_to(-WIDTH * 0.5, -HEIGHT * 0.5);
		ctx.line_to(WIDTH * 0.5, -HEIGHT * 0.5);
		ctx.stroke();

		ctx.begin_path();
		ctx.move_to(-WIDTH * 0.5 - 15.0, HEIGHT * 0.5);
		ctx.line_to(-WIDTH * 0.5 - 15.0, -HEIGHT * 0.5);
		ctx.stroke();

		ctx.begin_path();
		ctx.arc(-WIDTH * 0.5 - 15.0 - RADIUS - 4.0, 0.0, RADIUS, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
	}
}

impl Component for PTransistor {
	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![
			(-WIDTH * 0.5 - 15.0 - RADIUS * 2.0 - 4.0, 0.0),
			(WIDTH * 0.5, -HEIGHT * 0.5),
			(WIDTH * 0.5, HEIGHT * 0.5),
		]
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		match idx {
			0 => Ok(PinState::Disconnected),
			1 => Ok(PinState::Disconnected),
			2 => {
				if self.gate_state != PinState::On {
					Ok(self.source_state)
				} else {
					Ok(PinState::Disconnected)
				}
			},
			_ => Err(PinError::OutOfRange),
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => { self.gate_state = state; Ok(()) },
			1 => { self.source_state = state; Ok(()) },
			2 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}
	
	fn get_position(&self) -> (f64, f64) {
		self.position
	}
}
