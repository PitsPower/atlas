//! Transistor components.

use std::f64::consts::PI;

use crate::core::{Component, ComponentSimulator, PinError, PinState};
use crate::graphics::{BoundingBox, ComponentDrawer};

const WIDTH: f64 = 67.0;
const HEIGHT: f64 = 110.0;
const RADIUS: f64 = 11.5;

/// Simulates an N-type transistor. This kind of transistor lets a current through when
/// the gate is high.
pub struct NTransistorSimulator {
	source_state: PinState,
	gate_state: PinState,
}

impl NTransistorSimulator {
	/// Returns a new [`NTransistorSimulator`].
	pub fn new() -> Self {
		Self {
			source_state: PinState::Disconnected,
			gate_state: PinState::Disconnected,
		}
	}
}

impl Default for NTransistorSimulator {
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentSimulator for NTransistorSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
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

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => { self.gate_state = state; Ok(()) },
			1 => { self.source_state = state; Ok(()) },
			2 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}
}

/// Draws an N-type transistor.
pub struct NTransistorDrawer;

impl NTransistorDrawer {
	/// Returns a new [`NTransistorDrawer`].
	pub fn new() -> Self {
		Self
	}
}

impl Default for NTransistorDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentDrawer for NTransistorDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, _component: &Component) {
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

/// Simulates an P-type transistor. This kind of transistor lets a current through when
/// the gate is low.
pub struct PTransistorSimulator {
	source_state: PinState,
	gate_state: PinState,
}

impl PTransistorSimulator {
	/// Returns a new [`PTransistorSimulator`].
	pub fn new() -> Self {
		Self {
			source_state: PinState::Disconnected,
			gate_state: PinState::Disconnected,
		}
	}
}

impl Default for PTransistorSimulator {
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentSimulator for PTransistorSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
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

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match idx {
			0 => { self.gate_state = state; Ok(()) },
			1 => { self.source_state = state; Ok(()) },
			2 => Ok(()),
			_ => Err(PinError::OutOfRange),
		}
	}
}

/// Draws an P-type transistor.
pub struct PTransistorDrawer;

impl PTransistorDrawer {
	/// Returns a new [`PTransistorDrawer`].
	pub fn new() -> Self {
		Self
	}
}

impl Default for PTransistorDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentDrawer for PTransistorDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, _component: &Component) {
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
		
		ctx.set_fill_style(&"#000".into());

		ctx.begin_path();
		ctx.arc(-WIDTH * 0.5 - 15.0 - RADIUS - 4.0, 0.0, RADIUS, 0.0, 2.0 * PI).unwrap();
		ctx.fill();
		ctx.stroke();
	}
}
