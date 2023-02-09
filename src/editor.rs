use wasm_bindgen::prelude::*;

use crate::{editor_example, get_computer_circuit};
use crate::bus::{BusLayoutCommand, compute_wire_commands};
use crate::core::{Circuit, ComponentOptions, ComponentType, ExternalPin, PinState};
use crate::graphics::{BoundingBox, Renderer};

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

	/// Whether the user is holding the mouse down.
	is_mouse_down: bool,
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

	/// Whether the user is able to draw wires.
	is_in_wire_mode: bool,
	/// Whether the user is selecting the end pins as opposed to the start pins.
	is_selecting_end_pins: bool,
	/// The list of selected start pins.
	start_pins: Vec<ExternalPin>,
	/// The list of selected end pins.
	end_pins: Vec<ExternalPin>,
	/// Whether the user is editing the layout commands.
	is_editing_layout: bool,
	/// The current list of [`BusLayoutCommand`]s.
	layout_commands: Vec<BusLayoutCommand>,

	/// The clipboard.
	clipboard: Vec<(ComponentType, (f64, f64), ComponentOptions)>,
}

#[wasm_bindgen]
impl Editor {
	/// Creates a new editor instance.
	#[wasm_bindgen(constructor)]
	pub fn new(ctx: web_sys::CanvasRenderingContext2d) -> Self {
		let mut circuit = get_computer_circuit();
		let mut renderer = Renderer::new(ctx);
		
		renderer.update_sim_modes(&mut circuit);
		renderer.switch_pin_mode();

		Self {
			circuit,
			renderer,

			is_mouse_down: false,
			is_panning: false,
			has_moved: false,
			prev_cursor_pos: (0.0, 0.0),

			selected_chip_stacks: vec![],
			initial_cursors: vec![],
			initial_positions: vec![],

			is_in_wire_mode: true,
			is_selecting_end_pins: false,
			start_pins: vec![],
			end_pins: vec![],
			is_editing_layout: false,
			layout_commands: vec![],

			clipboard: vec![],
		}
	}

	/// Updates the editor size to fit the window size.
	pub fn update_size(&mut self) {
		self.renderer.update_size();
		self.renderer.update_sim_modes(&mut self.circuit);
	}

	/// Spawns a new component and returns the index of that component in the circuit.
	pub fn spawn_component(&mut self, component_type: ComponentType, should_flip_multi_junction: bool) -> usize {
		// let x = self.renderer.viewport.get_x();
		// let y = self.renderer.viewport.get_y();

		let x = 0.0;
		let y = 0.0;
		
		let component = component_type.create((x, y), ComponentOptions {
			size: match component_type {
				ComponentType::MultiBulb | ComponentType::MultiSwitch | ComponentType::MultiJunction |
				ComponentType::Adder | ComponentType::MultiDFlipFlop |
				ComponentType::MultiTriStateBuffer | ComponentType::MultiMultiplexer |
				ComponentType::FourWayMultiMultiplexer | ComponentType::Rom => 16,
				ComponentType::Junction => 3,
				_ => 1,
			},
			should_flip_multi_junction,
		});

		let index = self.circuit.add(component);

		self.selected_chip_stacks = vec![vec![index]];
		updateSelection(true, 0.0, 0.0);

		index
	}

	/// Copies the selected components to the clipboard.
	pub fn copy(&mut self) {
		self.clipboard.clear();

		for chip_stack in &self.selected_chip_stacks {
			let component = self.circuit.get_component_from_chip_stack(chip_stack).unwrap();

			self.clipboard.push((
				component.get_type(),
				component.position,
				component.options,
			));
		}
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
		let (_, y) = self.circuit.get_pos_from_chip_stack(chip_stack).unwrap();
		self.circuit.set_component_pos_from_chip_stack(chip_stack, x, y);
	}

	/// Set the y coordinate of the selected component.
	pub fn set_selected_y(&mut self, y: f64) {
		let chip_stack = &self.selected_chip_stacks[0];
		let (x, _) = self.circuit.get_pos_from_chip_stack(chip_stack).unwrap();
		self.circuit.set_component_pos_from_chip_stack(chip_stack, x, y);
	}

	/// Toggle wire mode on and off.
	pub fn toggle_wire_mode(&mut self) {
		self.is_in_wire_mode = !self.is_in_wire_mode;
		self.renderer.switch_pin_mode();
	}

	/// Returns the vector corresponding to the pins being selected
	/// (either `start_pins` or `end_pins`).
	fn get_pin_vec(&mut self) -> &mut Vec<ExternalPin> {
		if self.is_selecting_end_pins {
			&mut self.end_pins
		} else {
			&mut self.start_pins
		}
	}

	/// Update the wire layout.
	fn update_wire_layout(&mut self) {
		let start_positions: Vec<_> = self.start_pins.iter()
			.map(|p| {
				let component = &self.circuit.components[p.component_idx];
				let comp_pos = component.position;
				let pin_pos = component.get_pin_positions()[p.pin_idx];

				(comp_pos.0 + pin_pos.0, comp_pos.1 + pin_pos.1)
			})
			.collect();

			
		let end_positions: Vec<_> = self.end_pins.iter()
			.map(|p| {
				let component = &self.circuit.components[p.component_idx];
				let comp_pos = component.position;
				let pin_pos = component.get_pin_positions()[p.pin_idx];

				(comp_pos.0 + pin_pos.0, comp_pos.1 + pin_pos.1)
			})
			.collect();

		let wire_commands = compute_wire_commands(&self.layout_commands, &start_positions, &end_positions);

		for ((start, end), wc) in self.start_pins.iter().zip(&self.end_pins).zip(wire_commands) {
			self.circuit.connect(
				(start.component_idx, start.pin_idx),
				(end.component_idx, end.pin_idx),
				&wc,
			);
		}
	}

	/// Add a new layout command.
	fn add_layout_command(&mut self, command: BusLayoutCommand) {
		if let Some(end_command) = self.layout_commands.pop() {
			self.layout_commands.push(command);
			self.layout_commands.push(end_command);
			self.update_wire_layout();
		}
	}

	/// Align the current wire horizontally to the end pin.
	pub fn wire_align_horizontal(&mut self) {
		self.add_layout_command(BusLayoutCommand::AlignHorizontal);
	}

	/// Align the current wire vertically to the end pin.
	pub fn wire_align_vertical(&mut self) {
		self.add_layout_command(BusLayoutCommand::AlignVertical);
	}

	/// Move the wire to the horizontal center between the pins.
	pub fn wire_center_horizontal(&mut self) {
		self.add_layout_command(BusLayoutCommand::CenterHorizontal);
	}

	/// Move the wire to the vertical center between the pins,
	/// or paste the component on the clipboard.
	pub fn handle_ctrl_v(&mut self) {
		if self.is_editing_layout {
			self.add_layout_command(BusLayoutCommand::CenterVertical);
		} else {
			for (ctype, pos, options) in &self.clipboard {
				let component = ctype.create(*pos, *options);

				let index = self.circuit.add(component);
		
				self.selected_chip_stacks = vec![vec![index]];
				updateSelection(true, 0.0, 0.0);
			}
		}
	}

	/// Finish laying out the current wire or finish selecting pins.
	pub fn handle_confirm(&mut self) {
		if self.is_editing_layout {
			self.layout_commands.pop();
			self.update_wire_layout();
			
			self.is_selecting_end_pins = false;
			self.is_editing_layout = false;
	
			self.start_pins.clear();
			self.end_pins.clear();
		} else if self.is_selecting_end_pins {
			self.layout_commands = vec![
				BusLayoutCommand::MoveTo((0.0, 0.0)),
			];

			self.update_wire_layout();

			self.is_editing_layout = true;
		} else {
			self.is_selecting_end_pins = true;
		}
	}

	/// Handle the user pressing down a mouse button.
	pub fn handle_mouse_down(&mut self, x: f64, y: f64) {
		self.is_mouse_down = true;

		if self.is_in_wire_mode {
			if let Some(clicked_pin) = self.renderer.get_clicked_pin(&self.circuit, x, y) {
				self.get_pin_vec().push(clicked_pin);
				return;
			} else if !self.is_editing_layout {
				self.get_pin_vec().clear();
			}
		}

		let clicked_chip_stack = self.renderer.get_chip_stack_from_pos(&self.circuit, x, y);

		let did_click_component = !clicked_chip_stack.is_empty();
		// let did_click_component = false;

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
				&self.circuit, cs,
				x, y
			));
			self.initial_positions.push(self.circuit.get_pos_from_chip_stack(cs).unwrap());
		}
	}

	/// Handle the user moving the mouse.
	pub fn handle_mouse_move(&mut self, x: f64, y: f64, is_ctrl: bool, is_shift: bool, is_alt: bool) {
		self.has_moved = true;

		if self.is_panning {
			if !self.selected_chip_stacks.is_empty() {
				for (idx, chip_stack) in self.selected_chip_stacks.iter().enumerate() {
					let cursor = self.renderer.get_cursor_from_pos(
						&self.circuit, chip_stack,
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
						chip_stack,
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

			let mut new_x = cursor.get_x();
			let mut new_y = cursor.get_y();

			if is_alt {
				new_x = (new_x / 50.0).floor() * 50.0;
				new_y = (new_y / 50.0).floor() * 50.0;
			}

			let len = self.layout_commands.len();

			self.layout_commands[len - 1] =
				if is_ctrl {
					BusLayoutCommand::MoveYTo(new_y)
				} else if is_shift {
					BusLayoutCommand::MoveXTo(new_x)
				} else {
					BusLayoutCommand::MoveTo((new_x, new_y))
				};

			self.update_wire_layout();
		} else if self.is_mouse_down && !self.get_pin_vec().is_empty() {
			if let Some(clicked_pin) = self.renderer.get_clicked_pin(&self.circuit, x, y) {
				if !self.get_pin_vec().contains(&clicked_pin) {
					self.get_pin_vec().push(clicked_pin);
				}
			}
		}
	}

	/// Handle the user releasing a mouse button.
	pub fn handle_mouse_up(&mut self, x: f64, y: f64) {
		self.is_mouse_down = false;
		self.is_panning = false;

		let mut is_editing_layout = false;

		if let Some(clicked_pin) = self.renderer.get_clicked_pin(&self.circuit, x, y) {
			if self.start_pins.len() == 2 && self.start_pins[1] == clicked_pin {
				self.start_pins = vec![self.start_pins[0]];
				self.end_pins = vec![clicked_pin];

				let cursor = self.renderer.get_cursor_from_pos(&self.circuit, &[], x, y);
				self.layout_commands = vec![
					BusLayoutCommand::MoveTo((cursor.get_x(), cursor.get_y())),
				];
				
				self.update_wire_layout();

				is_editing_layout = true;
			}
		}

		let cursor = self.renderer.get_cursor_from_pos(&self.circuit, &[], x, y);
		
		if self.is_editing_layout {
			self.layout_commands.push(
				BusLayoutCommand::MoveTo((cursor.get_x(), cursor.get_y()))
			);

			self.update_wire_layout();
		} else if !self.has_moved {
			let clicked_chip_stack = self.renderer.get_chip_stack_from_pos(&self.circuit, x, y);

			if self.selected_chip_stacks.contains(&clicked_chip_stack) {
				let pos = self.circuit.get_pos_from_chip_stack(&clicked_chip_stack).unwrap();

				self.circuit.toggle_switch_from_chip_stack(
					&clicked_chip_stack,
					cursor.get_x() - pos.0,
					cursor.get_y() - pos.1,
				);
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
		if self.is_selecting_end_pins {
			self.renderer.render(&self.circuit, &self.selected_chip_stacks, &self.end_pins);
		} else {
			self.renderer.render(&self.circuit, &self.selected_chip_stacks, &self.start_pins);
		}
	}

	/// Returns the generated code for the circuit.
	pub fn generate_code(&self) -> String {
		let mut code = String::new();

		code += "let mut circuit = Circuit::new();\n\n";

		for (idx, component) in self.circuit.components.iter().enumerate() {
			let string = if component.options.size > 1 {
				if component.options.should_flip_multi_junction {
					format!(
						"let c{} = add!(circuit, {}, ({:.3}, {:.3}), {}, true);\n",
						idx,
						component.get_name(),
						component.position.0,
						component.position.1,
						component.options.size,
					)
				} else {
					format!(
						"let c{} = add!(circuit, {}, ({:.3}, {:.3}), {});\n",
						idx,
						component.get_name(),
						component.position.0,
						component.position.1,
						component.options.size,
					)
				}
			} else {
				format!(
					"let c{} = add!(circuit, {}, ({:.3}, {:.3}));\n",
					idx,
					component.get_name(),
					component.position.0,
					component.position.1,
				)
			};

			code += &string;
		}

		code += "\n";

		let mut has_set_switch = false;

		for (idx, component) in self.circuit.components.iter().enumerate() {
			if component.get_switch_count() == 1 && component.get_pin_state(0).unwrap() == PinState::On {
				has_set_switch = true;
	
				code += &format!(
					"circuit.components[c{idx}].simulator.as_mut().unwrap().set_pin_state_external(0, PinState::On).unwrap();\n",
				);
			}
		}

		if has_set_switch {
			code += "\n";
		}

		for wire in &self.circuit.wires {
			code += &format!(
				"circuit.connect((c{}, {}), (c{}, {}), &[{}]);\n",
				wire.pin1.component_idx,
				wire.pin1.pin_idx,
				wire.pin2.component_idx,
				wire.pin2.pin_idx,
				wire.layout_commands
					.try_get()
					.unwrap()
					.iter()
					.map(|lc| lc.as_string())
					.collect::<Vec<_>>().join(", "),
			);
		}

		code += "circuit";

		code
	}

	/// Toggles a switch in the circuit.
	pub fn toggle_switch(&mut self, idx: usize) {
		self.circuit.toggle_switch(idx);
	}
}
