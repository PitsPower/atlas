use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::graphics::{Drawable, Viewport, WireLayoutCommand};

pub trait Component: Drawable {
	fn as_chip(&self) -> Option<&Chip> {
		None
	}

	fn get_position(&self) -> (f64, f64);

	fn contains(&self, _viewport: &Viewport) -> bool {
		false
	}

	fn intersects(&self, _viewport: &Viewport) -> bool {
		true
	}
}

struct PinConnection {
	component_idx: usize,
	pin_idx: usize,
}

struct Wire {
	con1: PinConnection,
	con2: PinConnection,
	layout_commands: Vec<WireLayoutCommand>,
}

#[wasm_bindgen]
pub struct Circuit {
	components: Vec<Box<dyn Component>>,
	wires: Vec<Wire>,
}

impl Circuit {
	pub fn new() -> Self {
		Self {
			components: vec![],
			wires: vec![],
		}
	}

	pub fn get_components(&self) -> &Vec<Box<dyn Component>> {
		&self.components
	}

	pub fn add(&mut self, component: Box<dyn Component>) -> usize {
		let idx = self.components.len();
		self.components.push(component);
		idx
	}

	pub fn connect(
		&mut self, (comp1_idx, pin1_idx): (usize, usize),
		(comp2_idx, pin2_idx): (usize, usize),
		wire_commands: Vec<WireLayoutCommand>,
	) {
		self.wires.push(Wire {
			con1: PinConnection { component_idx: comp1_idx, pin_idx: pin1_idx },
			con2: PinConnection { component_idx: comp2_idx, pin_idx: pin2_idx },
			layout_commands: wire_commands,
		});
	}
}

impl Drawable for Circuit {
	fn draw(&self, ctx: &CanvasRenderingContext2d, viewport: Viewport) {
		for wire in &self.wires {
			let con1 = &wire.con1;
			let con2 = &wire.con2;

			let comp1 = &self.components[con1.component_idx];
			let comp2 = &self.components[con2.component_idx];

			let c1 = comp1.get_position();
			let c2 = comp2.get_position();

			let p1 = comp1.get_pin_positions()[con1.pin_idx];
			let p2 = comp2.get_pin_positions()[con2.pin_idx];

			let start = (c1.0 + p1.0, c1.1 + p1.1);
			let end = (c2.0 + p2.0, c2.1 + p2.1);

			ctx.set_line_width(7.0);
			ctx.set_stroke_style(&"#f00".into());

			ctx.begin_path();
			ctx.move_to(start.0, start.1);

			let mut current_pos = start;

			for command in &wire.layout_commands {
				match command {
					WireLayoutCommand::AlignHorizontal => {
						current_pos.1 = end.1;
					},
					WireLayoutCommand::AlignVertical => {
						current_pos.0 = end.0;
					},
					WireLayoutCommand::CenterHorizontal => {
						current_pos.0 = (start.0 + end.0) * 0.5;
					},
					WireLayoutCommand::CenterVertical => {
						current_pos.1 = (start.1 + end.1) * 0.5;
					},
					WireLayoutCommand::MoveHorizontal(amount) => {
						current_pos.0 += amount;
					},
					WireLayoutCommand::MoveVertical(amount) => {
						current_pos.1 += amount;
					},
				}

				ctx.line_to(current_pos.0, current_pos.1);
			}

			ctx.line_to(end.0, end.1);
			ctx.stroke();
		}

		for component in &self.components {
			ctx.save();
			
			let (x, y) = component.get_position();
			ctx.translate(x, y).unwrap();

			component.draw(ctx, viewport);
			ctx.restore();
		}
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![]
	}
}

pub struct Chip {
	pub circuit: Circuit,

	pub position: (f64, f64),
	pub size: (f64, f64),
	pub inner_scale: f64,
}

impl Drawable for Chip {
	fn draw(&self, ctx: &CanvasRenderingContext2d, viewport: Viewport) {
		ctx.set_line_width(10.0);
		
		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		let width = self.size.0;
		let height = self.size.1;

		ctx.stroke_rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill_rect(-width * 0.5, -height * 0.5, width, height);

		let start_ratio: f64 = 0.3;
		let end_ratio: f64 = 0.5;

		let height_ratio = height / viewport.get_size().1;
		// let height_ratio = end_ratio;

		if self.intersects(&viewport) && height_ratio > start_ratio {
			ctx.save();
			ctx.scale(self.inner_scale, self.inner_scale).unwrap();
			self.circuit.draw(ctx, viewport.transform_in_to_chip(self));
			ctx.restore();

			let opacity = ((end_ratio - height_ratio) / (end_ratio - start_ratio)).max(0.0);

			ctx.set_fill_style(&format!("rgba(0,0,0,{})", opacity).into());
			ctx.fill_rect(-width * 0.5, -height * 0.5, width, height);
		}
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![]
	}
}

impl Component for Chip {
	fn as_chip(&self) -> Option<&Chip> {
		Some(&self)
	}

	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn contains(&self, viewport: &Viewport) -> bool {
		let contains_x =
			self.position.0 + self.size.0 * 0.5 >= viewport.get_position().0 + viewport.get_size().0 * 0.5 &&
			self.position.0 - self.size.0 * 0.5 <= viewport.get_position().0 - viewport.get_size().0 * 0.5;

		let contains_y =
			self.position.1 + self.size.1 * 0.5 >= viewport.get_position().1 + viewport.get_size().1 * 0.5 &&
			self.position.1 - self.size.1 * 0.5 <= viewport.get_position().1 - viewport.get_size().1 * 0.5;

		contains_x && contains_y
	}

	fn intersects(&self, viewport: &Viewport) -> bool {
		let intersects_x =
			self.position.0 + self.size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - self.size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + self.size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - self.size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
	}
}