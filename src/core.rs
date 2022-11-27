//! Core ATLAS functionality.
//! 
//! Provides core data structures such as [`Circuit`] and some basic components like [`Switch`], [`Bulb`], and
//! [`Junction`].

use std::f64::consts::PI;

use wasm_bindgen::prelude::*;

use crate::graphics::{BoundingBox, Drawable, WireLayoutCommand};

/// A pin state.
/// 
/// Every [`Component`] has a number of pins that can be read from or written to. For example, an AND gate
/// has two pins for input and one pin for output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinState {
	/// Analogous to a high signal (e.g. 5V).
	On,
	/// Analogous to a low signal (e.g. 0V).
	Off,
	/// Analogous to a disconnected signal. In a lot of cases this functions in the same way as low,
	/// but with the [`Junction`] component it behaves differently.
	Disconnected,
}

impl PinState {
	/// Combines a pin state with another pin state. If one of the states is [`PinState::Disconnected`],
	/// the other one is chosen.
	fn combine<'a>(&'a self, other: &'a PinState) -> &'a PinState {
		if self == &PinState::Disconnected {
			other
		} else {
			self
		}
	}

	/// Toggles a pin state. If the state is [`PinState::Disconnected`], toggling doesn't affect it.
	pub fn toggle(&self) -> PinState {
		match *self {
			PinState::On => PinState::Off,
			PinState::Off => PinState::On,
			PinState::Disconnected => PinState::Disconnected,
		}
	}

	/// Returns the colour of the state. Used in rendering.
	fn get_colour(&self) -> &str {
		match self {
			PinState::On => "#fb016e",
			PinState::Off => "#333",
			PinState::Disconnected => "#999",
		}
	}

	/// Returns `true` if the input is [`PinState::On`] and `false` otherwise.
	pub fn to_bool(&self) -> bool {
		*self == PinState::On
	}

	/// Returns [`PinState::On`] if the input is `true` and [`PinState::Off`] if
	/// the input is `false`. 
	pub fn from_bool(b: bool) -> PinState {
		if b { PinState::On } else { PinState::Off }
	}

	/// Returns the XOR of this signal with the given signal.
	pub fn xor(&self, other: PinState) -> PinState {
		match (self.to_bool(), other.to_bool()) {
			(true, true) => PinState::Off,
			(true, false) => PinState::On,
			(false, true) => PinState::On,
			(false, false) => PinState::Off,
		}
	}
}

/// Converts a list of pin states into a number by interpreting the states as a binary value.
/// [`PinState::Disconnected`] and [`PinState::Off`] are treated as 0 and [`PinState::On`] is treated as 1.
/// The first state in the list is treated as the most significant bit.
pub fn states_to_num(states: &Vec<PinState>) -> u32 {
	let mut result = 0;

	for state in states {
		result *= 2;
		if *state == PinState::On {
			result += 1;
		}
	}

	result
}

/// Convert a number into a list of pin states where each pin state is a binary bit in the number.
/// The first state in the list is treated as the most significant bit.
pub fn num_to_states(num: u32) -> Vec<PinState> {
	let mut result = vec![];
	let mut current = num;

	while current != 0 {
		result.insert(0, if current % 2 == 1 { PinState::On } else { PinState::Off });
		current /= 2;
	}

	result
}

/// An error that may occur when getting or setting a pin.
#[derive(Debug)]
pub enum PinError {
	/// The pin index used is too large for the component.
	OutOfRange,
}

/// How a chip should be simulated.
#[derive(Clone, Copy)]
pub enum SimulationMode {
	/// Simulate the circuit in the chip.
	Circuit,
	/// Simulate the chip using the high level implementation.
	HighLevel,
}

/// The different kinds of component.
#[wasm_bindgen]
pub enum ComponentType {
	Bulb,
	Junction,
	Switch,
	
	NTransistor,
	PTransistor,

	AndGate,
	NandGate,
	NorGate,
	NotGate,
	OrGate,

	MultiSwitch,
	MultiBulb,

	Adder,
	MultiDFlipFlop,

	Multiplexer,
	TwoBitMultiplexer,
}

/// Returns the name of the given [`ComponentType`].
#[wasm_bindgen]
pub fn get_ct_name(ct: ComponentType) -> String {
	match ct {
		ComponentType::Bulb => String::from("Bulb"),
		ComponentType::Junction => String::from("Junction"),
		ComponentType::Switch => String::from("Switch"),

		ComponentType::NTransistor => String::from("N-type Transistor"),
		ComponentType::PTransistor => String::from("P-type Transistor"),

		ComponentType::AndGate => String::from("AND Gate"),
		ComponentType::NandGate => String::from("NAND Gate"),
		ComponentType::NorGate => String::from("NOR Gate"),
		ComponentType::NotGate => String::from("NOT Gate"),
		ComponentType::OrGate => String::from("OR Gate"),

		ComponentType::MultiSwitch => String::from("Multi Switch"),
		ComponentType::MultiBulb => String::from("Multi Bulb"),

		ComponentType::Adder => String::from("8-bit Adder"),
		ComponentType::MultiDFlipFlop => String::from("8-bit D Flip-Flop"),

		ComponentType::Multiplexer => String::from("Multiplexer"),
		ComponentType::TwoBitMultiplexer => String::from("2-bit Multiplexer"),
	}
}

/// Returns the slug of the given [`ComponentType`].
#[wasm_bindgen]
pub fn get_ct_slug(ct: ComponentType) -> String {
	match ct {
		ComponentType::Bulb => String::from("bulb"),
		ComponentType::Junction => String::from("junction"),
		ComponentType::Switch => String::from("switch"),

		ComponentType::NTransistor => String::from("ntransistor"),
		ComponentType::PTransistor => String::from("ptransistor"),

		ComponentType::AndGate => String::from("andgate"),
		ComponentType::NandGate => String::from("nandgate"),
		ComponentType::NorGate => String::from("norgate"),
		ComponentType::NotGate => String::from("notgate"),
		ComponentType::OrGate => String::from("orgate"),

		ComponentType::MultiSwitch => String::from("multiswitch"),
		ComponentType::MultiBulb => String::from("multibulb"),

		ComponentType::Adder => String::from("adder"),
		ComponentType::MultiDFlipFlop => String::from("multidflipflop"),

		ComponentType::Multiplexer => String::from("multiplexer"),
		ComponentType::TwoBitMultiplexer => String::from("twobitmultiplexer"),
	}
}

/// Something that can go in a circuit. A [`Component`] may be connected to another [`Component`] using a [`Wire`].
pub trait Component: Drawable {
	/// Returns the name of the component as a string.
	fn get_name(&self) -> String;

	/// Returns the component's internals. This is used for components that contain a [`Circuit`].
	fn get_internals(&self) -> Option<&ChipInternals> {
		None
	}

	/// Returns the component's internals as mutable.
	fn get_internals_mut(&mut self) -> Option<&mut ChipInternals> {
		None
	}

	/// Returns how many switches are in the component.
	fn get_switch_count(&self) -> usize {
		0
	}

	/// Returns whether the component is a [`Pin`] component or not.
	fn is_pin(&self) -> bool {
		false
	}

	/// Returns the positions of each internal pin.
	fn get_pin_positions(&self) -> Vec<(f64, f64)>;

	/// Returns the number of pins the component has.
	fn get_pin_count(&self) -> usize {
		self.get_pin_positions().len()
	}

	/// Returns the state of a pin.
	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError>;

	/// Sets the state of a pin.
	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError>;

	/// Returns the state of a pin when accessed externally. This is used for accessing a [`Pin`] from
	/// the [`Chip`] it's in.
	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		self.get_pin_state(idx)
	}

	/// Sets the state of a pin externally. This is used to manually modify pins (e.g. when turning on a switch).
	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		self.set_pin_state(idx, state)
	}

	/// Returns the position of the component.
	fn get_position(&self) -> (f64, f64);
	
	/// Sets the position of the component.
	fn set_position(&mut self, pos: (f64, f64));

	/// Translates the component.
	fn translate(&mut self, offset: (f64, f64)) {
		let old_pos = self.get_position();
		self.set_position((old_pos.0 + offset.0, old_pos.1 + offset.1));
	}
	
	/// Returns the size of the component.
	fn get_size(&self) -> (f64, f64);

	/// Sets the simulation mode of the chip to the given mode.
	fn set_mode(&mut self, _mode: SimulationMode) {

	}

	/// Returns whether the given viewport is fully contained within the component.
	fn contains(&self, _viewport: &BoundingBox) -> bool {
		false
	}

	/// Returns whether the given viewport is partially contained within the component.
	fn intersects(&self, viewport: &BoundingBox) -> bool {
		let position = self.get_position();
		let size = self.get_size();

		let intersects_x =
			position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
	}

	/// Returns whether the given viewport can see the internals of the component.
	fn are_internals_visible(&self, viewport: &BoundingBox) -> bool {
		let start_ratio = 0.3;

		let height = self.get_size().1;
		let height_ratio = height / viewport.get_size().1;

		self.intersects(viewport) && height_ratio > start_ratio
	}
}

/// A specifier for a pin on a particular component. This differs from [`Pin`], which is an internal
/// pin used in a [`Circuit`] within a [`Chip`].
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExternalPin {
	/// The index of the component.
	pub component_idx: usize,
	/// The index of the pin on the component.
	pub pin_idx: usize,
}

/// A wire. Wires connect two external pins together.
/// 
/// A wire stores two states, one for each pin. This allows for wires to work
/// correctly in both directions.
#[derive(Clone, Debug)]
pub struct Wire {
	/// The first pin that the wire is connected to.
	pub pin1: ExternalPin,
	/// The second pin that the wire is connected to.
	pub pin2: ExternalPin,
	/// Commands used to specify how the wire is rendered.
	pub layout_commands: Vec<WireLayoutCommand>,
	/// The state being emitted by pin 1.
	state1: PinState,
	/// The state being emitted by pin 2.
	state2: PinState,
}

/// A circuit that consists of components connected by wires.
#[wasm_bindgen]
pub struct Circuit {
	/// The list of components.
	components: Vec<Box<dyn Component>>,
	/// The list of wires connecting the components.
	wires: Vec<Wire>,
}

impl Circuit {
	/// Returns the list of components in the circuit.
	pub fn get_components(&self) -> &Vec<Box<dyn Component>> {
		&self.components
	}

	/// Returns the mutable list of components in the circuit.
	pub fn get_components_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
		&mut self.components
	}

	/// Returns the list of wires in the circuit.
	pub fn get_wires(&self) -> &Vec<Wire> {
		&self.wires
	}

	/// Returns the mutable list of wires in the circuit.
	pub fn get_wires_mut(&mut self) -> &mut Vec<Wire> {
		&mut self.wires
	}

	/// Returns the list of [`Pin`] components in the circuit.
	pub fn get_pins(&self) -> Vec<&dyn Component> {
		self.components.iter()
			.filter(|c| c.is_pin())
			.map(|c| c.as_ref())
			.collect()
	}

	/// Sets a [`Pin`] component to a given [`PinState`].
	pub fn set_pin(&mut self, idx: usize, state: PinState) {
		let true_idx = self.components.iter()
			.enumerate()
			.filter(|(_, c)| c.is_pin())
			.nth(idx)
			.unwrap().0;

		self.update_component(&ExternalPin {
			component_idx: true_idx,
			pin_idx: 0,
		}, state, true);
	}

	/// Adds a component to the circuit.
	pub fn add(&mut self, component: Box<dyn Component>) -> usize {
		let idx = self.components.len();
		self.components.push(component);
		idx
	}

	/// Removes a component from the circuit.
	pub fn remove(&mut self, component_idx: usize) {
		self.wires.retain(|w| w.pin1.component_idx != component_idx && w.pin2.component_idx != component_idx);

		for wire in &mut self.wires {
			if wire.pin1.component_idx > component_idx {
				wire.pin1.component_idx -= 1;
			}
			if wire.pin2.component_idx > component_idx {
				wire.pin2.component_idx -= 1;
			}
		}

		self.components.remove(component_idx);
	}

	/// Connects two components together with a wire.
	pub fn connect(
		&mut self, (comp1_idx, pin1_idx): (usize, usize),
		(comp2_idx, pin2_idx): (usize, usize),
		wire_commands: Vec<WireLayoutCommand>,
	) {
		let pin1 = ExternalPin { component_idx: comp1_idx, pin_idx: pin1_idx };
		let pin2 = ExternalPin { component_idx: comp2_idx, pin_idx: pin2_idx };

		for wire in &mut self.wires {
			if wire.pin1 == pin1 && wire.pin2 == pin2 {
				wire.layout_commands = wire_commands;
				return;
			}
		}

		let start_state = match self.components[comp1_idx].get_pin_state(pin1_idx) {
			Ok(state) => state,
			Err(PinError::OutOfRange) => panic!("Pin index {} out of range for component {}", pin1_idx, comp1_idx),
		};

		let end_state = match self.components[comp2_idx].get_pin_state(pin2_idx) {
			Ok(state) => state,
			Err(PinError::OutOfRange) => panic!("Pin index {} out of range for component {}", pin1_idx, comp1_idx),
		};

		let start_con = ExternalPin { component_idx: comp1_idx, pin_idx: pin1_idx };
		let end_con = ExternalPin { component_idx: comp2_idx, pin_idx: pin2_idx };

		if start_state == PinState::Disconnected && end_state != PinState::Disconnected {
			self.update_component(&start_con, end_state, false);
		}
		else if end_state == PinState::Disconnected && start_state != PinState::Disconnected {
			self.update_component(&end_con, start_state, false);
		}

		self.wires.push(Wire {
			pin1: start_con,
			pin2: end_con,
			layout_commands: wire_commands,
			state1: start_state,
			state2: end_state,
		});
	}
	
	/// Updates a pin and then propagates the changes. This function is the main
	/// part of the circuit simulator.
	pub fn update_component(&mut self, pin: &ExternalPin, state: PinState, set_manually: bool) {
		let component = &mut self.components[pin.component_idx];

		let old_pin_states: Vec<_> = (0..component.get_pin_count())
			.map(|i| component.get_pin_state(i).unwrap())
			.collect();

		if set_manually {
			component.set_pin_state_external(pin.pin_idx, state).unwrap();
		} else {
			component.set_pin_state(pin.pin_idx, state).unwrap();
		}

		let mut components_to_update = vec![];
		let mut wire_starts_to_update = vec![];
		let mut wire_ends_to_update = vec![];

		for (i, old_pin_state) in old_pin_states.iter().enumerate().take(component.get_pin_count()) {
			if i == pin.pin_idx && !set_manually {
				continue;
			}

			let con = ExternalPin { component_idx: pin.component_idx, pin_idx: i };
			let state = component.get_pin_state(i).unwrap();

			if let Some((wire_idx, wire)) = self.wires.iter().enumerate()
				.find(|(_, w)| w.pin1 == con || w.pin2 == con)
			{
				if wire.pin1 == con {
					wire_starts_to_update.push((wire_idx, state));

					if wire.state2 == PinState::Disconnected && wire.state1 != state {
						components_to_update.push((wire.pin2, state));
					} else if state == PinState::Disconnected && *old_pin_state != PinState::Disconnected {
						components_to_update.push((wire.pin1, wire.state2));
					}
				} else {
					wire_ends_to_update.push((wire_idx, state));
					
					if wire.state1 == PinState::Disconnected && wire.state2 != state {
						components_to_update.push((wire.pin1, state));
					} else if state == PinState::Disconnected && *old_pin_state != PinState::Disconnected {
						components_to_update.push((wire.pin2, wire.state1));
					}
				}
			}
		}
		
		for (idx, state) in wire_starts_to_update {
			self.wires[idx].state1 = state;
		}
		for (idx, state) in wire_ends_to_update {
			self.wires[idx].state2 = state;
		}
		for (con, state) in components_to_update {
			let mut true_state = state;

			if state == PinState::Disconnected {
				if let Some(wire) = self.wires.iter()
					.find(|w| w.pin1 == con || w.pin2 == con)
				{
					if wire.pin1 == con {
						true_state = wire.state2;
					} else {
						true_state = wire.state1;
					}
				}
			}

			self.update_component(&con, true_state, false);
		}
	}

	/// Returns a component given a chip stack.
	fn get_component_from_chip_stack(&mut self, stack: &[usize]) -> Option<&mut Box<dyn Component>> {
		match stack.len() {
			0 => None,
			1 => Some(&mut self.components[stack[0]]),
			_ => self.components[stack[0]]
					.get_internals_mut()
					.unwrap()
					.circuit
					.get_component_from_chip_stack(&stack[1..]),
		}
	}

	/// Returns a pin state given a chip stack.
	fn get_pin_state_from_chip_stack(&self, stack: &[usize], x: f64, y: f64) -> Option<(usize, PinState)> {
		match stack.len() {
			0 => None,
			1 => {
				let pin_idx = self.components[stack[0]].get_pin_positions().iter()
					.map(|pos| ((pos.0 - x) * (pos.0 - x) + (pos.1 - y) * (pos.1 - y)).sqrt())
					.enumerate()
					.min_by(|(_, v0), (_, v1)| v0.partial_cmp(v1).unwrap())
					.map(|(idx, _)| idx)
					.unwrap();

				Some((pin_idx, self.components[stack[0]].get_pin_state(pin_idx).unwrap()))
			},
			// _ => self.components[stack[0]]
			// 		.get_internals()
			// 		.unwrap()
			// 		.circuit
			// 		.get_pin_state_from_chip_stack(&stack[1..], pin_idx),
			_ => None,
		}
	}

	/// Updates a switch given a chip stack and pin index.
	fn set_switch_from_chip_stack(&mut self, stack: &[usize], pin_idx: usize, state: PinState, set_manually: bool) {
		match stack.len() {
			0 => {},
			1 => {
				let component = &self.components[stack[0]];

				if component.get_switch_count() > 0 {
					self.update_component(&ExternalPin { component_idx: stack[0], pin_idx }, state, set_manually);
				}
			},
			_ => {
				// TODO: Maybe turn this back on again? Although there's not much point really
				// self.components[stack[0]]
				// 	.get_internals_mut()
				// 	.unwrap()
				// 	.circuit
				// 	.set_switch_from_chip_stack(&stack[1..], pin_idx, state, set_manually);
			},
		}
	}

	/// Returns the positions of each [`Pin`].
	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		self.components.iter()
			.filter(|c| c.is_pin())
			.map(|c| c.get_position())
			.collect()
	}

	/// Draws the selection boxes for each component.
	pub fn draw_selection_boxes(&self, ctx: &web_sys::CanvasRenderingContext2d, selected_chip_stacks: &[Vec<usize>]) {
		let first_indices: Vec<_> = selected_chip_stacks.iter().map(|cs| cs[0]).collect();

		for (cidx, component) in self.components.iter().enumerate() {
			if first_indices.contains(&cidx) {
				let (x, y) = component.get_position();
				let (mut width, mut height) = component.get_size();

				width += 30.0;
				height += 30.0;

				ctx.set_stroke_style(&"#0f0".into());
				ctx.set_line_width(7.0);
				ctx.stroke_rect(x - width * 0.5, y - height * 0.5, width, height);
			}
		}
	}

	/// Draws the pin highlights.
	pub fn draw_pin_highlights(&self, ctx: &web_sys::CanvasRenderingContext2d, selected_pins: &[ExternalPin]) {
		for (cidx, component) in self.components.iter().enumerate() {
			ctx.save();
			
			let (x, y) = component.get_position();
			ctx.translate(x, y).unwrap();

			for (pidx, pin_pos) in component.get_pin_positions().iter().enumerate().rev() {
				let con = ExternalPin { component_idx: cidx, pin_idx: pidx };

				if self.get_wires().iter().any(|w| w.pin1 == con || w.pin2 == con) {
					continue;
				}

				ctx.set_fill_style(&"#000".into());
				ctx.begin_path();
				ctx.arc(pin_pos.0, pin_pos.1, 8.0, 0.0, 2.0 * PI).unwrap();
				ctx.fill();
	
				if selected_pins.contains(&con) {
					ctx.set_fill_style(&"#ff0".into());
				} else {
					ctx.set_fill_style(&"#0f0".into());
				}

				ctx.begin_path();
				ctx.arc(pin_pos.0, pin_pos.1, 5.0, 0.0, 2.0 * PI).unwrap();
				ctx.fill();
			}
			
			ctx.restore();
		}
	}
}

impl Circuit {
	/// Returns a blank [`Circuit`].
	pub fn new() -> Self {
		Self {
			components: vec![],
			wires: vec![],
		}
	}

	/// Toggles a [`Switch`] in the circuit.
	pub fn toggle_switch(&mut self, idx: usize) {
		let mut component_idx = 0;
		let mut pin_idx = idx;

		loop {
			let switch_count = match self.components.get(component_idx) {
				Some(c) => c.get_switch_count(),
				None => return,
			};
			
			if pin_idx < switch_count {
				break;
			}

			pin_idx -= switch_count;
			component_idx += 1;
		}

		let state = self.components[component_idx].get_pin_state(pin_idx).unwrap();
		self.update_component(&ExternalPin { component_idx, pin_idx }, state.toggle(), true);
	}

	/// Returns the coordinates of a component given the chip stack.
	pub fn get_pos_from_chip_stack(&mut self, stack: &[usize]) -> Option<(f64, f64)> {
		let component = self.get_component_from_chip_stack(stack)?;
		Some(component.get_position())
	}

	/// Sets the x coordinate of a component given the chip stack.
	pub fn set_x_from_chip_stack(&mut self, stack: &[usize], x: f64) {
		if let Some(component) = self.get_component_from_chip_stack(stack) {
			component.set_position((
				x,
				component.get_position().1,
			));
		}
	}

	/// Sets the y coordinate of a component given the chip stack.
	pub fn set_y_from_chip_stack(&mut self, stack: &[usize], y: f64) {
		if let Some(component) = self.get_component_from_chip_stack(stack) {
			component.set_position((
				component.get_position().0,
				y,
			));
		}
	}

	/// Toggles the switch referred to by the stack (if the component is a switch).
	pub fn toggle_switch_from_chip_stack(&mut self, stack: &[usize], x: f64, y: f64) {
		if let Some((pin_idx, state)) = self.get_pin_state_from_chip_stack(stack, x, y) {
			self.set_switch_from_chip_stack(stack, pin_idx, state.toggle(), true);
		}
	}

	/// Sets a component's position given a chip stack.
	pub fn set_component_pos_from_chip_stack(&mut self, stack: &[usize], x: f64, y: f64) {
		if let Some(component) = self.get_component_from_chip_stack(stack) {
			component.set_position((x, y));
		}
	}

	/// Connects two components with a wire externally.
	/// Used to connect wires from JavaScript.
	pub fn connect_external(
		&mut self, comp1_idx: usize, pin1_idx: usize,
		comp2_idx: usize, pin2_idx: usize,
	) {
		self.connect((comp1_idx, pin1_idx), (comp2_idx, pin2_idx), vec![]);
	}
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Circuit {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox) {
		for wire in &self.wires {
			let con1 = &wire.pin1;
			let con2 = &wire.pin2;

			let comp1 = &self.components[con1.component_idx];
			let comp2 = &self.components[con2.component_idx];

			let c1 = comp1.get_position();
			let c2 = comp2.get_position();

			let p1 = comp1.get_pin_positions()[con1.pin_idx];
			let p2 = comp2.get_pin_positions()[con2.pin_idx];

			let start = (c1.0 + p1.0, c1.1 + p1.1);
			let end = (c2.0 + p2.0, c2.1 + p2.1);

			ctx.begin_path();
			ctx.move_to(start.0, start.1);

			let mut current_pos = start;

			for (idx, command) in wire.layout_commands.iter().enumerate() {
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
					WireLayoutCommand::Move((x, y)) => {
						current_pos.0 += x;
						current_pos.1 += y;
					},
					WireLayoutCommand::MoveXTo(x) => {
						current_pos.0 = *x;
					},
					WireLayoutCommand::MoveYTo(y) => {
						current_pos.1 = *y;
					},
					WireLayoutCommand::MoveTo((x, y)) => {
						current_pos = (*x, *y);
					},
					WireLayoutCommand::DontRenderPrevious => {},
				}

				if command != &WireLayoutCommand::DontRenderPrevious &&
					(idx == wire.layout_commands.len() - 1 || 
					wire.layout_commands[idx + 1] != WireLayoutCommand::DontRenderPrevious) {
					ctx.line_to(current_pos.0, current_pos.1);
				}
			}

			ctx.line_to(end.0, end.1);

			ctx.set_line_width(15.0);
			ctx.set_stroke_style(&"#000".into());
			ctx.stroke();

			ctx.set_line_width(7.0);
			let wire_state = wire.state1.combine(&wire.state2);
			ctx.set_stroke_style(&wire_state.get_colour().into());
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
}

/// The data needed to represent the inside of a [`Chip`].
pub struct ChipInternals {
	/// The [`Circuit`] in the [`Chip`].
	pub circuit: Circuit,
	/// The scale of the [`Circuit`]. A bigger value here means the circuit will appear bigger.
	pub inner_scale: f64,
}

impl Drawable for ChipInternals {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox) {
		ctx.save();
		ctx.scale(self.inner_scale, self.inner_scale).unwrap();

		self.circuit.draw(ctx, viewport);

		ctx.restore();
	}
}

/// A kind of [`Component`] that contains a [`Circuit`]. The behaviour of the chip is
/// governed by the behaviour of the circuit.
pub trait Chip {
	/// Returns the name of the chip as a string.
	fn get_chip_name(&self) -> String;

	/// Returns the chip internals.
	fn get_chip_internals(&self) -> &ChipInternals;

	/// Returns the mutable chip internals.
	fn get_chip_internals_mut(&mut self) -> &mut ChipInternals;

	/// Returns the position of the chip.
	fn get_chip_position(&self) -> (f64, f64);

	/// Sets the position of the chip.
	fn set_chip_position(&mut self, pos: (f64, f64));

	/// Returns the size of the chip.
	fn get_chip_size(&self) -> (f64, f64);

	/// Returns the text info for the chip.
	fn get_text_info(&self) -> Option<&TextInfo>;
	
	/// Return the current simulation mode.
	fn get_mode(&self) -> SimulationMode {
		SimulationMode::Circuit
	}
	
	/// Sets the simulation mode of the chip to the given mode.
	fn set_mode(&mut self, _mode: SimulationMode) {

	}

	/// Returns the state of a pin.
	fn get_pin_state_high_level(&self, _idx: usize) -> Result<PinState, PinError> {
		panic!("Unexpected get_pin_state_high_level");
	}

	/// Sets the state of a pin.
	fn set_pin_state_high_level(&mut self, _idx: usize, _state: PinState) -> Result<(), PinError> {
		panic!("Unexpected set_pin_state_high_level");
	}

	/// Returns whether the given viewport is fully contained within the chip.
	fn contains(&self, viewport: &BoundingBox) -> bool;

	/// Returns whether the given viewport is partially contained within the chip.
	fn intersects(&self, viewport: &BoundingBox) -> bool;

	/// Draws the front of the chip (the part that fades away when zooming in).
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d);

	/// Draws the edge of the chip.
	fn draw_edge(&self, _ctx: &web_sys::CanvasRenderingContext2d) {

	}

	/// Draws the back of the chip.
	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

impl<T: Chip> Drawable for T {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox) {
		self.draw_back(ctx);

		// TODO: Probably merge this with the other implementation!
		let start_ratio = 0.3;
		let end_ratio = 0.5;

		// let start_ratio = 0.0;
		// let end_ratio = 0.0;

		let height = self.get_size().1;
		let height_ratio = height / viewport.get_size().1;

		if self.intersects(&viewport) && height_ratio > start_ratio {
			let new_viewport = viewport.transform_in_to_chip(
				self.get_chip_position(),
				self.get_chip_internals(),
			);

			self.get_chip_internals().draw(ctx, new_viewport);
		}
		
		let opacity = ((end_ratio - height_ratio) / (end_ratio - start_ratio)).max(0.0);
		ctx.set_global_alpha(opacity);
		
		self.draw_front(ctx);
		
		ctx.set_global_alpha(1.0);
		self.draw_edge(ctx);
	}
}

impl<T: Chip> Component for T {
	fn get_name(&self) -> String {
		self.get_chip_name()
	}

	fn get_internals(&self) -> Option<&ChipInternals> {
		Some(self.get_chip_internals())
	}

	fn get_internals_mut(&mut self) -> Option<&mut ChipInternals> {
		Some(self.get_chip_internals_mut())
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		self.get_chip_internals().circuit.get_pin_positions()
			.iter()
			.map(|p| {
				let scale = self.get_chip_internals().inner_scale;
				(p.0 * scale, p.1 * scale)
			})
			.collect()
	}

	fn get_position(&self) -> (f64, f64) {
		self.get_chip_position()
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.set_chip_position(pos);
	}

	fn get_size(&self) -> (f64, f64) {
		self.get_chip_size()
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		match self.get_mode() {
			SimulationMode::Circuit => {
				let maybe_pin_component = self.get_chip_internals().circuit.components.iter()
					.filter(|c| c.is_pin())
					.nth(idx);
		
				match maybe_pin_component {
					Some(pin_component) => {
						pin_component.get_pin_state_external(0)
					},
					None => Err(PinError::OutOfRange),
				}
			},
			SimulationMode::HighLevel => self.get_pin_state_high_level(idx),
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		match self.get_mode() {
			SimulationMode::Circuit => {
				let maybe_component_idx = self.get_chip_internals_mut().circuit.components.iter_mut()
					.enumerate()
					.filter(|(_, c)| c.is_pin())
					.nth(idx)
					.map(|(i, _)| i);

				match maybe_component_idx {
					Some(component_idx) => {
						let connection = ExternalPin {
							component_idx,
							pin_idx: 0,
						};
						self.get_chip_internals_mut().circuit.update_component(&connection, state, true);
						Ok(())
					},
					None => Err(PinError::OutOfRange),
				}
			},
			SimulationMode::HighLevel => self.set_pin_state_high_level(idx, state),
		}
	}
	
	fn set_mode(&mut self, mode: SimulationMode) {
		self.set_mode(mode);
	}

	fn contains(&self, viewport: &BoundingBox) -> bool {
		self.contains(viewport)
	}

	fn intersects(&self, viewport: &BoundingBox) -> bool {
		self.intersects(viewport)
	}
}

/// Information about text to be rendered on the front of a [`Chip`].
pub struct TextInfo {
	/// The text.
	pub text: String,
	/// The size of the text.
	pub size: u32,
}

/// A [`Chip`] that looks like a rectangle.
pub trait RectangleChip {
	/// Returns the name of the chip as a string.
	fn get_chip_name(&self) -> String;

	/// Returns the chip internals.
	fn get_chip_internals(&self) -> &ChipInternals;

	/// Returns the mutable chip internals.
	fn get_chip_internals_mut(&mut self) -> &mut ChipInternals;

	/// Returns the position of the chip.
	fn get_chip_position(&self) -> (f64, f64);

	/// Sets the position of the chip.
	fn set_chip_position(&mut self, pos: (f64, f64));

	/// Returns the size of the chip.
	fn get_chip_size(&self) -> (f64, f64);

	/// Returns the text info for the chip.
	fn get_text_info(&self) -> Option<&TextInfo>;
	
	/// Return the current simulation mode.
	fn get_mode(&self) -> SimulationMode;
	
	/// Sets the simulation mode of the chip to the given mode.
	fn set_mode(&mut self, mode: SimulationMode);

	/// Returns the state of a pin.
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError>;

	/// Sets the state of a pin.
	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError>;
}

impl<T: RectangleChip> Chip for T {
	fn get_chip_name(&self) -> String {
		self.get_chip_name()
	}

	fn get_chip_internals(&self) -> &ChipInternals {
		self.get_chip_internals()
	}
	
	fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
		self.get_chip_internals_mut()
	}

	fn get_chip_position(&self) -> (f64, f64) {
		self.get_chip_position()
	}

	fn set_chip_position(&mut self, pos: (f64, f64)) {
		self.set_chip_position(pos);
	}

	fn get_chip_size(&self) -> (f64, f64) {
		self.get_chip_size()
	}

	fn get_text_info(&self) -> Option<&TextInfo> {
		self.get_text_info()
	}
	
	fn get_mode(&self) -> SimulationMode {
		self.get_mode()
	}

	fn set_mode(&mut self, mode: SimulationMode) {
		self.set_mode(mode)
	}

	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		self.get_pin_state_high_level(idx)
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		self.set_pin_state_high_level(idx, state)
	}

	fn contains(&self, viewport: &BoundingBox) -> bool {
		let position = self.get_position();
		let size = self.get_size();

		let contains_x =
			position.0 + size.0 * 0.5 >= viewport.get_position().0 + viewport.get_size().0 * 0.5 &&
			position.0 - size.0 * 0.5 <= viewport.get_position().0 - viewport.get_size().0 * 0.5;

		let contains_y =
			position.1 + size.1 * 0.5 >= viewport.get_position().1 + viewport.get_size().1 * 0.5 &&
			position.1 - size.1 * 0.5 <= viewport.get_position().1 - viewport.get_size().1 * 0.5;

		contains_x && contains_y
	}

	fn intersects(&self, viewport: &BoundingBox) -> bool {
		let position = self.get_position();
		let size = self.get_size();

		let intersects_x =
			position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
	}

	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		// match self.get_mode() {
		// 	SimulationMode::Circuit => ctx.set_fill_style(&"#000".into()),
		// 	SimulationMode::HighLevel => ctx.set_fill_style(&"#f00".into()),
		// }

		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = self.get_size();

		ctx.begin_path();
		ctx.rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill();

		if let Some(info) = &self.get_text_info() {
			ctx.set_fill_style(&"#fff".into());
			ctx.set_font(format!("bold {}px monospace", info.size).as_str());
			ctx.set_text_align("center");
			ctx.set_text_baseline("middle");

			ctx.fill_text(info.text.as_str(), 0.0, 0.0).unwrap();
		}
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());
		
		// match self.get_mode() {
		// 	SimulationMode::Circuit => ctx.set_fill_style(&"#000".into()),
		// 	SimulationMode::HighLevel => ctx.set_fill_style(&"#f00".into()),
		// }

		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = self.get_size();

		ctx.begin_path();
		ctx.rect(-width * 0.5, -height * 0.5, width, height);

		ctx.stroke();
		ctx.fill();
	}
}

// pub struct RectangleChip {
// 	/// The chip internals.
// 	pub internals: ChipInternals,
// 	/// The position of the chip.
// 	pub position: (f64, f64),
// 	/// The size of the chip.
// 	pub size: (f64, f64),
// 	/// The text on the chip, if any.
// 	pub text: Option<TextInfo>,
// }

/// An "internal" pin. Used for connecting an internal [`Circuit`] to a [`Chip`].
/// 
/// Internal pins have two states since we need to be able to get the state of an input pin
/// from within the [`Circuit`] without it leaking to the outside of the [`Chip`].
pub struct Pin {
	/// The position of the pin.
	position: (f64, f64),
	/// The state accessible by the [`Circuit`].
	inner_state: PinState,
	/// The state accessible by the [`Chip`].
	outer_state: PinState,
}

impl Pin {
	/// Returns a new pin.
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			inner_state: PinState::Disconnected,
			outer_state: PinState::Disconnected,
		}
	}
}

impl Drawable for Pin {
	fn draw(&self, _ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox) {
		
	}
}

impl Component for Pin {
	fn get_name(&self) -> String {
		String::from("Pin")
	}

	fn is_pin(&self) -> bool {
		true
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![(0.0, 0.0)]
	}
	
	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_size(&self) -> (f64, f64) {
		(0.0, 0.0)
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.inner_state)
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			self.outer_state = state;
			Ok(())
		}
	}

	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.outer_state)
		}
	}

	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			self.inner_state = state;
			Ok(())
		}
	}
}

/// A switch that can be turned on and off.
pub struct Switch {
	position: (f64, f64),
	state: PinState,
}

impl Switch {
	/// Returns a new switch.
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			state: PinState::Off,
		}
	}
}

impl Drawable for Switch {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox) {
		ctx.set_fill_style(&self.state.get_colour().into());

		let (width, height) = self.get_size();

		ctx.fill_rect(
			-width * 0.5,
			-height * 0.5,
			width,
			height,
		);
	}
}

impl Component for Switch {
	fn get_name(&self) -> String {
		String::from("Switch")
	}

	fn get_switch_count(&self) -> usize {
		1
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![(0.0, 0.0)]
	}
	
	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_size(&self) -> (f64, f64) {
		(100.0, 100.0)
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.state)
		}
	}

	fn set_pin_state(&mut self, idx: usize, _state: PinState) -> Result<(), PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(())
		}
	}

	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			self.state = state;
			Ok(())
		}
	}
}

/// A collection of switches that can be turned on and off.
pub struct MultiSwitch {
	position: (f64, f64),
	size: usize,
	states: Vec<PinState>,
}

impl MultiSwitch {
	/// Returns a new set of switches.
	pub fn new(pos: (f64, f64), size: usize) -> Self {
		Self {
			position: pos,
			size,
			states: vec![PinState::Off; size],
		}
	}
}

impl Drawable for MultiSwitch {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox) {
		ctx.set_line_width(10.0);

		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		let size = self.states.len();

		let (width, height) = self.get_size();

		ctx.stroke_rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill_rect(-width * 0.5, -height * 0.5, width, height);
		
		ctx.set_fill_style(&"#fff".into());
		ctx.set_font("bold 70px monospace");
		ctx.set_text_align("center");
		ctx.set_text_baseline("middle");
		
		let num = states_to_num(&self.states);

		ctx.fill_text(format!("{}", num).as_str(), 0.0, -height * 0.1).unwrap();

		for i in 0..size {
			ctx.set_fill_style(&self.states[i].get_colour().into());

			let extra_width = if i == size-1 { 0.0 } else { 1.0 };
			ctx.fill_rect((i as f64 - size as f64 * 0.5) * 50.0, height * 0.5 - 50.0, 50.0 + extra_width, 50.0);
		}
	}
}

impl Component for MultiSwitch {
	fn get_name(&self) -> String {
		String::from("MultiSwitch")
	}

	fn get_switch_count(&self) -> usize {
		self.states.len()
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		let spacing = 50.0;
		let size = self.states.len();

		(0..size)
			.map(|i| (i as f64 - size as f64 * 0.5 + 0.5) * spacing)
			.map(|x| (x, 100.0))
			.collect()
	}
	
	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}
	
	fn get_size(&self) -> (f64, f64) {
		let width = 50.0 * self.size as f64;
		let height = 200.0;
		(width, height)
	}
	
	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.states[idx])
		}
	}

	fn set_pin_state(&mut self, idx: usize, _state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(())
		}
	}

	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			self.states[idx] = state;
			Ok(())
		}
	}
}

/// A bulb that shows a single state.
pub struct Bulb {
	position: (f64, f64),
	state: PinState,
}

impl Bulb {
	/// Returns a new bulb.
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			state: PinState::Disconnected,
		}
	}
}

impl Drawable for Bulb {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox) {
		ctx.set_fill_style(&self.state.get_colour().into());
		
		let radius = 50.0;

		ctx.begin_path();
		ctx.arc(
			0.0,
			0.0,
			radius,
			0.0,
			2.0 * PI,
		).unwrap();
		ctx.fill();
	}
}

impl Component for Bulb {
	fn get_name(&self) -> String {
		String::from("Bulb")
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![(0.0, 0.0)]
	}

	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_size(&self) -> (f64, f64) {
		(100.0, 100.0)
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			self.state = state;
			Ok(())
		}
	}
}

/// A collection of bulbs.
pub struct MultiBulb {
	position: (f64, f64),
	size: usize,
	states: Vec<PinState>,
}

impl MultiBulb {
	/// Returns a new set of bulbs.
	pub fn new(pos: (f64, f64), size: usize) -> Self {
		Self {
			position: pos,
			size,
			states: vec![PinState::Disconnected; size],
		}
	}
}

impl Drawable for MultiBulb {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox) {
		ctx.set_line_width(10.0);

		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		let size = self.states.len();

		let (width, height) = self.get_size();

		ctx.stroke_rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill_rect(-width * 0.5, -height * 0.5, width, height);
		
		ctx.set_fill_style(&"#fff".into());
		ctx.set_font("bold 70px monospace");
		ctx.set_text_align("center");
		ctx.set_text_baseline("middle");
		
		let num = states_to_num(&self.states);

		ctx.fill_text(format!("{}", num).as_str(), 0.0, -height * 0.1).unwrap();

		for i in 0..size {
			ctx.set_fill_style(&self.states[i].get_colour().into());

			ctx.begin_path();
			ctx.arc((i as f64 - size as f64 * 0.5) * 50.0 + 25.0, height * 0.5 - 25.0, 20.0, 0.0, 2.0 * PI).unwrap();
			ctx.fill();
		}
	}
}

impl Component for MultiBulb {
	fn get_name(&self) -> String {
		String::from("MultiBulb")
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		let spacing = 50.0;
		let size = self.states.len();

		(0..size)
			.map(|i| (i as f64 - size as f64 * 0.5 + 0.5) * spacing)
			.map(|x| (x, 100.0))
			.collect()
	}

	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_size(&self) -> (f64, f64) {
		let width = 50.0 * self.size as f64;
		let height = 200.0;
		(width, height)
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			self.states[idx] = state;
			Ok(())
		}
	}
}

/// A [`Component`] for splitting a signal onto many different wires.
/// Any [`Wire`] at a junction can act as either input or output at any time.
pub struct Junction {
	position: (f64, f64),
	states: Vec<PinState>,
}

impl Junction {
	/// Returns a new junction.
	pub fn new(pos: (f64, f64), pin_count: usize) -> Self {
		Self {
			position: pos,
			states: vec![PinState::Disconnected; pin_count],
		}
	}

	/// Returns the output state of the junction.
	fn get_state(&self) -> PinState {
		*self.states.iter().reduce(|accum, state| accum.combine(state)).unwrap()
	}
}

impl Drawable for Junction {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox) {
		ctx.set_fill_style(&self.get_state().get_colour().into());
		
		let radius = 10.0;

		ctx.begin_path();
		ctx.arc(
			0.0,
			0.0,
			radius,
			0.0,
			2.0 * PI,
		).unwrap();
		ctx.fill();
	}
}

impl Component for Junction {
	fn get_name(&self) -> String {
		String::from("Junction")
	}

	fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		vec![(0.0, 0.0); self.states.len()]
	}

	fn get_position(&self) -> (f64, f64) {
		self.position
	}

	fn set_position(&mut self, pos: (f64, f64)) {
		self.position = pos;
	}

	fn get_size(&self) -> (f64, f64) {
		(20.0, 20.0)
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else if self.states[idx] == PinState::Disconnected {
			Ok(self.get_state())
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			self.states[idx] = state;
			Ok(())
		}
	}
}
