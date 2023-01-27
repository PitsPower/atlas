//! Core ATLAS functionality.
//! 
//! Provides core data structures such as [`Circuit`] and some basic components like switches, bulbs and junctions.

use std::f64::consts::PI;

use wasm_bindgen::prelude::*;

use crate::add;
use crate::adder::*;
use crate::bus::{BusLayoutCommand, compute_wire_commands};
use crate::control::*;
use crate::gates::*;
use crate::graphics::{
	BoundingBox, ComponentDrawer, Drawable, NothingDrawer,
	RectangleChipDrawer, TextInfo, WireLayoutCommand,
};
use crate::latches::*;
use crate::memory::*;
use crate::multiplexer::*;
use crate::register::*;
use crate::transistor::*;
use crate::utils::{Lazy, states_to_num};

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
	/// but with the junction component it behaves differently.
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

/// An error that may occur when getting or setting a pin.
#[derive(Debug)]
pub enum PinError {
	/// The pin index used is too large for the component.
	OutOfRange,
}

/// How a chip should be simulated.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SimulationMode {
	/// Simulate the circuit in the chip.
	Circuit,
	/// Simulate the chip using the high level implementation.
	HighLevel,
}

/// Options for various components (e.g. size of the component).
#[derive(Clone, Copy)]
pub struct ComponentOptions {
	/// The size of the component (e.g. 8-bit, 32-bit, etc.).
	pub size: usize,
	/// Whether the multi-junction should be flipped so that it goes
	/// diagonally up instead of diagonally down.
	pub should_flip_multi_junction: bool,
}

/// The different kinds of component.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentType {
	/// A bulb that shows a single state.
	Bulb,
	/// A [`Component`] for splitting a signal onto many different wires.
	/// Any [`Wire`] at a junction can act as either input or output at any time.
	Junction,
	/// An "internal" pin. Used for connecting an internal [`Circuit`] to a chip.
	/// 
	/// Internal pins have two states since we need to be able to get the state of an input pin
	/// from within the [`Circuit`] without it leaking to the outside of the chip.
	Pin,
	/// A switch that can be turned on and off.
	Switch,
	
	NTransistor,
	PTransistor,

	AndGate,
	NandGate,
	NorGate,
	NotGate,
	OrGate,
	TriStateBuffer,
	XorGate,

	/// A collection of bulbs that can be turned on and off.	
	MultiBulb,
	/// A collection of junctions.
	MultiJunction,
	/// A collection of switches that can be turned on and off.	
	MultiSwitch,

	MultiTriStateBuffer,

	HalfAdder,
	FullAdder,
	Adder,

	SRLatch,
	DLatch,
	DFlipFlop,
	MultiDFlipFlop,

	Multiplexer,
	MultiMultiplexer,
	FourWayMultiMultiplexer,

	Register,

	Rom,
	Memory,
	Ram,

	Counter,
	ControlUnit,
}

macro_rules! chip {
	($func:expr, $scale:expr) => {
		ComponentInternals::Chip(Lazy::new(Box::new($func)), $scale)
	};
}

impl ComponentType {
	/// Returns the value of the component type as a string.
	fn as_string(&self) -> &'static str {
		match self {
			Self::Bulb => "Bulb",
			Self::Junction => "Junction",
			Self::Pin => "Pin",
			Self::Switch => "Switch",

			Self::NTransistor => "NTransistor",
			Self::PTransistor => "PTransistor",

			Self::AndGate => "AndGate",
			Self::NandGate => "NandGate",
			Self::NorGate => "NorGate",
			Self::NotGate => "NotGate",
			Self::OrGate => "OrGate",
			Self::TriStateBuffer => "TriStateBuffer",
			Self::XorGate => "XorGate",

			Self::MultiBulb => "MultiBulb",
			Self::MultiJunction => "MultiJunction",
			Self::MultiSwitch => "MultiSwitch",
			
			Self::MultiTriStateBuffer => "MultiTriStateBuffer",

			Self::HalfAdder => "Half Adder",
			Self::FullAdder => "Full Adder",
			Self::Adder => "Adder",

			Self::SRLatch => "SRLatch",
			Self::DLatch => "DLatch",
			Self::DFlipFlop => "DFlipFlop",
			Self::MultiDFlipFlop => "MultiDFlipFlop",

			Self::Multiplexer => "Multiplexer",
			Self::MultiMultiplexer => "MultiMultiplexer",
			Self::FourWayMultiMultiplexer => "FourWayMultiMultiplexer",

			Self::Register => "Register",
			
			Self::Rom => "Rom",
			Self::Memory => "Memory",
			Self::Ram => "Ram",
			
			Self::Counter => "Counter",
			Self::ControlUnit => "ControlUnit",
		}
	}

	/// Returns whether or not the component has switches.
	fn has_switches(&self) -> bool {
		matches!(self, Self::MultiSwitch | Self::Switch)
	}

	/// Whether the circuit should become full-sized when zooming in.
	/// This should be disabled for components that have weird shapes.
	fn should_expand_circuit(&self) -> bool {
		!matches!(
			self,
			Self::AndGate | Self::NandGate | Self::NorGate |
			Self::NotGate | Self::OrGate | Self::TriStateBuffer |
			Self::XorGate
		)
	}

	/// Whether the circuit should be padded horizontally instead of vertically.
	fn should_pad_horizontally(&self) -> bool {
		matches!(self, Self::Register | Self::Ram | Self::Counter | Self::ControlUnit)
	}

	/// The top/bottom padding size.
	fn get_pad_size(&self) -> f64 {
		match self {
			Self::Counter => 1000.0,
			_ => 100.0,
		}
	}

	/// Returns a new [`Component`] of the given type.
	pub fn create(&self, position: (f64, f64), options: ComponentOptions) -> Component {
		let transistor_width = 67.0;
		let transistor_height = 110.0;

		let mut internals = match self {
			Self::Bulb | Self::Pin | Self::Switch => {
				ComponentInternals::Atomic(vec![(0.0, 0.0)])
			},

			Self::Junction => ComponentInternals::Atomic(vec![(0.0, 0.0); options.size]),

			Self::NTransistor => {
				ComponentInternals::Atomic(vec![
					(-transistor_width * 0.5 - 15.0, 0.0),
					(transistor_width * 0.5, transistor_height * 0.5),
					(transistor_width * 0.5, -transistor_height * 0.5),
				])
			},
			Self::PTransistor => {
				ComponentInternals::Atomic(vec![
					(-transistor_width * 0.5 - 15.0, 0.0),
					(transistor_width * 0.5, -transistor_height * 0.5),
					(transistor_width * 0.5, transistor_height * 0.5),
				])
			},

			Self::AndGate => chip!(get_and_gate_circuit, 0.15),
			Self::NandGate => chip!(get_nand_gate_circuit, 0.07),
			Self::NorGate => chip!(get_nor_gate_circuit, 0.07),
			Self::NotGate => chip!(get_not_gate_circuit, 0.07),
			Self::OrGate => chip!(get_or_gate_circuit, 0.15),
			Self::TriStateBuffer => chip!(get_tri_state_buffer_circuit, 0.04),
			Self::XorGate => chip!(get_xor_gate_circuit, 0.15),
			
			Self::MultiBulb | Self::MultiSwitch => {
				let spacing = 50.0;
				let size = options.size;
		
				let pin_positions = (0..size)
					.map(|i| (i as f64 - size as f64 * 0.5 + 0.5) * spacing)
					.map(|x| (x, 100.0))
					.collect();

				ComponentInternals::Atomic(pin_positions)
			},
			Self::MultiJunction => {
				let spacing = 30.0;
				let size = options.size;
		
				let pin_positions = (0..size)
					.map(|i| (i as f64 - size as f64 * 0.5 + 0.5) * spacing)
					.flat_map(|x| {
						let y = if options.should_flip_multi_junction {
							-x
						} else {
							x
						};

						[(x, y); 3]
					})
					.collect();

				ComponentInternals::Atomic(pin_positions)
			},

			Self::MultiTriStateBuffer => chip!(move || get_multi_tri_state_buffer_circuit(options.size), 0.3),

			Self::HalfAdder => chip!(get_half_adder_circuit, 0.4),
			Self::FullAdder => chip!(get_full_adder_circuit, 0.4),
			Self::Adder => chip!(move || get_adder_circuit(options.size), 0.3),
			
			Self::SRLatch => chip!(get_sr_latch_circuit, 0.8),
			Self::DLatch => chip!(get_d_latch_circuit, 0.5),
			Self::DFlipFlop => chip!(get_d_flip_flop_circuit, 0.3),
			Self::MultiDFlipFlop => chip!(move || get_multi_d_flip_flop_circuit(options.size), 0.19),

			Self::Multiplexer => chip!(get_multiplexer_circuit, 0.4),
			Self::MultiMultiplexer => chip!(move || get_multi_multiplexer_circuit(options.size), 0.2),
			Self::FourWayMultiMultiplexer => chip!(move || get_four_way_circuit(options.size), 0.3),

			Self::Register => chip!(get_register_circuit, 0.2),
			
			Self::Rom => {
				let inner_scale = 0.2;
				chip!(move || get_rom_circuit(options.size, inner_scale), inner_scale)
			},
			Self::Memory => {
				let inner_scale = 0.2;
				chip!(move || get_memory_circuit(options.size, inner_scale), inner_scale)
			},
			Self::Ram => chip!(get_ram_circuit, 0.2),

			Self::Counter => chip!(get_counter_circuit, 0.2),
			Self::ControlUnit => chip!(get_control_unit_circuit, 0.2),
		};

		let size = match self {
			Self::Pin => (0.0, 0.0),
			Self::Junction => (50.0, 50.0),
			Self::Bulb | Self::Switch => (100.0, 100.0),

			Self::NTransistor | Self::PTransistor => (transistor_width, transistor_height),

			Self::AndGate | Self::NandGate | Self::NorGate | Self::NotGate |
			Self::OrGate | Self::TriStateBuffer | Self::XorGate => (110.0, 110.0),

			Self::HalfAdder => (200.0, 200.0),
			Self::FullAdder => (400.0, 200.0),
			Self::Adder => (800.0, options.size as f64 * 100.0),
			
			Self::SRLatch => (400.0, 400.0),
			Self::DLatch => (600.0, 400.0),
			Self::DFlipFlop => (600.0, 400.0),
			Self::MultiDFlipFlop => (400.0, options.size as f64 * 100.0),

			Self::MultiBulb | Self::MultiSwitch => {
				let width = 50.0 * options.size as f64;
				let height = 200.0;
				(width, height)
			},
			Self::MultiJunction => {
				let size = 30.0 * options.size as f64;
				(size, size)
			},

			Self::Rom => (700.0, 500.0),
			Self::Memory => (900.0, 700.0),

			_ => {
				let inner_scale = internals.get_inner_scale().unwrap();
				let circuit = internals.get_circuit_mut().unwrap();

				let mut min_x = f64::INFINITY;
				let mut min_y = f64::INFINITY;
				let mut max_x = f64::NEG_INFINITY;
				let mut max_y = f64::NEG_INFINITY;

				for component in &circuit.components {
					if component.is_pin() {
						let pos = component.position;
						
						if min_x > pos.0 {
							min_x = pos.0;
						}
						if min_y > pos.1 {
							min_y = pos.1;
						}
						if max_x < pos.0 {
							max_x = pos.0;
						}
						if max_y < pos.1 {
							max_y = pos.1;
						}
					}
				}

				let mut chip_size = (
					(max_x - min_x) * inner_scale,
					(max_y - min_y) * inner_scale,
				);

				let mut circuit_offset = (
					-(max_x - min_x) * 0.5 - min_x,
					-(max_y - min_y) * 0.5 - min_y,
				);

				if self.should_pad_horizontally() {
					let mut min_side_x = f64::INFINITY;
					let mut max_side_x = f64::NEG_INFINITY;

					for component in &circuit.components {
						if component.is_pin() {
							let pos = component.position;

							if pos.1 == min_y || pos.1 == max_y {
								if min_side_x > pos.0 {
									min_side_x = pos.0;
								}
								if max_side_x < pos.0 {
									max_side_x = pos.0;
								}
							}
						}
					}

					let left_pad = min_x - min_side_x;
					let right_pad = max_x - max_side_x;

					let pad_diff = right_pad - left_pad;
					chip_size.0 += pad_diff * inner_scale;

					if right_pad > -left_pad {
						circuit_offset.0 += pad_diff * 0.5;
					} else {
						circuit_offset.0 -= pad_diff * 0.5;
					}

					if left_pad == 0.0 && right_pad == 0.0 {
						chip_size.0 += self.get_pad_size();
					}
				} else {
					let mut min_side_y = f64::INFINITY;
					let mut max_side_y = f64::NEG_INFINITY;

					for component in &circuit.components {
						if component.is_pin() {
							let pos = component.position;

							if pos.0 == min_x || pos.0 == max_x {
								if min_side_y > pos.1 {
									min_side_y = pos.1;
								}
								if max_side_y < pos.1 {
									max_side_y = pos.1;
								}
							}
						}
					}

					let top_pad = min_y - min_side_y;
					let bottom_pad = max_y - max_side_y;

					let pad_diff = bottom_pad - top_pad;
					chip_size.1 += pad_diff * inner_scale;

					if bottom_pad > -top_pad {
						circuit_offset.1 += pad_diff * 0.5;
					} else {
						circuit_offset.1 -= pad_diff * 0.5;
					}

					if top_pad == 0.0 && bottom_pad == 0.0 {
						chip_size.1 += self.get_pad_size();
					}
				}

				for component in &mut circuit.components {
					component.position.0 += circuit_offset.0;
					component.position.1 += circuit_offset.1;
				}

				for wire in &mut circuit.wires {
					if let Some(commands) = wire.layout_commands.try_get_mut() {
						for command in commands {
							match command {
								WireLayoutCommand::MoveXTo(x) => {
									*command = WireLayoutCommand::MoveXTo(*x + circuit_offset.0);
								},
								WireLayoutCommand::MoveYTo(y) => {
									*command = WireLayoutCommand::MoveYTo(*y + circuit_offset.1);
								},
								WireLayoutCommand::MoveTo((x, y)) => {
									*command = WireLayoutCommand::MoveTo((
										*x + circuit_offset.0,
										*y + circuit_offset.1,
									));
								},
								_ => {},
							};
						}
					}
				}

				chip_size
			},
		};

		let simulator: Option<Box<dyn ComponentSimulator>> = match self {
			Self::Bulb => Some(Box::new(BulbSimulator::new())),
			Self::Junction => Some(Box::new(JunctionSimulator::new(options.size))),
			Self::Pin => Some(Box::new(PinSimulator::new())),
			Self::Switch => Some(Box::new(SwitchSimulator::new())),

			Self::NTransistor => Some(Box::new(NTransistorSimulator::new())),
			Self::PTransistor => Some(Box::new(PTransistorSimulator::new())),

			Self::MultiBulb => Some(Box::new(MultiBulbSimulator::new(options.size))),
			Self::MultiJunction => Some(Box::new(MultiJunctionSimulator::new(options.size))),
			Self::MultiSwitch => Some(Box::new(MultiSwitchSimulator::new(options.size))),
			
			Self::Rom => Some(Box::new(RomSimulator::new(options.size))),
			Self::Memory => Some(Box::new(MemorySimulator::new(options.size))),
			
			_ => None,
		};

		let drawer: Box<dyn ComponentDrawer> = match self {
			Self::Bulb => Box::new(BulbDrawer::new()),
			Self::Junction => Box::new(JunctionDrawer::new()),
			Self::Pin => Box::new(NothingDrawer::new()),
			Self::Switch => Box::new(SwitchDrawer::new()),

			Self::NTransistor => Box::new(NTransistorDrawer::new()),
			Self::PTransistor => Box::new(PTransistorDrawer::new()),

			Self::AndGate => Box::new(AndGateDrawer::new()),
			Self::NandGate => Box::new(NandGateDrawer::new()),
			Self::NorGate => Box::new(NorGateDrawer::new()),
			Self::NotGate => Box::new(NotGateDrawer::new()),
			Self::OrGate => Box::new(OrGateDrawer::new()),
			Self::TriStateBuffer => Box::new(TriStateBufferDrawer::new()),
			Self::XorGate => Box::new(XorGateDrawer::new()),
			
			Self::MultiBulb => Box::new(MultiBulbDrawer::new()),
			Self::MultiJunction => Box::new(MultiJunctionDrawer::new()),
			Self::MultiSwitch => Box::new(MultiSwitchDrawer::new()),
			
			Self::MultiTriStateBuffer => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit Tri-State Buffer", options.size),
				size: 17,
			})),

			Self::HalfAdder => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("Half Adder"),
				size: 27,
			})),
			Self::FullAdder => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("Full Adder"),
				size: 50,
			})),
			Self::Adder => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit Adder", options.size),
				size: 75,
			})),

			Self::SRLatch => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("SR Latch"),
				size: 70,
			})),
			Self::DLatch => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("D Latch"),
				size: 70,
			})),
			Self::DFlipFlop => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("D Flip-Flop"),
				size: 70,
			})),
			Self::MultiDFlipFlop => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit D Flip-Flop", options.size),
				size: 40,
			})),
			
			Self::Multiplexer => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("Multiplexer"),
				size: 27,
			})),
			Self::MultiMultiplexer => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit Multiplexer", options.size),
				size: 60,
			})),
			Self::FourWayMultiMultiplexer => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit Four Way Multiplexer", options.size),
				size: 60,
			})),

			Self::Register => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("16-bit Register"),
				size: 60,
			})),

			Self::Rom => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit ROM", options.size),
				size: 60,
			})),
			Self::Memory => Box::new(RectangleChipDrawer::new(TextInfo {
				text: format!("{}-bit Memory", options.size),
				size: 60,
			})),
			Self::Ram => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("RAM"),
				size: 240,
			})),
			
			Self::Counter => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("Counter"),
				size: 120,
			})),
			Self::ControlUnit => Box::new(RectangleChipDrawer::new(TextInfo {
				text: String::from("Control Unit"),
				size: 120,
			})),
		};

		Component {
			internals,
			options,
			position,
			size,
			ctype: *self,
			sim_mode: if simulator.is_none() { SimulationMode::Circuit } else { SimulationMode::HighLevel },
			simulator,
			drawer,
		}
	}
}

/// Returns whether or not the component should be displayed in the editor toolbar.
#[wasm_bindgen]
pub fn is_ct_spawnable(ct: ComponentType) -> bool {
	!matches!(
		ct,
		ComponentType::Pin |
		ComponentType::HalfAdder | ComponentType::FullAdder |
		ComponentType::SRLatch | ComponentType::DLatch | ComponentType::DFlipFlop
	)
}

/// Returns the name of the given [`ComponentType`].
#[wasm_bindgen]
pub fn get_ct_name(ct: ComponentType) -> String {
	match ct {
		ComponentType::Bulb => String::from("Bulb"),
		ComponentType::Junction => String::from("Junction"),
		ComponentType::Pin => String::from("Pin"),
		ComponentType::Switch => String::from("Switch"),

		ComponentType::NTransistor => String::from("N-type Transistor"),
		ComponentType::PTransistor => String::from("P-type Transistor"),

		ComponentType::AndGate => String::from("AND Gate"),
		ComponentType::NandGate => String::from("NAND Gate"),
		ComponentType::NorGate => String::from("NOR Gate"),
		ComponentType::NotGate => String::from("NOT Gate"),
		ComponentType::OrGate => String::from("OR Gate"),
		ComponentType::TriStateBuffer => String::from("Tri-State Buffer"),
		ComponentType::XorGate => String::from("XOR Gate"),

		ComponentType::MultiBulb => String::from("Multi Bulb"),
		ComponentType::MultiJunction => String::from("Multi Junction"),
		ComponentType::MultiSwitch => String::from("Multi Switch"),
			
		ComponentType::MultiTriStateBuffer => String::from("Multi Tri-State Buffer"),

		ComponentType::HalfAdder => String::from("Half Adder"),
		ComponentType::FullAdder => String::from("Full Adder"),
		ComponentType::Adder => String::from("Multi Adder"),

		ComponentType::SRLatch => String::from("SR Latch"),
		ComponentType::DLatch => String::from("D Latch"),
		ComponentType::DFlipFlop => String::from("D Flip-Flop"),
		ComponentType::MultiDFlipFlop => String::from("Multi D Flip-Flop"),

		ComponentType::Multiplexer => String::from("Multiplexer"),
		ComponentType::MultiMultiplexer => String::from("Multi Multiplexer"),
		ComponentType::FourWayMultiMultiplexer => String::from("Four-way Multi Multiplexer"),

		ComponentType::Register => String::from("16-bit Register"),

		ComponentType::Rom => String::from("16-bit ROM"),
		ComponentType::Memory => String::from("16-bit Memory"),
		ComponentType::Ram => String::from("RAM"),
		
		ComponentType::Counter => String::from("Counter"),
		ComponentType::ControlUnit => String::from("Control Unit"),
	}
}

/// Returns the slug of the given [`ComponentType`].
#[wasm_bindgen]
pub fn get_ct_slug(ct: ComponentType) -> String {
	match ct {
		ComponentType::Bulb => String::from("bulb"),
		ComponentType::Junction => String::from("junction"),
		ComponentType::Pin => String::from("pin"),
		ComponentType::Switch => String::from("switch"),

		ComponentType::NTransistor => String::from("ntransistor"),
		ComponentType::PTransistor => String::from("ptransistor"),

		ComponentType::AndGate => String::from("andgate"),
		ComponentType::NandGate => String::from("nandgate"),
		ComponentType::NorGate => String::from("norgate"),
		ComponentType::NotGate => String::from("notgate"),
		ComponentType::OrGate => String::from("orgate"),
		ComponentType::TriStateBuffer => String::from("tristatebuffer"),
		ComponentType::XorGate => String::from("xorgate"),

		ComponentType::MultiBulb => String::from("multibulb"),
		ComponentType::MultiJunction => String::from("multijunction"),
		ComponentType::MultiSwitch => String::from("multiswitch"),
			
		ComponentType::MultiTriStateBuffer => String::from("multitristatebuffer"),

		ComponentType::HalfAdder => String::from("halfadder"),
		ComponentType::FullAdder => String::from("fulladder"),
		ComponentType::Adder => String::from("adder"),

		ComponentType::SRLatch => String::from("srlatch"),
		ComponentType::DLatch => String::from("dlatch"),
		ComponentType::DFlipFlop => String::from("dflipflop"),
		ComponentType::MultiDFlipFlop => String::from("multidflipflop"),

		ComponentType::Multiplexer => String::from("multiplexer"),
		ComponentType::MultiMultiplexer => String::from("multiplexer"),
		ComponentType::FourWayMultiMultiplexer => String::from("fourway"),

		ComponentType::Register => String::from("register"),

		ComponentType::Rom => String::from("rom"),
		ComponentType::Memory => String::from("memory"),
		ComponentType::Ram => String::from("ram"),
		
		ComponentType::Counter => String::from("counter"),
		ComponentType::ControlUnit => String::from("controlunit"),
	}
}

/// Something that simulates the behaviour of a component.
pub trait ComponentSimulator {
	/// Gives the simulator some memory (used for ROM and RAM).
	fn give_memory(&mut self, _memory: &[u16]) {

	}
	/// Takes memory away from the simualotor (used for ROM and RAM).
	fn take_memory(&self) -> &[u16] {
		&[]
	}

	/// Switches the simulation mode to [`SimulationMode::HighLevel`].
	fn set_mode_to_high_level(&mut self, _circuit: &Circuit) {

	}

	/// Switches the simulation mode to [`SimulationMode::Circuit`].
	fn set_mode_to_circuit(&mut self, _circuit: &mut Circuit) {

	}

	/// Returns the state of a pin.
	fn get_pin_state_high_level(&self, _idx: usize) -> Result<PinState, PinError> {
		panic!("Unexpected get_pin_state_high_level");
	}

	/// Sets the state of a pin.
	fn set_pin_state_high_level(&mut self, _idx: usize, _state: PinState) -> Result<(), PinError> {
		panic!("Unexpected set_pin_state_high_level");
	}

	/// Returns the state of a pin when accessed externally. This is used for accessing a pin from
	/// the chip it's in.
	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		self.get_pin_state_high_level(idx)
	}

	/// Sets the state of a pin externally. This is used to manually modify pins (e.g. when turning on a switch).
	fn set_pin_state_external(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		self.set_pin_state_high_level(idx, state)
	}
}

/// A [`ComponentSimulator`] for an internal pin.
struct PinSimulator {
	/// The state accessible by the [`Circuit`].
	inner_state: PinState,
	/// The state accessible by the [`Chip`].
	outer_state: PinState,
}

impl PinSimulator {
	/// Returns a new [`PinSimulator`].
	fn new() -> Self {
		Self {
			inner_state: PinState::Disconnected,
			outer_state: PinState::Disconnected,
		}
	}
}

impl ComponentSimulator for PinSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.inner_state)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
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

/// A simulator for a switch.
struct SwitchSimulator {
	/// The state of the switch
	state: PinState,
}

impl SwitchSimulator {
	/// Returns a new [`SwitchSimulator`].
	fn new() -> Self {
		Self {
			state: PinState::Off,
		}
	}
}

impl ComponentSimulator for SwitchSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.state)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, _state: PinState) -> Result<(), PinError> {
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

/// A [`ComponentDrawer`] that draws a switch.
struct SwitchDrawer;

impl SwitchDrawer {
	/// Returns a new [`SwitchDrawer`].
	pub fn new() -> Self {
		Self
	}
}

impl Default for SwitchDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentDrawer for SwitchDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, component: &Component) {
		ctx.set_fill_style(&component.get_pin_state(0).unwrap().get_colour().into());

		let (width, height) = component.size;

		ctx.fill_rect(
			-width * 0.5,
			-height * 0.5,
			width,
			height,
		);
	}
}

/// A [`ComponentSimulator`] for a multi-switch.
struct MultiSwitchSimulator {
	/// The states of each switch.
	states: Vec<PinState>,
}

impl MultiSwitchSimulator {
	/// Returns a new [`MultiSwitchSimulator`].
	fn new(size: usize) -> Self {
		Self {
			states: vec![PinState::Off; size],
		}
	}
}

impl ComponentSimulator for MultiSwitchSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.states[idx])
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, _state: PinState) -> Result<(), PinError> {
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

/// A [`ComponentDrawer`] that draws a multi-switch.
struct MultiSwitchDrawer;

impl MultiSwitchDrawer {
	/// Returns a new [`MultiSwitchDrawer`].
	fn new() -> Self {
		Self
	} 
}

impl ComponentDrawer for MultiSwitchDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, component: &Component) {
		ctx.set_line_width(10.0);

		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		let (width, height) = component.size;

		ctx.stroke_rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill_rect(-width * 0.5, -height * 0.5, width, height);
		
		ctx.set_fill_style(&"#fff".into());
		ctx.set_font("bold 70px monospace");
		ctx.set_text_align("center");
		ctx.set_text_baseline("middle");

		let size = component.options.size;

		let states: Vec<_> = (0..size)
			.map(|i| component.get_pin_state(i).unwrap())
			.collect();
		
		let num = states_to_num(&states);

		ctx.fill_text(format!("{}", num).as_str(), 0.0, -height * 0.1).unwrap();

		for (i, state) in states.iter().enumerate().take(size) {
			ctx.set_fill_style(&state.get_colour().into());

			let extra_width = if i == size-1 { 0.0 } else { 1.0 };
			ctx.fill_rect((i as f64 - size as f64 * 0.5) * 50.0, height * 0.5 - 50.0, 50.0 + extra_width, 50.0);
		}
	}
}

/// A [`ComponentSimulator`] for a bulb.
struct BulbSimulator {
	/// The bulb's state.
	state: PinState,
}

impl BulbSimulator {
	/// Returns a new [`BulbSimulator`].
	fn new() -> Self {
		Self {
			state: PinState::Disconnected,
		}
	}
}

impl ComponentSimulator for BulbSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			self.state = state;
			Ok(())
		}
	}

	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		if idx > 0 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.state)
		}
	}
}

/// A [`ComponentDrawer`] that draws a bulb.
struct BulbDrawer;

impl BulbDrawer {
	/// Returns a new [`BulbDrawer`].
	fn new() -> Self {
		Self
	}
}

impl ComponentDrawer for BulbDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, component: &Component) {
		let state = component.simulator.as_ref().unwrap().get_pin_state_external(0).unwrap();

		ctx.set_fill_style(&state.get_colour().into());
		
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


/// A [`ComponentSimulator`] for a multi-bulb.
struct MultiBulbSimulator {
	/// The bulb states.
	states: Vec<PinState>,
}

impl MultiBulbSimulator {
	/// Returns a new [`MultiBulbSimulator`].
	fn new(size: usize) -> Self {
		Self {
			states: vec![PinState::Disconnected; size],
		}
	}
}

impl ComponentSimulator for MultiBulbSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			self.states[idx] = state;
			Ok(())
		}
	}

	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.states[idx])
		}
	}
}

/// A [`ComponentDrawer`] that draws a bulb.
struct MultiBulbDrawer;

impl MultiBulbDrawer {
	/// Returns a new [`MultiBulbDrawer`].
	fn new() -> Self {
		Self
	}
}

impl ComponentDrawer for MultiBulbDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, component: &Component) {
		ctx.set_line_width(10.0);

		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		let size = component.options.size;

		let states: Vec<_> = (0..size)
			.map(|i| component.simulator.as_ref().unwrap().get_pin_state_external(i).unwrap())
			.collect();

		let (width, height) = component.size;

		ctx.stroke_rect(-width * 0.5, -height * 0.5, width, height);
		ctx.fill_rect(-width * 0.5, -height * 0.5, width, height);
		
		ctx.set_fill_style(&"#fff".into());
		ctx.set_font("bold 70px monospace");
		ctx.set_text_align("center");
		ctx.set_text_baseline("middle");
		
		let num = states_to_num(&states);

		ctx.fill_text(format!("{}", num).as_str(), 0.0, -height * 0.1).unwrap();

		for (i, state) in states.iter().enumerate().take(size) {
			ctx.set_fill_style(&state.get_colour().into());

			ctx.begin_path();
			ctx.arc((i as f64 - size as f64 * 0.5) * 50.0 + 25.0, height * 0.5 - 25.0, 20.0, 0.0, 2.0 * PI).unwrap();
			ctx.fill();
		}
	}
}

/// A [`ComponentSimulator`] for a junction.
struct JunctionSimulator {
	/// The junction's states.
	states: Vec<PinState>,
}

impl JunctionSimulator {
	/// Returns a new [`JunctionSimulator`].
	fn new(size: usize) -> Self {
		Self {
			states: vec![PinState::Disconnected; size],
		}
	}

	/// Returns the output state of the junction.
	fn get_state(&self) -> PinState {
		*self.states.iter().reduce(|accum, state| accum.combine(state)).unwrap()
	}
}

impl ComponentSimulator for JunctionSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else if self.states[idx] == PinState::Disconnected {
			Ok(self.get_state())
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			self.states[idx] = state;
			Ok(())
		}
	}

	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.get_state())
		}
	}
}

/// A [`ComponentDrawer`] that draws a junction.
struct JunctionDrawer;

impl JunctionDrawer {
	/// Returns a new [`JunctionDrawer`].
	fn new() -> Self {
		Self
	}
}

impl ComponentDrawer for JunctionDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, component: &Component) {
		let state = component.simulator.as_ref().unwrap().get_pin_state_external(0).unwrap();

		ctx.set_fill_style(&state.get_colour().into());
		
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

/// A [`ComponentSimulator`] for a multi-junction.
struct MultiJunctionSimulator {
	/// The set of states for each junction.
	states: Vec<Vec<PinState>>,
}

impl MultiJunctionSimulator {
	/// Returns a new [`MultiJunctionSimulator`].
	fn new(size: usize) -> Self {
		Self {
			states: vec![vec![PinState::Disconnected; 3]; size],
		}
	}

	/// Returns the output state of the junction.
	fn get_state(&self, idx: usize) -> PinState {
		*self.states[idx].iter().reduce(|accum, state| accum.combine(state)).unwrap()
	}
}

impl ComponentSimulator for MultiJunctionSimulator {
	fn get_pin_state_high_level(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() * 3 {
			Err(PinError::OutOfRange)
		} else if self.states[idx / 3][idx % 3] == PinState::Disconnected {
			Ok(self.get_state(idx / 3))
		} else {
			Ok(PinState::Disconnected)
		}
	}

	fn set_pin_state_high_level(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		if idx >= self.states.len() * 3 {
			Err(PinError::OutOfRange)
		} else {
			self.states[idx / 3][idx % 3] = state;
			Ok(())
		}
	}

	fn get_pin_state_external(&self, idx: usize) -> Result<PinState, PinError> {
		if idx >= self.states.len() * 3 {
			Err(PinError::OutOfRange)
		} else {
			Ok(self.get_state(idx / 3))
		}
	}
}

/// A [`ComponentDrawer`] that draws a multi-junction.
struct MultiJunctionDrawer;

impl MultiJunctionDrawer {
	/// Returns a new [`MultiJunctionDrawer`].
	fn new() -> Self {
		Self
	}
}

impl ComponentDrawer for MultiJunctionDrawer {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, _viewport: BoundingBox, component: &Component) {
		let size = component.options.size;

		let states = (0..size)
			.map(|i| component.simulator.as_ref().unwrap().get_pin_state_external(i * 3).unwrap());

		for (idx, state) in states.enumerate() {
			ctx.set_fill_style(&state.get_colour().into());
			
			let spacing = 30.0;
			let radius = 10.0;

			let x = (idx as f64 - size as f64 * 0.5 + 0.5) * spacing;
			let y = if component.options.should_flip_multi_junction {
				-x
			} else {
				x
			};

			ctx.begin_path();
			ctx.arc(
				x,
				y,
				radius,
				0.0,
				2.0 * PI,
			).unwrap();
			ctx.fill();
		}
	}
}

/// The data needed to represent the inside of a [`Component`].
pub enum ComponentInternals {
	/// A list of pin positions defined manually.
	Atomic(Vec<(f64, f64)>),
	/// A chip defined by a circuit and a scale.
	Chip(Lazy<Circuit>, f64),
}

impl ComponentInternals {
	/// Returns the mutable circuit in the component, if any.
	pub fn get_circuit(&self) -> Option<&Circuit> {
		match self {
			ComponentInternals::Atomic(_) => None,
			ComponentInternals::Chip(circuit, _) => Some(circuit.get()),
		}
	}

	/// Returns the mutable circuit in the component, if any.
	pub fn get_circuit_mut(&mut self) -> Option<&mut Circuit> {
		match self {
			ComponentInternals::Atomic(_) => None,
			ComponentInternals::Chip(circuit, _) => Some(circuit.get_mut()),
		}
	}

	/// Returns the inner scale of the circuit in the component, if any.
	pub fn get_inner_scale(&self) -> Option<f64> {
		match self {
			ComponentInternals::Atomic(_) => None,
			ComponentInternals::Chip(_, scale) => Some(*scale),
		}
	}
}

/// Something that can go in a circuit. A [`Component`] may be connected to another [`Component`] using a [`Wire`].
pub struct Component {
	/// The [`ComponentInternals`] used to implement the component.
	pub internals: ComponentInternals,
	/// The position of the component.
	pub position: (f64, f64),
	/// The size of the component.
	pub size: (f64, f64),
	/// Options for the component (e.g. size).
	pub options: ComponentOptions,
	
	/// The type of component.
	ctype: ComponentType,
	/// The simulation mode of the component.
	sim_mode: SimulationMode,

	/// A [`ComponentSimulator`] instance. Used to simulate the component's functionality.
	/// If [`None`], the internal circuit is used exclusively.
	pub simulator: Option<Box<dyn ComponentSimulator>>,
	/// A [`ComponentDrawer`] instance. Used to draw the component.
	drawer: Box<dyn ComponentDrawer>,
}

impl Component {
	/// Returns the type of the component.
	pub fn get_type(&self) -> ComponentType {
		self.ctype
	}

	/// Returns the name of the component as a string.
	pub fn get_name(&self) -> String {
		String::from(self.ctype.as_string())
	}

	/// Returns how many switches are in the component.
	pub fn get_switch_count(&self) -> usize {
		if self.ctype.has_switches() {
			self.options.size
		} else {
			0
		}
	}

	/// Returns the list of pin positions.
	pub fn get_pin_positions(&self) -> Vec<(f64, f64)> {
		match &self.internals {
			ComponentInternals::Atomic(pin_positions) => pin_positions.clone(),
			ComponentInternals::Chip(circuit, inner_scale) => {
				circuit.get().get_pin_positions().iter()
					.map(|(x, y)| (x * inner_scale, y * inner_scale))
					.collect()
			},
		}
	}

	/// Returns how many pins the component has.
	fn get_pin_count(&self) -> usize {
		// Some components have hard-coded calculations to prevent
		// excessive recursion (e.g. memory chips).

		match self.get_type() {
			ComponentType::Memory => 16 + 16 + 1 + self.options.size,
			_ => self.get_pin_positions().len(),
		}
	}

	/// Returns whether the component is an internal pin.
	fn is_pin(&self) -> bool {
		self.ctype == ComponentType::Pin
	}

	/// Returns the current simulation mode.
	fn get_mode(&self) -> SimulationMode {
		self.sim_mode
	}

	/// Sets the current simulation mode.
	pub fn set_mode(&mut self, mode: SimulationMode) {
		if let Some(simulator) = self.simulator.as_mut() {
			if let ComponentInternals::Chip(circuit, _) = &mut self.internals {
				if self.sim_mode == SimulationMode::HighLevel && mode == SimulationMode::Circuit {
					simulator.set_mode_to_circuit(circuit.get_mut());
					self.sim_mode = mode;
				}
				else if self.sim_mode == SimulationMode::Circuit && mode == SimulationMode::HighLevel {
					simulator.set_mode_to_high_level(circuit.get());
					self.sim_mode = mode;
				}
			}
		} else {
			self.sim_mode = SimulationMode::Circuit;
		}
	}

	/// Returns the state of a pin.
	pub fn get_pin_state(&self, idx: usize) -> Result<PinState, PinError> {
		if let ComponentInternals::Chip(circuit, _) = &self.internals {
			match self.get_mode() {
				SimulationMode::Circuit => {
					let maybe_pin_component = circuit.get().components.iter()
						.filter(|c| c.is_pin())
						.nth(idx);
			
					match maybe_pin_component {
						Some(pin_component) => {
							pin_component.simulator.as_ref().unwrap().get_pin_state_external(0)
						},
						None => Err(PinError::OutOfRange),
					}
				},
				SimulationMode::HighLevel => self.simulator.as_ref().unwrap().get_pin_state_high_level(idx),
			}
		} else {
			self.simulator.as_ref().unwrap().get_pin_state_high_level(idx)
		}
	}

	/// Sets the state of a pin.
	fn set_pin_state(&mut self, idx: usize, state: PinState) -> Result<(), PinError> {
		let mode = self.get_mode();

		if let ComponentInternals::Chip(circuit, _) = &mut self.internals {
			match mode {
				SimulationMode::Circuit => {
					let maybe_component_idx = circuit.get_mut().components.iter_mut()
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
							circuit.get_mut().update_component(&connection, state, true);
							Ok(())
						},
						None => Err(PinError::OutOfRange),
					}
				},
				SimulationMode::HighLevel => self.simulator.as_mut().unwrap().set_pin_state_high_level(idx, state),
			}
		} else {
			self.simulator.as_mut().unwrap().set_pin_state_high_level(idx, state)
		}
	}

	/// Returns whether the given viewport is fully contained within the component.
	pub fn contains(&self, viewport: &BoundingBox) -> bool {
		if self.ctype.should_expand_circuit() {
			let position = self.position;
			let size = self.size;
	
			let contains_x =
				position.0 + size.0 * 0.5 >= viewport.get_position().0 + viewport.get_size().0 * 0.5 &&
				position.0 - size.0 * 0.5 <= viewport.get_position().0 - viewport.get_size().0 * 0.5;
	
			let contains_y =
				position.1 + size.1 * 0.5 >= viewport.get_position().1 + viewport.get_size().1 * 0.5 &&
				position.1 - size.1 * 0.5 <= viewport.get_position().1 - viewport.get_size().1 * 0.5;
	
			contains_x && contains_y
		} else {
			false
		}
	}

	/// Returns whether the given viewport is partially contained within the component.
	pub fn intersects(&self, viewport: &BoundingBox) -> bool {
		let position = self.position;
		let size = self.size;

		let intersects_x =
			position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
	}

	/// Returns whether the given viewport can see the internals of the component.
	pub fn are_internals_visible(&self, viewport: &BoundingBox) -> bool {
		let start_ratio = 0.3;

		let height = self.size.1;
		let height_ratio = height / viewport.get_size().1;

		self.intersects(viewport) && height_ratio > start_ratio
	}
}

impl Drawable for Component {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox) {
		self.drawer.draw(ctx, viewport, self)
	}
}

/// A specifier for a pin on a particular component. This differs from the pin component, which is an internal
/// pin used in a [`Circuit`] within a chip.
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
#[derive(Debug)]
pub struct Wire {
	/// The first pin that the wire is connected to.
	pub pin1: ExternalPin,
	/// The second pin that the wire is connected to.
	pub pin2: ExternalPin,
	/// Commands used to specify how the wire is rendered.
	pub layout_commands: Lazy<Vec<WireLayoutCommand>>,
	/// The state being emitted by pin 1.
	state1: PinState,
	/// The state being emitted by pin 2.
	state2: PinState,
}

/// A group of pins.
type Group = Vec<(usize, usize)>;

/// A circuit that consists of components connected by wires.
pub struct Circuit {
	/// The list of components.
	pub components: Vec<Component>,
	/// The list of wires connecting the components.
	pub wires: Vec<Wire>,
	/// The list of [`BusLayoutCommand`]s that need to be computed.
	pub bus_commands: Vec<(Group, Group, Vec<BusLayoutCommand>)>,
}

impl Circuit {
	/// Returns the `i`th pin component in the circuit.
	pub fn get_pin(&self, i: usize) -> Option<&Component> {
		self.components.iter()
			.filter(|c| c.is_pin())
			.nth(i)
	}

	/// Sets a pin component to a given [`PinState`].
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
	pub fn add(&mut self, component: Component) -> usize {
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

	/// Connects two components together with a wire where the layout of the wire
	/// is defined lazily.
	fn connect_lazy(
		&mut self, (comp1_idx, pin1_idx): (usize, usize),
		(comp2_idx, pin2_idx): (usize, usize),
		wire_commands: Lazy<Vec<WireLayoutCommand>>,
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

	/// Connects two components together with a wire.
	pub fn connect(&mut self, pin1: (usize, usize), pin2: (usize, usize), wire_commands: &[WireLayoutCommand]) {
		self.connect_lazy(pin1, pin2, Lazy::from(wire_commands.to_vec()));
	}

	/// Computes a list of [`WireLayoutCommand`]s given a list of [`BusLayoutCommand`]s.
	fn compute_wire_commands(
		&self,
		group1: &[(usize, usize)], group2: &[(usize, usize)],
		commands: &[BusLayoutCommand]
	) -> Vec<Vec<WireLayoutCommand>> {
		if group1.is_empty() || group2.is_empty() {
			return vec![];
		}

		let group1_positions: Vec<_> = group1.iter()
			.map(|(cidx, pidx)| {
				(
					self.components[*cidx].position.0 + self.components[*cidx].get_pin_positions()[*pidx].0,
					self.components[*cidx].position.1 + self.components[*cidx].get_pin_positions()[*pidx].1,
				)
			})
			.collect();

		let group2_positions: Vec<_> = group2.iter()
			.map(|(cidx, pidx)| {
				(
					self.components[*cidx].position.0 + self.components[*cidx].get_pin_positions()[*pidx].0,
					self.components[*cidx].position.1 + self.components[*cidx].get_pin_positions()[*pidx].1,
				)
			})
			.collect();

		compute_wire_commands(commands, &group1_positions, &group2_positions)
	}

	/// Connects one group of pins to another.
	pub fn connect_groups(&mut self, group1: &[(usize, usize)], group2: &[(usize, usize)], commands: &[BusLayoutCommand]) {
		// Save the commands to be evaluated later
		self.bus_commands.push((group1.to_vec(), group2.to_vec(), commands.to_vec()));

		for (pin1, pin2) in group1.iter().zip(group2) {
			self.connect_lazy(*pin1, *pin2, Lazy::empty());
		}
	}

	/// Converts switch or bulb components into individual pins.
	pub fn pinify(&mut self, indices: &mut [usize]) {
		indices.sort();
		indices.reverse();

		// Stores the list of updates needed to propagate pin changes
		let mut all_updates = vec![];

		// Disconnect all the switches and bulbs first
		for idx in indices.iter() {
			let component = &self.components[*idx];

			if !matches!(component.ctype, ComponentType::Switch | ComponentType::MultiSwitch) {
				continue;
			}

			let pin_count = component.get_pin_count();

			for pin_idx in 0..pin_count {
				let con = ExternalPin {
					component_idx: *idx,
					pin_idx,
				};

				self.update_component(&con, PinState::Disconnected, true);
			}
		}

		for idx in indices.iter() {
			let component = &self.components[*idx];
			let position = component.position;
	
			match component.ctype {
				ComponentType::Bulb | ComponentType::Switch => {
					let pin_idx = add!(self, Pin, component.position);
	
					for wire in &mut self.wires {
						if wire.pin1.component_idx == *idx {
							wire.pin1.component_idx = pin_idx;
							all_updates.push((wire.pin1, wire.state2));
						}
						if wire.pin2.component_idx == *idx {
							wire.pin2.component_idx = pin_idx;
							all_updates.push((wire.pin2, wire.state1));
						}
					}
				},
	
				ComponentType::MultiBulb | ComponentType::MultiSwitch => {
					let size = component.options.size;
					let spacing = 50.0;

					let pin_indices: Vec<_> = (0..size)
						.map(|i| {
							let pos = (
								position.0 + (i as f64 - size as f64 * 0.5 + 0.5) * spacing,
								position.1,
							);
							add!(self, Pin, pos)
						})
						.collect();
	
					for wire in &mut self.wires {
						if wire.pin1.component_idx == *idx {
							wire.pin1.component_idx = pin_indices[wire.pin1.pin_idx];
							wire.pin1.pin_idx = 0;
							all_updates.push((wire.pin1, wire.state2));
						}
						if wire.pin2.component_idx == *idx {
							wire.pin2.component_idx = pin_indices[wire.pin2.pin_idx];
							wire.pin2.pin_idx = 0;
							all_updates.push((wire.pin2, wire.state1));
						}
					}
				},
	
				_ => {},
			}
		}

		for (con, state) in all_updates {
			// false is used because we're updating a pin, so we want the change to propagate.
			self.update_component(&con, state, false);
		}

		for idx in indices {
			self.remove(*idx);
		}
	}

	/// Sets a component's memory (used for ROM and RAM).
	pub fn set_memory(&mut self, cidx: usize, memory: &[u16]) {
		let component = &mut self.components[cidx];

		if let Some(ref mut sim) = component.simulator {
			sim.give_memory(memory);

			let mut pin_states = Vec::with_capacity(component.get_pin_count());

			for i in 0..component.get_pin_count() {
				let state = component.get_pin_state(i).unwrap();
				pin_states.push(state);
			}

			for (i, state) in pin_states.iter().enumerate() {
				if *state != PinState::Disconnected {
					self.update_component(&ExternalPin { component_idx: cidx, pin_idx: i }, *state, true);
				}
			}
		}
	}
	
	/// Updates a pin and then propagates the changes. This function is the main
	/// part of the circuit simulator.
	pub fn update_component(&mut self, pin: &ExternalPin, state: PinState, set_manually: bool) {
		let component = &mut self.components[pin.component_idx];

		let old_pin_states: Vec<_> = (0..component.get_pin_count())
			.map(|i| component.get_pin_state(i).unwrap())
			.collect();

		if set_manually {
			component.simulator.as_mut().unwrap().set_pin_state_external(pin.pin_idx, state).unwrap();
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
	pub fn get_component_from_chip_stack(&mut self, stack: &[usize]) -> Option<&mut Component> {
		match stack.len() {
			0 => None,
			1 => Some(&mut self.components[stack[0]]),
			_ => match &mut self.components[stack[0]].internals {
				ComponentInternals::Atomic(_) => None,
				ComponentInternals::Chip(circuit, _) =>
					circuit.get_mut().get_component_from_chip_stack(&stack[1..]),
			},
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
			.map(|c| c.position)
			.collect()
	}

	/// Draws the selection boxes for each component.
	pub fn draw_selection_boxes(&self, ctx: &web_sys::CanvasRenderingContext2d, selected_chip_stacks: &[Vec<usize>]) {
		let first_indices: Vec<_> = selected_chip_stacks.iter().map(|cs| cs[0]).collect();

		for (cidx, component) in self.components.iter().enumerate() {
			if first_indices.contains(&cidx) {
				let (x, y) = component.position;
				let (mut width, mut height) = component.size;

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
			
			let (x, y) = component.position;
			ctx.translate(x, y).unwrap();

			for (pidx, pin_pos) in component.get_pin_positions().iter().enumerate().rev() {
				let con = ExternalPin { component_idx: cidx, pin_idx: pidx };

				if self.wires.iter().any(|w| w.pin1 == con || w.pin2 == con) {
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
			bus_commands: vec![],
		}
	}

	/// Toggles a switch in the circuit.
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
		Some(component.position)
	}

	/// Sets the x coordinate of a component given the chip stack.
	pub fn set_x_from_chip_stack(&mut self, stack: &[usize], x: f64) {
		if let Some(component) = self.get_component_from_chip_stack(stack) {
			component.position.0 = x;
		}
	}

	/// Sets the y coordinate of a component given the chip stack.
	pub fn set_y_from_chip_stack(&mut self, stack: &[usize], y: f64) {
		if let Some(component) = self.get_component_from_chip_stack(stack) {
			component.position.1 = y;
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
			component.position = (x, y);
		}
	}

	/// Connects two components with a wire externally.
	/// Used to connect wires from JavaScript.
	pub fn connect_external(
		&mut self, comp1_idx: usize, pin1_idx: usize,
		comp2_idx: usize, pin2_idx: usize,
	) {
		self.connect((comp1_idx, pin1_idx), (comp2_idx, pin2_idx), &[]);
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

			let c1 = comp1.position;
			let c2 = comp2.position;

			let p1 = comp1.get_pin_positions()[con1.pin_idx];
			let p2 = comp2.get_pin_positions()[con2.pin_idx];

			let start = (c1.0 + p1.0, c1.1 + p1.1);
			let end = (c2.0 + p2.0, c2.1 + p2.1);

			ctx.begin_path();
			ctx.move_to(start.0, start.1);

			let mut current_pos = start;

			if wire.layout_commands.try_get().is_none() {
				for (group1, group2, commands) in &self.bus_commands {
					let maybe_idx = group1.iter().zip(group2)
						.enumerate()
						.find(|(_, (p1, p2))| {
							**p1 == (con1.component_idx, con1.pin_idx) &&
							**p2 == (con2.component_idx, con2.pin_idx)
						});
	
					if let Some((idx, _)) = maybe_idx {
						let wire_commands = self.compute_wire_commands(group1, group2, commands);
						wire.layout_commands.set(wire_commands[idx].clone());
					}
				}
			}

			if let Some(commands) = wire.layout_commands.try_get() {
				for (idx, command) in commands.iter().enumerate() {
					let prev_pos = current_pos;
	
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
						WireLayoutCommand::DontRenderPreviousHorizontal => {},
						WireLayoutCommand::DontRenderPreviousVertical => {},
					}
	
					if matches!(
						*command,
						WireLayoutCommand::DontRenderPrevious |
						WireLayoutCommand::DontRenderPreviousHorizontal |
						WireLayoutCommand::DontRenderPreviousVertical
					) {
						continue;
					}
	
					if idx == commands.len() - 1 {
						ctx.line_to(current_pos.0, current_pos.1);
						continue;
					}
	
					match commands[idx + 1] {
						WireLayoutCommand::DontRenderPrevious => {},
						WireLayoutCommand::DontRenderPreviousHorizontal => {
							ctx.line_to(prev_pos.0, current_pos.1);
						},
						WireLayoutCommand::DontRenderPreviousVertical => {
							ctx.line_to(current_pos.0, prev_pos.1);
						},
						_ => {
							ctx.line_to(current_pos.0, current_pos.1);
						},
					}
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
			
			let (x, y) = component.position;
			ctx.translate(x, y).unwrap();

			component.draw(ctx, viewport);
			
			ctx.restore();
		}
	}
}

impl Drawable for ComponentInternals {
	fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, viewport: BoundingBox) {
		match self {
			ComponentInternals::Atomic(_) => {},
			ComponentInternals::Chip(circuit, inner_scale) => {
				ctx.save();
				ctx.scale(*inner_scale, *inner_scale).unwrap();
		
				circuit.get().draw(ctx, viewport);
		
				ctx.restore();
			},
		}
	}
}
