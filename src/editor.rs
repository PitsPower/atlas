use wasm_bindgen::prelude::*;

use crate::core::{Bulb, Circuit, ComponentType, Junction, Switch, ExternalPin};
use crate::gates::{AndGate, NorGate, OrGate};
use crate::graphics::{BoundingBox, Renderer, WireLayoutCommand};
use crate::transistor::{NTransistor, PTransistor};

#[wasm_bindgen(module="/web/src/updateSelection.js")]
extern "C" {
	fn updateSelection(has_selection: bool, selected_x: f64, selected_y: f64);
}

/// The circuit editor. Used to create circuits using a GUI.
#[wasm_bindgen]
pub struct Editor {
	/// The circuit being edited.
	circuit: Circuit,
	/// The renderer used to display the circuit.
	renderer: Renderer,

	/// Whether the user is currently panning.
	is_panning: bool,
	/// Whether the user has moved after the last mouse down.
	has_moved: bool,
	/// The previous cursor position.
	prev_cursor_pos: (f64, f64),

	/// The set of currently selected chip stack.
	selected_chip_stacks: Vec<Vec<usize>>,
	/// The initial cursor locations relative to the selected chip stacks.
	initial_cursors: Vec<BoundingBox>,
	/// The initial positions of the selected components.
	initial_positions: Vec<(f64, f64)>,

	/// The list of selected start pins.
	start_pins: Vec<ExternalPin>,
	/// The list of selected end pins.
	end_pins: Vec<ExternalPin>,
	/// Whether the user is editing the layout commands
	is_editing_layout: bool,
	/// The current list of [`WireLayoutCommands`].
	layout_commands: Vec<WireLayoutCommand>,
}

#[wasm_bindgen]
impl Editor {
	/// Creates a new editor instance.
	#[wasm_bindgen(constructor)]
	pub fn new(ctx: web_sys::CanvasRenderingContext2d) -> Self {
		let mut circuit = Circuit::new();
		let mut renderer = Renderer::new(ctx);
		
		renderer.update_sim_modes(&mut circuit);
		renderer.switch_pin_mode();

		Self {
			circuit,
			renderer,

			is_panning: false,
			has_moved: false,
			prev_cursor_pos: (0.0, 0.0),

			selected_chip_stacks: vec![],
			initial_cursors: vec![],
			initial_positions: vec![],

			start_pins: vec![],
			end_pins: vec![],
			is_editing_layout: false,
			layout_commands: vec![],
		}
	}

	/// Updates the editor size to fit the window size.
	pub fn update_size(&mut self) {
		self.renderer.update_size();
		self.renderer.update_sim_modes(&mut self.circuit);
	}

	/// Spawns a new component and returns the index of that component in the circuit.
	pub fn spawn_component(&mut self, component_type: ComponentType) -> usize {
		// let x = self.renderer.viewport.get_x();
		// let y = self.renderer.viewport.get_y();

		let x = 0.0;
		let y = 0.0;

		let index = match component_type {
			ComponentType::Bulb => crate::add!(self.circuit, Bulb, (x, y)),
			ComponentType::Junction => crate::add!(self.circuit, Junction, (x, y), 3),
			ComponentType::Switch => crate::add!(self.circuit, Switch, (x, y)),

			ComponentType::NTransistor => crate::add!(self.circuit, NTransistor, (x, y)),
			ComponentType::PTransistor => crate::add!(self.circuit, PTransistor, (x, y)),

			ComponentType::AndGate => crate::add!(self.circuit, AndGate, (x, y)),
			ComponentType::NorGate => crate::add!(self.circuit, NorGate, (x, y)),
			ComponentType::OrGate => crate::add!(self.circuit, OrGate, (x, y)),
		};

		self.selected_chip_stacks = vec![vec![index]];
		updateSelection(true, 0.0, 0.0);

		index
	}

	/// Deletes the selected component.
	pub fn delete_selected(&mut self) {
		for chip_stack in &self.selected_chip_stacks {
			self.circuit.remove(chip_stack[0]);
		}

		self.selected_chip_stacks = vec![];
		updateSelection(false, 0.0, 0.0);
	}

	/// Set the x coordinate of the selected component.
	pub fn set_selected_x(&mut self, x: f64) {
		let chip_stack = &self.selected_chip_stacks[0];
		let (_, y) = self.circuit.get_pos_from_chip_stack(&chip_stack).unwrap();
		self.circuit.set_component_pos_from_chip_stack(&chip_stack, x, y);
	}

	/// Set the y coordinate of the selected component.
	pub fn set_selected_y(&mut self, y: f64) {
		let chip_stack = &self.selected_chip_stacks[0];
		let (x, _) = self.circuit.get_pos_from_chip_stack(&chip_stack).unwrap();
		self.circuit.set_component_pos_from_chip_stack(&chip_stack, x, y);
	}

	/// Update the wire layout.
	fn update_wire_layout(&mut self) {
		let start = self.start_pins[0];
		let end = self.end_pins[0];
		self.circuit.re_lay_wire(start, end, self.layout_commands.clone());
	}

	/// Align the current wire horizontally to the end pin.
	pub fn wire_align_horizontal(&mut self) {
		let end_command = self.layout_commands.pop().unwrap();
		self.layout_commands.push(WireLayoutCommand::AlignHorizontal);
		self.layout_commands.push(end_command);
		self.update_wire_layout();
	}

	/// Align the current wire vertically to the end pin.
	pub fn wire_align_vertical(&mut self) {
		let end_command = self.layout_commands.pop().unwrap();
		self.layout_commands.push(WireLayoutCommand::AlignVertical);
		self.layout_commands.push(end_command);
		self.update_wire_layout();
	}

	/// Finish laying out the current wire.
	pub fn finish_layout(&mut self) {
		self.layout_commands.pop();
		self.update_wire_layout();
		
		self.is_editing_layout = false;

		self.start_pins.clear();
		self.end_pins.clear();
	}

	/// Handle the user pressing down a mouse button.
	pub fn handle_mouse_down(&mut self, x: f64, y: f64) {
		if let Some(clicked_pin) = self.renderer.get_clicked_pin(&self.circuit, x, y) {
			self.start_pins.push(clicked_pin);
			return;
		} else if !self.is_editing_layout {
			self.start_pins.clear();
		}

		let clicked_chip_stack = self.renderer.get_chip_stack_from_pos(&self.circuit, x, y);

		let did_click_component = clicked_chip_stack.len() > 0;

		if did_click_component {
			let (cx, cy) = self.circuit.get_pos_from_chip_stack(&clicked_chip_stack[..]).unwrap();

			self.selected_chip_stacks = vec![clicked_chip_stack];
			updateSelection(true, cx, cy);
		} else {
			self.selected_chip_stacks = vec![];
			updateSelection(false, 0.0, 0.0);
		}

		if !self.is_editing_layout {
			self.is_panning = true;
		}

		self.has_moved = false;
		self.prev_cursor_pos = (x, y);

		self.initial_cursors.clear();
		self.initial_positions.clear();

		for cs in self.selected_chip_stacks.iter() {
			self.initial_cursors.push(self.renderer.get_cursor_from_pos(
				&self.circuit, &cs,
				x, y
			));
			self.initial_positions.push(self.circuit.get_pos_from_chip_stack(cs).unwrap());
		}
	}

	/// Handle the user moving the mouse.
	pub fn handle_mouse_move(&mut self, x: f64, y: f64, is_ctrl: bool, is_shift: bool, is_alt: bool) {
		self.has_moved = true;

		if self.is_panning {
			if self.selected_chip_stacks.len() > 0 {
				for (idx, chip_stack) in self.selected_chip_stacks.iter().enumerate() {
					let cursor = self.renderer.get_cursor_from_pos(
						&self.circuit, &chip_stack,
						x, y
					);
	
					let mut x_diff = cursor.get_x() - self.initial_cursors[idx].get_x();
					let mut y_diff = cursor.get_y() - self.initial_cursors[idx].get_y();

					if is_alt {
						x_diff = (x_diff / 50.0).floor() * 50.0;
						y_diff = (y_diff / 50.0).floor() * 50.0;
					}

					let mut new_x = self.initial_positions[idx].0;
					let mut new_y = self.initial_positions[idx].1;

					if !is_ctrl {
						new_x += x_diff;
					}
					if !is_shift {
						new_y += y_diff;
					}
		
					self.circuit.set_component_pos_from_chip_stack(
						&chip_stack,
						new_x, new_y,
					);

					updateSelection(true, new_x, new_y);
				}
			} else {
				let x_diff = x - self.prev_cursor_pos.0;
				let y_diff = y - self.prev_cursor_pos.1;
	
				self.renderer.pan(x_diff, y_diff);
			}
	
			self.prev_cursor_pos = (x, y);
		} else if self.is_editing_layout {
			let cursor = self.renderer.get_cursor_from_pos(&self.circuit, &[], x, y);

			let len = self.layout_commands.len();
			self.layout_commands[len - 1] =
				WireLayoutCommand::MoveTo((cursor.get_x(), cursor.get_y()));

			self.update_wire_layout();
		}
	}

	/// Handle the user releasing a mouse button.
	pub fn handle_mouse_up(&mut self, x: f64, y: f64) {
		self.is_panning = false;

		let mut is_editing_layout = false;

		if let Some(clicked_pin) = self.renderer.get_clicked_pin(&self.circuit, x, y) {
			if self.start_pins.len() == 1 && self.start_pins[0] != clicked_pin {
				self.end_pins = vec![clicked_pin];

				let start = self.start_pins[0];
				let end = clicked_pin;

				let cursor = self.renderer.get_cursor_from_pos(&self.circuit, &[], x, y);
				self.layout_commands = vec![
					WireLayoutCommand::MoveTo((cursor.get_x(), cursor.get_y())),
				];

				self.circuit.connect(
					(start.component_idx, start.pin_idx),
					(end.component_idx, end.pin_idx),
					self.layout_commands.clone(),
				);

				is_editing_layout = true;
			}
		}
		
		if self.is_editing_layout {
			let cursor = self.renderer.get_cursor_from_pos(&self.circuit, &[], x, y);

			self.layout_commands.push(
				WireLayoutCommand::MoveTo((cursor.get_x(), cursor.get_y()))
			);

			self.update_wire_layout();
		} else if !self.has_moved {
			let clicked_chip_stack = self.renderer.get_chip_stack_from_pos(&self.circuit, x, y);

			if self.selected_chip_stacks.contains(&clicked_chip_stack) {
				self.circuit.toggle_switch_from_chip_stack(&clicked_chip_stack);
			} 
		}

		if is_editing_layout {
			self.is_editing_layout = true;
		}
	}

	/// Zoom in or out.
	pub fn zoom(&mut self, zoom: f64, x: f64, y: f64) {
		self.renderer.zoom(zoom, x, y);
		self.renderer.update_sim_modes(&mut self.circuit);
	}

	/// Render the editor.
	pub fn render(&mut self) {
		self.renderer.render(&self.circuit, &self.selected_chip_stacks, &self.start_pins);
	}
}
