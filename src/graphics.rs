//! The graphics subsystem.
//! 
//! Provides data structures for rendering circuits onto a canvas.

use wasm_bindgen::prelude::*;

use crate::core::{ChipInternals, Circuit, Component, ExternalPin, SimulationMode};

/// A thing that can be drawn on the screen.
pub trait Drawable {
	/// Draws an object.
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox);
}

/// A command used to control how a wire looks.
#[derive(Clone, Copy, Debug)]
pub enum WireLayoutCommand {
	/// Moves the wire up or down to be aligned with the end pin.
	AlignHorizontal,
	/// Moves the wire left or right to be aligned with the end pin.
	AlignVertical,
	/// Moves the wire in-between the start and end pins horizontally.
	CenterHorizontal,
	/// Moves the wire in-between the start and end pins vertically.
	CenterVertical,
	/// Moves the wire horizontally.
	MoveHorizontal(f64),
	/// Moves the wire vertically.
	MoveVertical(f64),
	/// Moves the wire horizontally and vertically at the same time.
	Move((f64, f64)),
}

/// A bounding box.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
	/// The position of the bounding box.
	position: (f64, f64),
	/// The width and height of the bounding box.
	size: (f64, f64),
}

impl BoundingBox {
	/// Returns a new bounding box.
	fn new(width: f64, height: f64) -> Self {
		Self {
			position: (0.0, 0.0),
			size: (width, height),
		}
	}

	// Returns the bounding box's position.
	pub fn get_position(&self) -> (f64, f64) {
		self.position
	}

	// Returns the bounding box's size.
	pub fn get_size(&self) -> (f64, f64) {
		self.size
	}

	/// Returns the scale of the bounding box relative to the screen.
	fn scale(&self, ctx: &web_sys::CanvasRenderingContext2d) -> f64 {
		self.size.0 / ctx.canvas().unwrap().width() as f64
	}

	/// Returns a new bounding box that sees the same thing as the old one when the given chip internals
	/// are scaled to full size.
	pub fn transform_in_to_chip(&self, position: (f64, f64), internals: &ChipInternals) -> BoundingBox {
		let mut result = *self;

		let scale = internals.inner_scale;
	
		result.position.0 -= position.0;
		result.position.1 -= position.1;

		result.position.0 /= scale;
		result.position.1 /= scale;
		result.size.0 /= scale;
		result.size.1 /= scale;

		result
	}
	
	/// Returns a new bounding box that sees the same thing as the old one when the given chip internals
	/// are scaled back down to regular size.
	fn transform_out_of_chip(&self, position: (f64, f64), internals: &ChipInternals) -> BoundingBox {
		let mut result = *self;

		let scale = internals.inner_scale;

		result.position.0 *= scale;
		result.position.1 *= scale;
		result.size.0 *= scale;
		result.size.1 *= scale;

		result.position.0 += position.0;
		result.position.1 += position.1;
		
		result
	}
}

#[wasm_bindgen]
impl BoundingBox {
	/// Returns the bounding box's x position.
	pub fn get_x(&self) -> f64 {
		self.position.0
	}

	/// Returns the bounding box's y position.
	pub fn get_y(&self) -> f64 {
		self.position.1
	}
}

/// The renderer. This handles drawing every [`Component`] and [`Wire`] on the screen, as well
/// as handling infinite zoom.
pub struct Renderer {
	/// The canvas context.
	ctx: web_sys::CanvasRenderingContext2d,
	/// The viewport that the user sees through.
	pub viewport: BoundingBox,
	/// If true, nothing will be scaled or translated. Instead, the viewport will be rendered
	/// using a yellow box.
	show_viewport: bool,
	/// If true, external pins will be shown with green circles.
	show_pins: bool,
	/// The stack of [`Chip`] indices that are being zoomed into. Used for infinite zoom.
	chip_stack: Vec<usize>,
}

impl Renderer {
	/// Returns a new renderer.
	pub fn new(ctx: web_sys::CanvasRenderingContext2d) -> Self {
		let width = ctx.canvas().unwrap().width() as f64;
		let height = ctx.canvas().unwrap().height() as f64;

		Self {
			ctx,
			viewport: BoundingBox::new(
				width,
				height,
			),
			show_viewport: false,
			show_pins: false,
			chip_stack: vec![],
		}
	}

	/// Returns the size of the canvas.
	fn get_canvas_size(&self) -> (f64, f64) {
		let width = self.ctx.canvas().unwrap().width() as f64;
		let height = self.ctx.canvas().unwrap().height() as f64;

		(width, height)
	}

	/// Updates the viewport to match the new canvas size.
	pub fn update_size(&mut self) {
		let (width, height) = self.get_canvas_size();

		self.viewport.size = (
			width * self.viewport.scale(&self.ctx),
			height * self.viewport.scale(&self.ctx),
		);
	}

	/// Translates the viewport.
	pub fn pan(&mut self, x_diff: f64, y_diff: f64) {
		self.viewport.position.0 -= x_diff * self.viewport.scale(&self.ctx);
		self.viewport.position.1 -= y_diff * self.viewport.scale(&self.ctx);
	}

	/// Scales the viewport.
	pub fn zoom(&mut self, zoom: f64, cursor_x: f64, cursor_y: f64) {
		let (width, height) = self.get_canvas_size();

		let cursor_vec = (
			cursor_x * self.viewport.size.0 / width - self.viewport.size.0 * 0.5,
			cursor_y * self.viewport.size.1 / height - self.viewport.size.1 * 0.5,
		);

		self.viewport.position.0 += cursor_vec.0 * (1.0 - zoom);
		self.viewport.position.1 += cursor_vec.1 * (1.0 - zoom);

		self.viewport.size.0 *= zoom;
		self.viewport.size.1 *= zoom;
	}

	/// Switches between "yellow box" mode and regular mode.
	pub fn switch_viewport_mode(&mut self) {
		self.show_viewport = !self.show_viewport;
	}

	/// Switches between showing and hiding pins.
	pub fn switch_pin_mode(&mut self) {
		self.show_pins = !self.show_pins;
	}

	/// Returns the [`Circuit`] that is currently being rendered. If the viewport is in a [`Chip`],
	/// the [`Circuit`] in that [`Chip`] will be returned.
	fn get_current_circuit<'a>(&self, circuit: &'a Circuit) -> &'a Circuit {
		let mut result = circuit;

		for index in &self.chip_stack {
			result = &result.get_components()[*index].get_internals().unwrap().circuit;
		}
		
		result
	}

	/// Returns the [`Circuit`] that is currently being rendered as mutable.
	fn get_current_circuit_mut<'a>(&self, circuit: &'a mut Circuit) -> &'a mut Circuit {
		let mut result = circuit;

		for index in &self.chip_stack {
			result = &mut result.get_components_mut()[*index].get_internals_mut().unwrap().circuit;
		}
		
		result
	}

	/// Returns the [`Chip`] that houses the current [`Circuit`], unless the current [`Circuit`]
	/// is at the top level.
	fn get_parent_chip<'a>(&self, circuit: &'a Circuit) -> Option<&'a Box<dyn Component>> {
		if self.chip_stack.len() == 0 {
			return None;
		}

		let mut result = &circuit.get_components()[self.chip_stack[0]];

		for i in 1..self.chip_stack.len() {
			let index = self.chip_stack[i];
			result = &result.get_internals().unwrap().circuit.get_components()[index];
		}
		
		Some(result)
	}

	fn update_sim_modes_with_viewport(&mut self, circuit: &mut Circuit, viewport: BoundingBox) {
		for component in circuit.get_components_mut() {
			if component.are_internals_visible(&viewport) {
				component.set_mode(SimulationMode::Circuit);
				
				let pos = component.get_position();

				match component.get_internals_mut() {
					Some(internals) => {
						let new_viewport = viewport.transform_in_to_chip(
							pos,
							&internals,
						);

						self.update_sim_modes_with_viewport(&mut internals.circuit, new_viewport);
					},
					None => {},
				}
			} else {
				component.set_mode(SimulationMode::HighLevel);
			}
		}
	}

	/// Updates the simulation modes for a given circuit.
	pub fn update_sim_modes(&mut self, root_circuit: &mut Circuit) {
		let circuit = self.get_current_circuit_mut(root_circuit);
		self.update_sim_modes_with_viewport(circuit, self.viewport);
	}

	/// Returns the stack of component indices over a given [`Viewport`].
	fn get_chip_stack_from_viewport(&mut self, circuit: &Circuit, cursor: BoundingBox, viewport: BoundingBox) -> Vec<usize> {
		for (idx, component) in circuit.get_components().iter().enumerate().rev() {
			if component.intersects(&cursor) {
				if !component.are_internals_visible(&viewport) {
					return vec![idx];
				}

				if let Some(internals) = component.get_internals() {
					let new_cursor = cursor.transform_in_to_chip(
						component.get_position(),
						internals,
					);
					let new_viewport = viewport.transform_in_to_chip(
						component.get_position(),
						internals,
					);

					let mut result = self.get_chip_stack_from_viewport(&internals.circuit, new_cursor, new_viewport);
					result.insert(0, idx);

					return result;
				} else {
					return vec![idx];
				}
			}
		}

		vec![]
	}

	/// Returns the stack of component indices over a given position.
	pub fn get_chip_stack_from_pos(&mut self, root_circuit: &Circuit, cursor_x: f64, cursor_y: f64) -> Vec<usize> {
		let circuit = self.get_current_circuit(root_circuit);

		let (width, height) = self.get_canvas_size();

		let cursor_vec = (
			cursor_x * self.viewport.size.0 / width - self.viewport.size.0 * 0.5 + self.viewport.position.0,
			cursor_y * self.viewport.size.1 / height - self.viewport.size.1 * 0.5 + self.viewport.position.1,
		);

		let cursor = BoundingBox {
			position: cursor_vec,
			size: (0.0, 0.0),
		};

		let result = self.get_chip_stack_from_viewport(circuit, cursor, self.viewport);

		let mut final_result = self.chip_stack.clone();
		final_result.extend(result);
		final_result
	}

	/// Returns the new cursor position after descending down the chip stack.
	pub fn get_cursor_from_pos(&mut self, circuit: &Circuit, stack: &[usize], cursor_x: f64, cursor_y: f64) -> BoundingBox {
		let (width, height) = self.get_canvas_size();

		let cursor_vec = (
			cursor_x * self.viewport.size.0 / width - self.viewport.size.0 * 0.5 + self.viewport.position.0,
			cursor_y * self.viewport.size.1 / height - self.viewport.size.1 * 0.5 + self.viewport.position.1,
		);

		let cursor = BoundingBox {
			position: cursor_vec,
			size: (0.0, 0.0),
		};

		if stack.len() == 0 {
			return cursor;
		}
		
		let mut viewport = cursor;
		let mut current_circuit = circuit;

		for i in 0..stack.len() - 1 {
			let idx = stack[i];

			let internals = &current_circuit.get_components()[idx].get_internals().unwrap();

			if i >= self.chip_stack.len() {
				viewport = viewport.transform_in_to_chip(
					current_circuit.get_components()[idx].get_position(),
					internals,
				);
			}

			current_circuit = &internals.circuit;
		}

		viewport
	}
	
	/// Returns the clicked pin.
	pub fn get_clicked_pin(&self, root_circuit: &Circuit, cursor_x: f64, cursor_y: f64) -> Option<ExternalPin> {
		let circuit = self.get_current_circuit(root_circuit);

		let (width, height) = self.get_canvas_size();

		let cursor_vec = (
			cursor_x * self.viewport.size.0 / width - self.viewport.size.0 * 0.5 + self.viewport.position.0,
			cursor_y * self.viewport.size.1 / height - self.viewport.size.1 * 0.5 + self.viewport.position.1,
		);

		for (cidx, component) in circuit.get_components().iter().enumerate() {
			for (pidx, pin_pos) in component.get_pin_positions().iter().enumerate() {
				let con = ExternalPin { component_idx: cidx, pin_idx: pidx };

				if circuit.get_wires().iter().find(|w| w.pin1 == con || w.pin2 == con).is_some() {
					continue;
				}

				let true_pin_pos = (
					component.get_position().0 + pin_pos.0,
					component.get_position().1 + pin_pos.1,
				);

				if (true_pin_pos.0 - cursor_vec.0).powf(2.0) + (true_pin_pos.1 - cursor_vec.1).powf(2.0) <= 100.0 {
					return Some(con);
				}
			}
		}

		None
	}

	/// Renders the given [`Circuit`].
	pub fn render(&mut self, root_circuit: &Circuit, selected_chip_stacks: &Vec<Vec<usize>>, selected_pins: &Vec<ExternalPin>) {
		let ctx = &self.ctx;
		let mut circuit = self.get_current_circuit(root_circuit);

		for i in 0..circuit.get_components().len() {
			if circuit.get_components()[i].contains(&self.viewport) {
				let chip = &circuit.get_components()[i];

				let new_viewport = self.viewport
					.transform_in_to_chip(chip.get_position(), chip.get_internals().unwrap());

				self.chip_stack.push(i);
				self.viewport = new_viewport;
				circuit = self.get_current_circuit(root_circuit);

				break;
			}
		}

		while let Some(parent_chip) = self.get_parent_chip(root_circuit) {
			let new_viewport = self.viewport
				.transform_out_of_chip(parent_chip.get_position(), parent_chip.get_internals().unwrap());

			if !parent_chip.contains(&new_viewport) {
				self.chip_stack.pop();
				self.viewport = new_viewport;
				circuit = self.get_current_circuit(root_circuit);
			} else {
				break;
			}
		}

		ctx.save();

		let width = ctx.canvas().unwrap().width() as f64;
		let height = ctx.canvas().unwrap().height() as f64;

		let scaled_width = self.viewport.size.0;
		let scaled_height = self.viewport.size.1;

		ctx.translate(
			width * 0.5,
			height * 0.5,
		).unwrap();

		if !self.show_viewport {
			ctx.scale(1.0 / self.viewport.scale(&self.ctx), 1.0 / self.viewport.scale(&self.ctx)).unwrap();
			ctx.translate(
				-self.viewport.position.0,
				-self.viewport.position.1,
			).unwrap();
		} else {
			// ctx.scale(0.3, 0.3).unwrap();
		}
		
		circuit.draw(ctx, self.viewport);
		circuit.draw_selection_boxes(ctx, selected_chip_stacks);

		if self.show_pins {
			circuit.draw_pin_highlights(ctx, selected_pins);
		}

		if self.show_viewport {
			ctx.set_line_width(3.0);
			ctx.set_stroke_style(&"#ff0".into());

			ctx.stroke_rect(
				self.viewport.position.0 - scaled_width * 0.5,
				self.viewport.position.1 - scaled_height * 0.5,
				scaled_width,
				scaled_height
			);
		}

		ctx.restore();
	}
}
