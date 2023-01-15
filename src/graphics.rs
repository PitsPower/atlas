//! The graphics subsystem.
//! 
//! Provides data structures for rendering circuits onto a canvas.

use wasm_bindgen::prelude::*;

use crate::core::{ComponentInternals, Circuit, Component, ExternalPin, SimulationMode};

/// A thing that can be drawn on the screen.
pub trait Drawable {
	/// Draws an object.
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox);
}

/// A thing that can draw a component.
pub trait ComponentDrawer {
	/// Draws a component.
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox, component: &Component);
}

/// A thing that can draw a chip.
pub trait ChipDrawer {
	/// Draws the front of the chip.
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &Component);

	/// Draws the edge of the chip.
	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &Component);

	/// Draws the back of the chip.
	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &Component);
}

impl<T: ChipDrawer> ComponentDrawer for T {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox, component: &Component) {
		self.draw_back(ctx, component);

		// TODO: Probably merge this with the other implementation!
		let start_ratio = 0.3;
		let end_ratio = 0.5;

		// let start_ratio = 0.0;
		// let end_ratio = 0.0;

		let height = component.size.1;
		let height_ratio = height / viewport.get_size().1;

		if let ComponentInternals::Chip(_, inner_scale) = component.internals {
			if component.intersects(&viewport) && height_ratio > start_ratio {
				let new_viewport = viewport.transform_in_to_chip(
					component.position,
					inner_scale,
				);
	
				component.internals.draw(ctx, new_viewport);
			}
		}
		
		let opacity = ((end_ratio - height_ratio) / (end_ratio - start_ratio)).max(0.0);
		ctx.set_global_alpha(opacity);
		
		self.draw_front(ctx, component);
		
		ctx.set_global_alpha(1.0);
		self.draw_edge(ctx, component);
	}
}

/// Information about text to be rendered on the front of a chip.
pub struct TextInfo {
	/// The text.
	pub text: String,
	/// The size of the text.
	pub size: u32,
}

/// A thing that can draw a rectangular chip.
pub struct RectangleChipDrawer {
	text_info: TextInfo,
}

impl RectangleChipDrawer {
	pub fn new(text_info: TextInfo) -> Self {
		Self {
			text_info,
		}
	}
}

impl ChipDrawer for RectangleChipDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill();

		ctx.set_fill_style(&"#fff".into());
		ctx.set_font(format!("bold {}px monospace", self.text_info.size).as_str());
		ctx.set_text_align("center");
		ctx.set_text_baseline("middle");

		ctx.fill_text(self.text_info.text.as_str(), 0.0, 0.0).unwrap();
	}

	fn draw_edge(&self, _ctx: &web_sys::CanvasRenderingContext2d, _component: &Component) {
		
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &Component) {
		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());

		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.rect(-width * 0.5, -height * 0.5, width, height);

		ctx.stroke();
		ctx.fill();
	}
}

/// A [`ComponentDrawer`] that doesn't draw anything.
pub struct NothingDrawer;

impl NothingDrawer {
	/// Returns a new [`NothingDrawer`].
	pub fn new() -> Self {
		Self
	}
}

impl Default for NothingDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentDrawer for NothingDrawer {
	fn draw(&self, _ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox,_componentt: &Component) {
		// Nothing at all...
	}
}

/// A command used to control how a wire looks.
#[derive(Clone, Copy, Debug, PartialEq)]
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
	/// Moves the x coordinate to an absolute location.
	MoveXTo(f64),
	/// Moves the y coordinate to an absolute location.
	MoveYTo(f64),
	/// Moves the wire to an absolute location.
	MoveTo((f64, f64)),
	/// Tells the renderer to not draw the last command, but to still change the position.
	DontRenderPrevious,
	/// Tells the renderer to render the vertical change, but not the horizontal change.
	DontRenderPreviousHorizontal,
	/// Tells the renderer to render the horizontal change, but not the vertical change.
	DontRenderPreviousVertical,
}

impl WireLayoutCommand {
	/// Returns the layout command as a string.
	pub fn as_string(self) -> String {
		match self {
			WireLayoutCommand::AlignHorizontal => String::from("WireLayoutCommand::AlignHorizontal"),
			WireLayoutCommand::AlignVertical => String::from("WireLayoutCommand::AlignVertical"),
			WireLayoutCommand::CenterHorizontal => String::from("WireLayoutCommand::CenterHorizontal"),
			WireLayoutCommand::CenterVertical => String::from("WireLayoutCommand::CenterVertical"),
			WireLayoutCommand::MoveHorizontal(x) => format!("WireLayoutCommand::MoveHorizontal({:.3})", x),
			WireLayoutCommand::MoveVertical(y) => format!("WireLayoutCommand::MoveVertical({:.3})", y),
			WireLayoutCommand::Move((x, y)) => format!("WireLayoutCommand::Move(({:.3}, {:.3}))", x, y),
			WireLayoutCommand::MoveXTo(x) => format!("WireLayoutCommand::MoveXTo({:.3})", x),
			WireLayoutCommand::MoveYTo(y) => format!("WireLayoutCommand::MoveYTo({:.3})", y),
			WireLayoutCommand::MoveTo((x, y)) => format!("WireLayoutCommand::MoveTo(({:.3}, {:.3}))", x, y),
			WireLayoutCommand::DontRenderPrevious => String::from("WireLayoutCommand::DontRenderPrevious"),
			WireLayoutCommand::DontRenderPreviousHorizontal => String::from("WireLayoutCommand::DontRenderPreviousHorizontal"),
			WireLayoutCommand::DontRenderPreviousVertical => String::from("WireLayoutCommand::DontRenderPreviousVertical"),
		}
	}
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
	pub fn new(width: f64, height: f64) -> Self {
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
	pub fn transform_in_to_chip(&self, position: (f64, f64), scale: f64) -> BoundingBox {
		let mut result = *self;
	
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
	fn transform_out_of_chip(&self, position: (f64, f64), scale: f64) -> BoundingBox {
		let mut result = *self;

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


/// Returns the stack of component indices over a given [`Viewport`].
fn get_chip_stack_from_viewport(circuit: &Circuit, cursor: BoundingBox, viewport: BoundingBox) -> Vec<usize> {
	for (idx, component) in circuit.components.iter().enumerate().rev() {
		if component.intersects(&cursor) {
			if !component.are_internals_visible(&viewport) {
				return vec![idx];
			}

			if let ComponentInternals::Chip(circuit, inner_scale) = &component.internals {
				let new_cursor = cursor.transform_in_to_chip(
					component.position,
					*inner_scale,
				);
				let new_viewport = viewport.transform_in_to_chip(
					component.position,
					*inner_scale,
				);

				let mut result = get_chip_stack_from_viewport(circuit.get(), new_cursor, new_viewport);
				result.insert(0, idx);

				return result;
			} else {
				return vec![idx];
			}
		}
	}

	vec![]
}

/// Updates the simulation modes for a given circuit and a given viewport.
pub fn update_sim_modes_with_viewport(circuit: &mut Circuit, viewport: BoundingBox) {
	for component in &mut circuit.components {
		if component.are_internals_visible(&viewport) {
			component.set_mode(SimulationMode::Circuit);
			
			let pos = component.position;

			if let ComponentInternals::Chip(circuit, inner_scale) = &mut component.internals {
				let new_viewport = viewport.transform_in_to_chip(
					pos,
					*inner_scale,
				);

				update_sim_modes_with_viewport(circuit.get_mut(), new_viewport);
			}
		} else {
			component.set_mode(SimulationMode::HighLevel);
		}
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
	/// The stack of chip indices that are being zoomed into. Used for infinite zoom.
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
			result = result.components[*index].internals.get_circuit().unwrap();
		}
		
		result
	}

	/// Returns the [`Circuit`] that is currently being rendered as mutable.
	fn get_current_circuit_mut<'a>(&self, circuit: &'a mut Circuit) -> &'a mut Circuit {
		let mut result = circuit;

		for index in &self.chip_stack {
			result = result.components[*index].internals.get_circuit_mut().unwrap();
		}
		
		result
	}

	/// Returns the [`Chip`] that houses the current [`Circuit`], unless the current [`Circuit`]
	/// is at the top level.
	fn get_parent_chip<'a>(&self, circuit: &'a Circuit) -> Option<&'a Component> {
		if self.chip_stack.is_empty() {
			return None;
		}

		let mut result = &circuit.components[self.chip_stack[0]];

		for i in 1..self.chip_stack.len() {
			let index = self.chip_stack[i];
			result = &result.internals.get_circuit().unwrap().components[index];
		}
		
		Some(result)
	}

	/// Updates the simulation modes for a given circuit.
	pub fn update_sim_modes(&mut self, root_circuit: &mut Circuit) {
		let circuit = self.get_current_circuit_mut(root_circuit);
		update_sim_modes_with_viewport(circuit, self.viewport);
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

		let result = get_chip_stack_from_viewport(circuit, cursor, self.viewport);

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

		if stack.is_empty() {
			return cursor;
		}
		
		let mut viewport = cursor;
		let mut current_circuit = circuit;

		for (i, idx) in stack.iter().enumerate().take(stack.len() - 1) {
			let circuit = &current_circuit.components[*idx].internals.get_circuit().unwrap();
			let inner_scale = current_circuit.components[*idx].internals.get_inner_scale().unwrap();

			if i >= self.chip_stack.len() {
				viewport = viewport.transform_in_to_chip(
					current_circuit.components[*idx].position,
					inner_scale,
				);
			}

			current_circuit = circuit;
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

		for (cidx, component) in circuit.components.iter().enumerate() {
			for (pidx, pin_pos) in component.get_pin_positions().iter().enumerate() {
				let con = ExternalPin { component_idx: cidx, pin_idx: pidx };

				if circuit.wires.iter().any(|w| w.pin1 == con || w.pin2 == con) {
					continue;
				}

				let true_pin_pos = (
					component.position.0 + pin_pos.0,
					component.position.1 + pin_pos.1,
				);

				if (true_pin_pos.0 - cursor_vec.0).powf(2.0) + (true_pin_pos.1 - cursor_vec.1).powf(2.0) <= 100.0 {
					return Some(con);
				}
			}
		}

		None
	}

	/// Renders the given [`Circuit`].
	pub fn render(&mut self, root_circuit: &Circuit, selected_chip_stacks: &[Vec<usize>], selected_pins: &[ExternalPin]) {
		let ctx = &self.ctx;
		let mut circuit = self.get_current_circuit(root_circuit);

		for i in 0..circuit.components.len() {
			if circuit.components[i].contains(&self.viewport) {
				let chip = &circuit.components[i];

				if let ComponentInternals::Chip(_, inner_scale) = chip.internals {
					let new_viewport = self.viewport
						.transform_in_to_chip(chip.position, inner_scale);
	
					self.chip_stack.push(i);
					self.viewport = new_viewport;
					circuit = self.get_current_circuit(root_circuit);
	
					break;
				}
			}
		}

		while let Some(parent_chip) = self.get_parent_chip(root_circuit) {
			let new_viewport = self.viewport
				.transform_out_of_chip(parent_chip.position, parent_chip.internals.get_inner_scale().unwrap());

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
