use std::f64::consts::PI;

use crate::core::{Component, PinState, GetPinError, SetPinError};
use crate::graphics::{Drawable, Viewport};

const WIDTH: f64 = 67.0;
const HEIGHT: f64 = 110.0;
const RADIUS: f64 = 11.5;

pub struct NTransistor {
	position: (f64, f64),
}

impl NTransistor {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
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

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![
			(-WIDTH * 0.5 - 15.0, 0.0),
			(WIDTH * 0.5, HEIGHT * 0.5),
			(WIDTH * 0.5, -HEIGHT * 0.5),
		]
	}
}

impl Component for NTransistor {
	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError> {
		todo!();
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		todo!();
	}
	
	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		todo!();
	}

	fn get_position(&self) -> (f64, f64) {
		self.position
	}
}

pub struct PTransistor {
	position: (f64, f64),
}

impl PTransistor {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
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

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![
			(-WIDTH * 0.5 - 15.0 - RADIUS * 2.0 - 4.0, 0.0),
			(WIDTH * 0.5, HEIGHT * 0.5),
			(WIDTH * 0.5, -HEIGHT * 0.5),
		]
	}
}

impl Component for PTransistor {
	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError> {
		todo!();
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		todo!();
	}

	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		todo!();
	}
	
	fn get_position(&self) -> (f64, f64) {
		self.position
	}
}
