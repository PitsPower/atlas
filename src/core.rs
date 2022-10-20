use std::f64::consts::PI;

use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::graphics::{Drawable, Viewport, WireLayoutCommand};
use crate::log;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PinState {
	On,
	Off,
	Disconnected,
}

impl PinState {
	fn combine<'a>(&'a self, other: &'a PinState) -> &'a PinState {
		if self == &PinState::Disconnected {
			other
		} else {
			self
		}
	}

	fn toggle(&self) -> PinState {
		match *self {
			PinState::On => PinState::Off,
			PinState::Off => PinState::On,
			PinState::Disconnected => PinState::Disconnected,
		}
	}

    fn get_colour(&self) -> &str {
        match self {
            PinState::On => "#fb016e",
            PinState::Off => "#333",
            PinState::Disconnected => "#999",
        }
    }
}

#[derive(Debug)]
pub enum GetPinError {
	OutOfRange,
	Shorted,
}

#[derive(Debug)]
pub enum SetPinError {
	OutOfRange,
}

pub trait Component: Drawable {
	fn as_chip(&self) -> Option<&Chip> {
		None
	}

	fn is_switch(&self) -> bool {
		false
	}

	fn get_pin_count(&self) -> usize {
		self.get_pin_positions().len()
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError>;
	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError>;

	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		self.set_pin_state(idx, state)
	}

	fn get_position(&self) -> (f64, f64);

	fn contains(&self, _viewport: &Viewport) -> bool {
		false
	}

	fn intersects(&self, _viewport: &Viewport) -> bool {
		true
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PinConnection {
	component_idx: usize,
	pin_idx: usize,
}

#[derive(Debug)]
struct Wire {
	start_con: PinConnection,
	end_con: PinConnection,
	layout_commands: Vec<WireLayoutCommand>,
	start_state: PinState,
	end_state: PinState,
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
		let start_state = match self.components[comp1_idx].get_pin_state(pin1_idx) {
			Ok(state) => state,
			Err(GetPinError::OutOfRange) => panic!("Pin index {} out of range for component {}", pin1_idx, comp1_idx),
			Err(GetPinError::Shorted) => todo!(),
		};

		let end_state = match self.components[comp2_idx].get_pin_state(pin2_idx) {
			Ok(state) => state,
			Err(GetPinError::OutOfRange) => panic!("Pin index {} out of range for component {}", pin1_idx, comp1_idx),
			Err(GetPinError::Shorted) => todo!(),
		};

		let start_con = PinConnection { component_idx: comp1_idx, pin_idx: pin1_idx };
		let end_con = PinConnection { component_idx: comp2_idx, pin_idx: pin2_idx };

		if start_state == PinState::Disconnected && end_state != PinState::Disconnected {
			self.update_component(&start_con, end_state, false);
		}
		else if end_state == PinState::Disconnected && start_state != PinState::Disconnected {
			self.update_component(&end_con, start_state, false);
		}

		self.wires.push(Wire {
			start_con,
			end_con,
			layout_commands: wire_commands,
			start_state,
			end_state,
		});
	}

	fn update_component(&mut self, pin: &PinConnection, state: PinState, set_manually: bool) {
		// log!("{:?}", pin);

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

		for i in 0..component.get_pin_count() {
			if i == pin.pin_idx && !set_manually {
				continue;
			}

			let con = PinConnection { component_idx: pin.component_idx, pin_idx: i };
			let state = component.get_pin_state(i).unwrap();

			if let Some((wire_idx, wire)) = self.wires.iter().enumerate()
				.find(|(_, w)| w.start_con == con || w.end_con == con)
			{
				// log!("{:?}", wire);

				if wire.start_con == con {
					wire_starts_to_update.push((wire_idx, state));

					if wire.end_state == PinState::Disconnected {
						components_to_update.push((wire.end_con, state));
					} else if state == PinState::Disconnected && old_pin_states[i] != PinState::Disconnected {
						components_to_update.push((wire.start_con, wire.end_state));
					}
				} else {
					wire_ends_to_update.push((wire_idx, state));
					
					if wire.start_state == PinState::Disconnected {
						components_to_update.push((wire.start_con, state));
					} else if state == PinState::Disconnected && old_pin_states[i] != PinState::Disconnected {
						components_to_update.push((wire.end_con, wire.start_state));
					}
				}
			}
		}
		
		for (idx, state) in wire_starts_to_update {
			self.wires[idx].start_state = state;
		}
		for (idx, state) in wire_ends_to_update {
			self.wires[idx].end_state = state;
		}
		for (con, state) in components_to_update {
			self.update_component(&con, state, false);
		}
	}
}

#[wasm_bindgen]
impl Circuit {
	pub fn toggle_switch(&mut self, idx: usize) {
		let true_idx = self.components.iter()
			.enumerate()
			.filter(|(_, c)| c.is_switch())
			.nth(idx)
			.unwrap().0;

		let state = self.components[true_idx].get_pin_state(0).unwrap();
		self.update_component(&PinConnection { component_idx: true_idx, pin_idx: 0 }, state.toggle(), true);
	}
}

impl Drawable for Circuit {
	fn draw(&self, ctx: &CanvasRenderingContext2d, viewport: Viewport) {
		for wire in &self.wires {
			let con1 = &wire.start_con;
			let con2 = &wire.end_con;

			let comp1 = &self.components[con1.component_idx];
			let comp2 = &self.components[con2.component_idx];

			let c1 = comp1.get_position();
			let c2 = comp2.get_position();

			let p1 = comp1.get_pin_positions()[con1.pin_idx];
			let p2 = comp2.get_pin_positions()[con2.pin_idx];

			let start = (c1.0 + p1.0, c1.1 + p1.1);
			let end = (c2.0 + p2.0, c2.1 + p2.1);

			ctx.set_line_width(7.0);

			let wire_state = wire.start_state.combine(&wire.end_state);

			ctx.set_stroke_style(&wire_state.get_colour().into());

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

	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError> {
		todo!();
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		todo!();
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

pub struct Switch {
	position: (f64, f64),
	state: PinState,
}

impl Switch {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			state: PinState::Off,
		}
	}
}

impl Drawable for Switch {
    fn draw(&self, ctx: &CanvasRenderingContext2d, _viewport: Viewport) {
        ctx.set_fill_style(&self.state.get_colour().into());

		let width = 100.0;
		let height = 100.0;

		ctx.fill_rect(
			-width * 0.5,
			-height * 0.5,
			width,
			height,
		);
    }

    fn get_pin_positions(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0)]
    }
}

impl Component for Switch {
	fn is_switch(&self) -> bool {
		true
	}

	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError> {
		if idx > 0 {
			Err(GetPinError::OutOfRange)
		} else {
			Ok(self.state)
		}
	}

	fn set_pin_state(&mut self, idx: usize, _state: PinState) -> Result<(), SetPinError> {
		if idx > 0 {
			Err(SetPinError::OutOfRange)
		} else {
			Ok(())
		}
	}

	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		if idx > 0 {
			Err(SetPinError::OutOfRange)
		} else {
			self.state = state;
			Ok(())
		}
	}
	
    fn get_position(&self) -> (f64, f64) {
        self.position
    }
}

pub struct Bulb {
	position: (f64, f64),
	state: PinState,
}

impl Bulb {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
			state: PinState::Disconnected,
		}
	}
}

impl Drawable for Bulb {
    fn draw(&self, ctx: &CanvasRenderingContext2d, _viewport: Viewport) {
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

    fn get_pin_positions(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0)]
    }
}

impl Component for Bulb {
	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError> {
		if idx > 0 {
			Err(GetPinError::OutOfRange)
		} else {
			Ok(self.state)
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		if idx > 0 {
			Err(SetPinError::OutOfRange)
		} else {
			self.state = state;
			Ok(())
		}
	}

    fn get_position(&self) -> (f64, f64) {
        self.position
    }
}

pub struct Junction {
	position: (f64, f64),
	states: Vec<PinState>,
}

impl Junction {
	pub fn new(pos: (f64, f64), pin_count: usize) -> Self {
		Self {
			position: pos,
			states: vec![PinState::Disconnected; pin_count],
		}
	}

	fn get_state(&self) -> PinState {
		*self.states.iter().reduce(|accum, state| accum.combine(state)).unwrap()
	}
}

impl Drawable for Junction {
    fn draw(&self, ctx: &CanvasRenderingContext2d, _viewport: Viewport) {
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

    fn get_pin_positions(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0); self.states.len()]
    }
}

impl Component for Junction {
	fn get_pin_state(&self, idx: usize) -> Result<PinState, GetPinError> {
		// log!("{:?}", self.states);

		if idx >= self.states.len() {
			Err(GetPinError::OutOfRange)
		} else {
			Ok(self.get_state())
		}
	}

	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), SetPinError> {
		if idx >= self.states.len() {
			Err(SetPinError::OutOfRange)
		} else {
			self.states[idx] = state;
			Ok(())
		}
	}

    fn get_position(&self) -> (f64, f64) {
        self.position
    }
}
