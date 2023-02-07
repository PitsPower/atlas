#![feature(test)]

extern crate test;

pub mod adder;
pub mod assembler;
pub mod bus;
pub mod control;
pub mod core;
pub mod editor;
pub mod gates;
pub mod graphics;
pub mod latches;
pub mod memory;
pub mod multiplexer;
pub mod register;
pub mod transistor;
pub mod utils;
pub mod vm;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use crate::assembler::assemble;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;
use crate::vm::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		web_sys::console::log_1(&format!($($arg)*).into())
	};
}

#[macro_export]
macro_rules! add {
	($circ:expr, $type:ident, $pos:expr) => {{
		let component = ComponentType::$type.create($pos, ComponentOptions {
			size: 1,
			should_flip_multi_junction: false,
		});
		$circ.add(component)
	}};
	($circ:expr, $type:ident, $pos:expr, $size:expr) => {{
		let component = ComponentType::$type.create($pos, ComponentOptions {
			size: $size,
			should_flip_multi_junction: false,
		});
		$circ.add(component)
	}};
	($circ:expr, $type:ident, $pos:expr, $size:expr, $bool:expr) => {{
		let component = ComponentType::$type.create($pos, ComponentOptions {
			size: $size,
			should_flip_multi_junction: $bool,
		});
		$circ.add(component)
	}};
}

// #[wasm_bindgen]
// pub fn example1(n: u32) -> Circuit {
// 	fn iter(n: u32) -> Circuit {
// 		let mut result = Circuit::new();
		
// 		if n > 0 {
// 			let chip1 = RectangleChip {
// 				internals: ChipInternals {
// 					circuit: iter(n-1),
// 					inner_scale: 0.4,
// 				},
// 				position: (-180.0, 0.0),
// 				size: (300.0, 300.0),
// 				text: None,
// 			};
// 			let chip2 = RectangleChip {
// 				internals: ChipInternals {
// 					circuit: iter(n-1),
// 					inner_scale: 0.4,
// 				},
// 				position: (180.0, 0.0),
// 				size: (300.0, 300.0),
// 				text: None,
// 			};

// 			result.add(Box::new(chip1));
// 			result.add(Box::new(chip2));
// 		}

// 		result
// 	}

// 	let chip = RectangleChip {
// 		internals: ChipInternals {
// 			circuit: iter(n),
// 			inner_scale: 1.0,
// 		},
// 		position: (0.0, 10.0),
// 		size: (850.0, 500.0),
// 		text: None,
// 	};

// 	let mut circuit = Circuit::new();
// 	circuit.add(Box::new(chip));
// 	circuit
// }

// #[wasm_bindgen]
// pub fn example2() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let switch = circuit.add(Box::new(Switch::new((-200.0, 0.0))));
// 	let junction = circuit.add(Box::new(Junction::new((0.0, 0.0), 3)));
// 	let bulb1 = circuit.add(Box::new(Bulb::new((200.0, 0.0))));
// 	let bulb2 = circuit.add(Box::new(Bulb::new((0.0, -200.0))));

// 	circuit.connect((switch, 0), (junction, 0), vec![]);
// 	circuit.connect((junction, 1), (bulb1, 0), vec![]);
// 	circuit.connect((junction, 2), (bulb2, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn transistor_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let transistor = circuit.add(Box::new(NTransistor::new((0.0, 0.0))));
	
// 	let gate = circuit.add(Box::new(Switch::new((-200.0, 0.0))));

// 	let offset = circuit.get_components()[transistor].get_pin_positions()[1].0;

// 	let source = circuit.add(Box::new(Switch::new((offset, 200.0))));
// 	let drain = circuit.add(Box::new(Bulb::new((offset, -200.0))));

// 	circuit.connect((gate, 0), (transistor, 0), vec![]);
// 	circuit.connect((source, 0), (transistor, 1), vec![]);
// 	circuit.connect((transistor, 2), (drain, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn transistor_example2() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let transistor = circuit.add(Box::new(PTransistor::new((0.0, 0.0))));
	
// 	let gate = circuit.add(Box::new(Switch::new((-200.0, 0.0))));

// 	let offset = circuit.get_components()[transistor].get_pin_positions()[1].0;

// 	let source = circuit.add(Box::new(Switch::new((offset, -200.0))));
// 	let drain = circuit.add(Box::new(Bulb::new((offset, 200.0))));

// 	circuit.connect((gate, 0), (transistor, 0), vec![]);
// 	circuit.connect((source, 0), (transistor, 1), vec![]);
// 	circuit.connect((transistor, 2), (drain, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn bidirectional_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let transistor1 = add!(circuit, NTransistor, (-400.0, 200.0));
// 	let transistor2 = add!(circuit, NTransistor, (400.0, 200.0));

// 	let switch1 = add!(circuit, Switch, (-600.0, 200.0));
// 	let switch2 = add!(circuit, Switch, (200.0, 200.0));

// 	let offset = circuit.get_components()[transistor1].get_pin_positions()[1].0;

// 	let source1 = add!(circuit, Switch, (-400.0 + offset, 400.0));
// 	let source2 = add!(circuit, Switch, (400.0 + offset, 400.0));

// 	circuit.toggle_switch(2);
// 	circuit.toggle_switch(3);

// 	let junction1 = add!(circuit, Junction, (-400.0 + offset, 0.0), 3);
// 	let junction2 = add!(circuit, Junction, (400.0 + offset, 0.0), 3);

// 	let bulb1 = add!(circuit, Bulb, (-400.0 + offset, -200.0));
// 	let bulb2 = add!(circuit, Bulb, (400.0 + offset, -200.0));

// 	circuit.connect((switch1, 0), (transistor1, 0), vec![]);
// 	circuit.connect((source1, 0), (transistor1, 1), vec![]);
// 	circuit.connect((transistor1, 2), (junction1, 0), vec![]);
// 	circuit.connect((junction1, 1), (bulb1, 0), vec![]);

// 	circuit.connect((switch2, 0), (transistor2, 0), vec![]);
// 	circuit.connect((source2, 0), (transistor2, 1), vec![]);
// 	circuit.connect((transistor2, 2), (junction2, 0), vec![]);
// 	circuit.connect((junction2, 1), (bulb2, 0), vec![]);
	
// 	circuit.connect((junction1, 2), (junction2, 2), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn not_gate_example() -> Circuit {
// 	let mut circuit = Circuit::new();
	
// 	let input = add!(circuit, Switch, (-300.0, 0.0));
// 	let not_gate_1 = add!(circuit, NotGate, (0.0, -150.0));
// 	let not_gate_2 = add!(circuit, NotGate, (0.0, 150.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input, 0), (not_gate_1, 0), vec![
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((not_gate_1, 1), (not_gate_2, 0), vec![
// 		WireLayoutCommand::MoveHorizontal(100.0),
// 		WireLayoutCommand::CenterVertical,
// 		WireLayoutCommand::MoveHorizontal(-300.0),
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((not_gate_2, 1), (output, 0), vec![
// 		WireLayoutCommand::AlignVertical,
// 	]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn nor_gate_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input1 = add!(circuit, Switch, (-300.0, -100.0));
// 	let input2 = add!(circuit, Switch, (-300.0, 100.0));
// 	let gate = add!(circuit, NorGate, (0.0, 0.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input1, 0), (gate, 0), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((input2, 0), (gate, 1), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((gate, 2), (output, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn or_gate_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input1 = add!(circuit, Switch, (-300.0, -100.0));
// 	let input2 = add!(circuit, Switch, (-300.0, 100.0));
// 	let gate = add!(circuit, OrGate, (0.0, 0.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input1, 0), (gate, 0), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((input2, 0), (gate, 1), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((gate, 2), (output, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn nand_gate_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input1 = add!(circuit, Switch, (-300.0, -100.0));
// 	let input2 = add!(circuit, Switch, (-300.0, 100.0));
// 	let gate = add!(circuit, NandGate, (0.0, 0.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input1, 0), (gate, 0), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((input2, 0), (gate, 1), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((gate, 2), (output, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn and_gate_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input1 = add!(circuit, Switch, (-300.0, -100.0));
// 	let input2 = add!(circuit, Switch, (-300.0, 100.0));
// 	let gate = add!(circuit, AndGate, (0.0, 0.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input1, 0), (gate, 0), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((input2, 0), (gate, 1), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((gate, 2), (output, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn xor_gate_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input1 = add!(circuit, Switch, (-300.0, -100.0));
// 	let input2 = add!(circuit, Switch, (-300.0, 100.0));
// 	let gate = add!(circuit, XorGate, (0.0, 0.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input1, 0), (gate, 0), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((input2, 0), (gate, 1), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((gate, 2), (output, 0), vec![]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn nor_latch_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input1 = add!(circuit, Switch, (-300.0, -100.0));
// 	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	
// 	let nor1 = add!(circuit, NorGate, (0.0, -100.0));
// 	let nor2 = add!(circuit, NorGate, (0.0, 100.0));

// 	let junction1 = add!(circuit, Junction, (150.0, -100.0), 3);
// 	let junction2 = add!(circuit, Junction, (150.0, 100.0), 3);

// 	let output1 = add!(circuit, Bulb, (300.0, -100.0));
// 	let output2 = add!(circuit, Bulb, (300.0, 100.0));

// 	circuit.connect((input1, 0), (nor1, 0), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((input2, 0), (nor2, 1), vec![
// 		WireLayoutCommand::CenterHorizontal,
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
	
// 	circuit.connect((nor1, 2), (junction1, 0), vec![]);
// 	circuit.connect((junction1, 1), (output1, 0), vec![]);
	
// 	circuit.connect((nor2, 2), (junction2, 0), vec![]);
// 	circuit.connect((junction2, 1), (output2, 0), vec![]);

// 	circuit.connect((junction1, 2), (nor2, 0), vec![
// 		WireLayoutCommand::MoveVertical(35.0),
// 		WireLayoutCommand::Move((-250.0, 100.0)),
// 		WireLayoutCommand::AlignHorizontal,
// 	]);
// 	circuit.connect((junction2, 2), (nor1, 1), vec![
// 		WireLayoutCommand::MoveVertical(-35.0),
// 		WireLayoutCommand::Move((-250.0, -100.0)),
// 		WireLayoutCommand::AlignHorizontal,
// 	]);

// 	circuit
// }

// #[wasm_bindgen]
// pub fn test_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let size = 8;
	
// 	let input1 = add!(circuit, MultiSwitch, (-600.0, -500.0), size);
// 	let input2 = add!(circuit, MultiSwitch, (-600.0, 0.0), size);

// 	let adder = add!(circuit, Adder, (0.0, 0.0), size);

// 	let output = add!(circuit, MultiBulb, (600.0, -200.0), size);

// 	for i in 0..size {
// 		circuit.connect((input1, i), (adder, size - i - 1), vec![
// 			WireLayoutCommand::AlignHorizontal,
// 		]);
// 		circuit.connect((input2, i), (adder, size + (size - i - 1)), vec![
// 			WireLayoutCommand::AlignHorizontal,
// 		]);
// 		circuit.connect((adder, 2 * (size as usize) + (size - i - 1) as usize), (output, i), vec![
// 			WireLayoutCommand::AlignVertical,
// 		]);
// 	}

// 	circuit
// }

// #[wasm_bindgen]
// pub fn bus_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input = add!(circuit, MultiSwitch, (-300.0, 0.0), 8);
// 	let output = add!(circuit, MultiBulb, (300.0, 0.0), 8);

// 	for i in 0..8 {
// 		circuit.connect((input, i), (output, i), vec![
// 			WireLayoutCommand::MoveVertical(i as f64 * 50.0 + 50.0),
// 			WireLayoutCommand::AlignVertical,
// 		]);
// 	}

// 	circuit
// }

// #[wasm_bindgen]
// pub fn latch_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let size = 8;

// 	let clock = add!(circuit, Switch, (-600.0, 550.0));
// 	let increment = add!(circuit, MultiSwitch, (0.0, -700.0), size);

// 	circuit.toggle_switch(size);

// 	let dff = add!(circuit, MultiDFlipFlop, (-600.0, 0.0), size);
// 	let adder = add!(circuit, Adder, (600.0, 0.0), size);
	
// 	let juncs: Vec<_> = (0..size)
// 		.map(|i| add!(circuit, Junction, (1100.0 + i as f64 * 30.0, (-(size as f64) * 0.5 + i as f64 + 0.5) * 30.0), 3))
// 		.collect();

// 	let output = add!(circuit, MultiBulb, (1800.0, -500.0), 8);

// 	circuit.connect((clock, 0), (dff, size), vec![]);

// 	let fsize = size as f64;

// 	for i in 0..size {
// 		let fi = i as f64;

// 		circuit.connect((increment, size - i - 1), (adder, i), vec![
// 			WireLayoutCommand::MoveVertical((if i < size/2 { fsize * 0.5 - 1.0 - fi } else { fi - fsize * 0.5 }) * 30.0 + 30.0),
// 			WireLayoutCommand::MoveHorizontal((if i < size/2 { -(fsize * 0.5 - 1.0 - fi + 0.5) } else { fi - fsize * 0.5 + 0.5 }) * 30.0),
// 			WireLayoutCommand::AlignHorizontal,
// 		]);

// 		circuit.connect((dff, size+1 + i), (adder, size + i), vec![
// 			WireLayoutCommand::MoveHorizontal(50.0),
// 			WireLayoutCommand::MoveHorizontal(200.0 - (fsize - fi - 1.0) * 15.0),
// 			WireLayoutCommand::AlignHorizontal,
// 		]);

// 		circuit.connect((adder, 2*size + i), (juncs[size - i - 1], 0), vec![
// 			WireLayoutCommand::MoveHorizontal((if i < size/2 { -(fsize * 0.5 - 1.0 - fi) } else { -(fi - fsize * 0.5) }) * 30.0 + 120.0),
// 			WireLayoutCommand::AlignHorizontal,
// 		]);

// 		circuit.connect((juncs[i], 2), (output, i), vec![
// 			WireLayoutCommand::AlignVertical,
// 		]);

// 		circuit.connect((juncs[i], 1), (dff, size - i - 1), vec![
// 			WireLayoutCommand::MoveVertical(900.0),
// 			WireLayoutCommand::MoveHorizontal(-2300.0),
// 			WireLayoutCommand::AlignHorizontal,
// 		]);
// 	}

// 	circuit
// }

// #[wasm_bindgen]
// pub fn register_example() -> Circuit {
// 	let mut circuit = Circuit::new();

// 	let input = add!(circuit, Switch, (-300.0, 0.0));
// 	let enable = add!(circuit, Switch, (0.0, -300.0));
// 	let tsb = add!(circuit, TriStateBuffer, (0.0, 0.0));
// 	let output = add!(circuit, Bulb, (300.0, 0.0));

// 	circuit.connect((input, 0), (tsb, 0), vec![]);
// 	circuit.connect((enable, 0), (tsb, 1), vec![]);
// 	circuit.connect((tsb, 2), (output, 0), vec![]);

// 	circuit
// }

pub fn register_example() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, MultiDFlipFlop, (0.000, 0.000), 8);
	let c1 = add!(circuit, MultiJunction, (0.000, 750.000), 8);
	let c2 = add!(circuit, Switch, (0.000, 550.000));
	let c3 = add!(circuit, MultiTriStateBuffer, (400.000, 0.000), 8);
	let c4 = add!(circuit, Switch, (400.000, -350.000));
	let c5 = add!(circuit, MultiJunction, (0.000, 1300.000), 8);
	let c6 = add!(circuit, MultiTriStateBuffer, (-350.000, 1650.000), 8);
	let c7 = add!(circuit, MultiBulb, (400.000, 1000.000), 8);
	let c8 = add!(circuit, Switch, (-350.000, 1250.000));
	let c9 = add!(circuit, MultiSwitch, (-700.000, 1450.000), 8);
	
	circuit.connect((c1, 0), (c0, 0), &[WireLayoutCommand::MoveTo((-450.000, 645.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((105.000, -0.000)), WireLayoutCommand::MoveTo((-345.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 3), (c0, 1), &[WireLayoutCommand::MoveTo((-450.000, 675.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((75.000, -0.000)), WireLayoutCommand::MoveTo((-375.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 6), (c0, 2), &[WireLayoutCommand::MoveTo((-450.000, 705.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((45.000, -0.000)), WireLayoutCommand::MoveTo((-405.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 9), (c0, 3), &[WireLayoutCommand::MoveTo((-450.000, 735.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, -0.000)), WireLayoutCommand::MoveTo((-435.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 12), (c0, 4), &[WireLayoutCommand::MoveTo((-450.000, 765.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, 0.000)), WireLayoutCommand::MoveTo((-465.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 15), (c0, 5), &[WireLayoutCommand::MoveTo((-450.000, 795.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-45.000, 0.000)), WireLayoutCommand::MoveTo((-495.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 18), (c0, 6), &[WireLayoutCommand::MoveTo((-450.000, 825.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-75.000, 0.000)), WireLayoutCommand::MoveTo((-525.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 21), (c0, 7), &[WireLayoutCommand::MoveTo((-450.000, 855.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-105.000, 0.000)), WireLayoutCommand::MoveTo((-555.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 0), (c0, 8), &[]);
	circuit.connect((c0, 16), (c3, 0), &[]);
	circuit.connect((c0, 15), (c3, 1), &[]);
	circuit.connect((c0, 14), (c3, 2), &[]);
	circuit.connect((c0, 13), (c3, 3), &[]);
	circuit.connect((c0, 12), (c3, 4), &[]);
	circuit.connect((c0, 11), (c3, 5), &[]);
	circuit.connect((c0, 10), (c3, 6), &[]);
	circuit.connect((c0, 9), (c3, 7), &[]);
	circuit.connect((c3, 16), (c1, 1), &[WireLayoutCommand::MoveTo((650.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((597.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 15), (c1, 4), &[WireLayoutCommand::MoveTo((650.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((612.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 14), (c1, 7), &[WireLayoutCommand::MoveTo((650.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((627.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 13), (c1, 10), &[WireLayoutCommand::MoveTo((650.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((642.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 12), (c1, 13), &[WireLayoutCommand::MoveTo((650.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((657.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 11), (c1, 16), &[WireLayoutCommand::MoveTo((650.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((672.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 10), (c1, 19), &[WireLayoutCommand::MoveTo((650.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((687.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 9), (c1, 22), &[WireLayoutCommand::MoveTo((650.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((702.500, 750.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 0), (c3, 8), &[]);
	circuit.connect((c1, 2), (c5, 0), &[]);
	circuit.connect((c1, 5), (c5, 3), &[]);
	circuit.connect((c1, 8), (c5, 6), &[]);
	circuit.connect((c1, 11), (c5, 9), &[]);
	circuit.connect((c1, 14), (c5, 12), &[]);
	circuit.connect((c1, 17), (c5, 15), &[]);
	circuit.connect((c1, 20), (c5, 18), &[]);
	circuit.connect((c1, 23), (c5, 21), &[]);
	circuit.connect((c6, 9), (c5, 1), &[WireLayoutCommand::MoveTo((0.000, 1597.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 10), (c5, 4), &[WireLayoutCommand::MoveTo((0.000, 1612.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 11), (c5, 7), &[WireLayoutCommand::MoveTo((0.000, 1627.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 12), (c5, 10), &[WireLayoutCommand::MoveTo((0.000, 1642.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 13), (c5, 13), &[WireLayoutCommand::MoveTo((0.000, 1657.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 14), (c5, 16), &[WireLayoutCommand::MoveTo((0.000, 1672.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 15), (c5, 19), &[WireLayoutCommand::MoveTo((0.000, 1687.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 16), (c5, 22), &[WireLayoutCommand::MoveTo((0.000, 1702.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 2), (c7, 0), &[WireLayoutCommand::MoveTo((350.000, 1195.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 5), (c7, 1), &[WireLayoutCommand::MoveTo((350.000, 1225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 8), (c7, 2), &[WireLayoutCommand::MoveTo((350.000, 1255.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 11), (c7, 3), &[WireLayoutCommand::MoveTo((350.000, 1285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 14), (c7, 4), &[WireLayoutCommand::MoveTo((350.000, 1315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 17), (c7, 5), &[WireLayoutCommand::MoveTo((350.000, 1345.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 20), (c7, 6), &[WireLayoutCommand::MoveTo((350.000, 1375.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 23), (c7, 7), &[WireLayoutCommand::MoveTo((350.000, 1405.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 0), (c6, 8), &[]);
	circuit.connect((c9, 7), (c6, 7), &[WireLayoutCommand::MoveTo((-525.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 6), (c6, 6), &[WireLayoutCommand::MoveTo((-575.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 5), (c6, 5), &[WireLayoutCommand::MoveTo((-625.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 4), (c6, 4), &[WireLayoutCommand::MoveTo((-675.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 3), (c6, 3), &[WireLayoutCommand::MoveTo((-725.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 2), (c6, 2), &[WireLayoutCommand::MoveTo((-775.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 1), (c6, 1), &[WireLayoutCommand::MoveTo((-825.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 0), (c6, 0), &[WireLayoutCommand::MoveTo((-875.000, 1650.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	
	circuit
}

pub fn register_bus_example() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Register, (0.000, 0.000));
	let c1 = add!(circuit, Register, (1000.000, 0.000));
	let c2 = add!(circuit, MultiJunction, (3.844, 850.000), 16);
	let c3 = add!(circuit, Register, (2000.000, 0.000));
	let c4 = add!(circuit, Register, (3750.000, 0.000));
	let c5 = add!(circuit, Register, (4750.000, 0.000));
	let c6 = add!(circuit, MultiJunction, (2000.000, 850.000), 16);
	let c7 = add!(circuit, MultiJunction, (3750.000, 850.000), 16);
	let c8 = add!(circuit, Adder, (5450.000, -1299.109), 16);
	let c9 = add!(circuit, MultiJunction, (1000.000, 850.000), 16);
	let c10 = add!(circuit, MultiTriStateBuffer, (6050.000, -1300.000), 16);
	let c11 = add!(circuit, MultiTriStateBuffer, (-650.000, 1400.000), 16);
	let c12 = add!(circuit, MultiBulb, (0.000, -850.000), 16);
	let c13 = add!(circuit, MultiBulb, (1000.000, -850.000), 16);
	let c14 = add!(circuit, MultiBulb, (2000.000, -850.000), 16);
	let c15 = add!(circuit, MultiSwitch, (-1250.000, 1100.000), 16);
	let c16 = add!(circuit, Junction, (500.000, -1500.000), 3);
	let c17 = add!(circuit, Junction, (1500.000, -1500.000), 3);
	let c18 = add!(circuit, Junction, (3250.000, -1500.000), 3);
	let c19 = add!(circuit, Junction, (-600.000, -1500.000), 3);
	let c20 = add!(circuit, Switch, (-650.000, 800.000));
	let c21 = add!(circuit, Switch, (550.000, -150.000));
	let c22 = add!(circuit, Switch, (550.000, 150.000));
	let c23 = add!(circuit, Switch, (1550.000, -150.000));
	let c24 = add!(circuit, Switch, (-450.000, -150.000));
	let c25 = add!(circuit, Switch, (-450.000, 150.000));
	let c26 = add!(circuit, Switch, (1550.000, 150.000));
	let c27 = add!(circuit, Switch, (6050.000, -1900.000));
	let c28 = add!(circuit, Switch, (3300.000, -150.000));
	let c29 = add!(circuit, Switch, (3300.000, 150.000));
	let c30 = add!(circuit, Switch, (4300.000, -150.000));
	let c31 = add!(circuit, Switch, (4300.000, 150.000));
	let c32 = add!(circuit, MultiJunction, (4750.000, 850.000), 16);
	let c33 = add!(circuit, MultiJunction, (6700.000, 850.000), 16);
	let c34 = add!(circuit, MultiJunction, (8150.000, 850.000), 16);
	let c35 = add!(circuit, MultiJunction, (10050.000, 850.000), 16);
	let c36 = add!(circuit, Register, (8150.000, 0.000));
	let c37 = add!(circuit, Ram, (10050.000, -100.000));
	let c38 = add!(circuit, Junction, (-700.000, -1500.000), 3);
	let c39 = add!(circuit, Switch, (-850.000, -1500.000));
	let c40 = add!(circuit, Junction, (7450.000, -1250.000), 3);
	let c41 = add!(circuit, Switch, (7700.000, 150.000));
	let c42 = add!(circuit, Switch, (7700.000, -150.000));
	let c43 = add!(circuit, Switch, (9000.000, 50.000));
	let c44 = add!(circuit, Switch, (8850.000, -100.000));
	let c45 = add!(circuit, Switch, (9000.000, -250.000));
	
	circuit.connect((c0, 17), (c2, 0), &[WireLayoutCommand::MoveTo((-75.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((-120.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 18), (c2, 3), &[WireLayoutCommand::MoveTo((-65.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((-120.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 19), (c2, 6), &[WireLayoutCommand::MoveTo((-55.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((-120.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 20), (c2, 9), &[WireLayoutCommand::MoveTo((-45.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((-120.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 21), (c2, 12), &[WireLayoutCommand::MoveTo((-35.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((-120.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 22), (c2, 15), &[WireLayoutCommand::MoveTo((-25.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((-120.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 23), (c2, 18), &[WireLayoutCommand::MoveTo((-15.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((-120.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 24), (c2, 21), &[WireLayoutCommand::MoveTo((-5.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((-120.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 25), (c2, 24), &[WireLayoutCommand::MoveTo((5.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((120.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 26), (c2, 27), &[WireLayoutCommand::MoveTo((15.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((120.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 27), (c2, 30), &[WireLayoutCommand::MoveTo((25.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((120.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 28), (c2, 33), &[WireLayoutCommand::MoveTo((35.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((120.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 29), (c2, 36), &[WireLayoutCommand::MoveTo((45.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((120.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 30), (c2, 39), &[WireLayoutCommand::MoveTo((55.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((120.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 31), (c2, 42), &[WireLayoutCommand::MoveTo((65.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((120.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 32), (c2, 45), &[WireLayoutCommand::MoveTo((75.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((120.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 17), (c9, 0), &[WireLayoutCommand::MoveTo((925.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((880.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 18), (c9, 3), &[WireLayoutCommand::MoveTo((935.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((880.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 19), (c9, 6), &[WireLayoutCommand::MoveTo((945.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((880.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 20), (c9, 9), &[WireLayoutCommand::MoveTo((955.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((880.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 21), (c9, 12), &[WireLayoutCommand::MoveTo((965.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((880.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 22), (c9, 15), &[WireLayoutCommand::MoveTo((975.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((880.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 23), (c9, 18), &[WireLayoutCommand::MoveTo((985.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((880.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 24), (c9, 21), &[WireLayoutCommand::MoveTo((995.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((880.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 25), (c9, 24), &[WireLayoutCommand::MoveTo((1005.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((1120.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 26), (c9, 27), &[WireLayoutCommand::MoveTo((1015.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((1120.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 27), (c9, 30), &[WireLayoutCommand::MoveTo((1025.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((1120.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 28), (c9, 33), &[WireLayoutCommand::MoveTo((1035.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((1120.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 29), (c9, 36), &[WireLayoutCommand::MoveTo((1045.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((1120.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 30), (c9, 39), &[WireLayoutCommand::MoveTo((1055.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((1120.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 31), (c9, 42), &[WireLayoutCommand::MoveTo((1065.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((1120.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 32), (c9, 45), &[WireLayoutCommand::MoveTo((1075.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((1120.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 1), (c8, 31), &[WireLayoutCommand::MoveTo((4675.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 2), (c8, 30), &[WireLayoutCommand::MoveTo((4685.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 3), (c8, 29), &[WireLayoutCommand::MoveTo((4695.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 4), (c8, 28), &[WireLayoutCommand::MoveTo((4705.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 5), (c8, 27), &[WireLayoutCommand::MoveTo((4715.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 6), (c8, 26), &[WireLayoutCommand::MoveTo((4725.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 7), (c8, 25), &[WireLayoutCommand::MoveTo((4735.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 8), (c8, 24), &[WireLayoutCommand::MoveTo((4745.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 9), (c8, 23), &[WireLayoutCommand::MoveTo((4755.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 10), (c8, 22), &[WireLayoutCommand::MoveTo((4765.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 11), (c8, 21), &[WireLayoutCommand::MoveTo((4775.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 12), (c8, 20), &[WireLayoutCommand::MoveTo((4785.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 13), (c8, 19), &[WireLayoutCommand::MoveTo((4795.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 14), (c8, 18), &[WireLayoutCommand::MoveTo((4805.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 15), (c8, 17), &[WireLayoutCommand::MoveTo((4815.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 16), (c8, 16), &[WireLayoutCommand::MoveTo((4825.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 1), (c8, 15), &[WireLayoutCommand::MoveTo((3675.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 2), (c8, 14), &[WireLayoutCommand::MoveTo((3685.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 3), (c8, 13), &[WireLayoutCommand::MoveTo((3695.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 4), (c8, 12), &[WireLayoutCommand::MoveTo((3705.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 5), (c8, 11), &[WireLayoutCommand::MoveTo((3715.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 6), (c8, 10), &[WireLayoutCommand::MoveTo((3725.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 7), (c8, 9), &[WireLayoutCommand::MoveTo((3735.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 8), (c8, 8), &[WireLayoutCommand::MoveTo((3745.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 9), (c8, 7), &[WireLayoutCommand::MoveTo((3755.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 10), (c8, 6), &[WireLayoutCommand::MoveTo((3765.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 11), (c8, 5), &[WireLayoutCommand::MoveTo((3775.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 12), (c8, 4), &[WireLayoutCommand::MoveTo((3785.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 13), (c8, 3), &[WireLayoutCommand::MoveTo((3795.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 14), (c8, 2), &[WireLayoutCommand::MoveTo((3805.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 15), (c8, 1), &[WireLayoutCommand::MoveTo((3815.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 16), (c8, 0), &[WireLayoutCommand::MoveTo((3825.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c8, 47), (c10, 0), &[]);
	circuit.connect((c8, 46), (c10, 1), &[]);
	circuit.connect((c8, 45), (c10, 2), &[]);
	circuit.connect((c8, 44), (c10, 3), &[]);
	circuit.connect((c8, 43), (c10, 4), &[]);
	circuit.connect((c8, 42), (c10, 5), &[]);
	circuit.connect((c8, 41), (c10, 6), &[]);
	circuit.connect((c8, 40), (c10, 7), &[]);
	circuit.connect((c8, 39), (c10, 8), &[]);
	circuit.connect((c8, 38), (c10, 9), &[]);
	circuit.connect((c8, 37), (c10, 10), &[]);
	circuit.connect((c8, 36), (c10, 11), &[]);
	circuit.connect((c8, 35), (c10, 12), &[]);
	circuit.connect((c8, 34), (c10, 13), &[]);
	circuit.connect((c8, 33), (c10, 14), &[]);
	circuit.connect((c8, 32), (c10, 15), &[]);
	circuit.connect((c3, 17), (c6, 0), &[WireLayoutCommand::MoveTo((1925.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((1880.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 18), (c6, 3), &[WireLayoutCommand::MoveTo((1935.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((1880.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 19), (c6, 6), &[WireLayoutCommand::MoveTo((1945.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((1880.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 20), (c6, 9), &[WireLayoutCommand::MoveTo((1955.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((1880.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 21), (c6, 12), &[WireLayoutCommand::MoveTo((1965.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((1880.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 22), (c6, 15), &[WireLayoutCommand::MoveTo((1975.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((1880.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 23), (c6, 18), &[WireLayoutCommand::MoveTo((1985.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((1880.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 24), (c6, 21), &[WireLayoutCommand::MoveTo((1995.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((1880.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 25), (c6, 24), &[WireLayoutCommand::MoveTo((2005.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((2120.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 26), (c6, 27), &[WireLayoutCommand::MoveTo((2015.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((2120.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 27), (c6, 30), &[WireLayoutCommand::MoveTo((2025.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((2120.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 28), (c6, 33), &[WireLayoutCommand::MoveTo((2035.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((2120.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 29), (c6, 36), &[WireLayoutCommand::MoveTo((2045.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((2120.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 30), (c6, 39), &[WireLayoutCommand::MoveTo((2055.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((2120.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 31), (c6, 42), &[WireLayoutCommand::MoveTo((2065.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((2120.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 32), (c6, 45), &[WireLayoutCommand::MoveTo((2075.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((2120.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 17), (c7, 0), &[WireLayoutCommand::MoveTo((3675.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((3630.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 18), (c7, 3), &[WireLayoutCommand::MoveTo((3685.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((3630.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 19), (c7, 6), &[WireLayoutCommand::MoveTo((3695.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((3630.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 20), (c7, 9), &[WireLayoutCommand::MoveTo((3705.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((3630.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 21), (c7, 12), &[WireLayoutCommand::MoveTo((3715.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((3630.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 22), (c7, 15), &[WireLayoutCommand::MoveTo((3725.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((3630.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 23), (c7, 18), &[WireLayoutCommand::MoveTo((3735.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((3630.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 24), (c7, 21), &[WireLayoutCommand::MoveTo((3745.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((3630.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 25), (c7, 24), &[WireLayoutCommand::MoveTo((3755.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((3870.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 26), (c7, 27), &[WireLayoutCommand::MoveTo((3765.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((3870.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 27), (c7, 30), &[WireLayoutCommand::MoveTo((3775.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((3870.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 28), (c7, 33), &[WireLayoutCommand::MoveTo((3785.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((3870.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 29), (c7, 36), &[WireLayoutCommand::MoveTo((3795.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((3870.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 30), (c7, 39), &[WireLayoutCommand::MoveTo((3805.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((3870.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 31), (c7, 42), &[WireLayoutCommand::MoveTo((3815.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((3870.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 32), (c7, 45), &[WireLayoutCommand::MoveTo((3825.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((3870.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 1), (c12, 0), &[WireLayoutCommand::MoveTo((-75.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((-200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 2), (c12, 1), &[WireLayoutCommand::MoveTo((-65.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((-200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 3), (c12, 2), &[WireLayoutCommand::MoveTo((-55.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 4), (c12, 3), &[WireLayoutCommand::MoveTo((-45.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((-200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 5), (c12, 4), &[WireLayoutCommand::MoveTo((-35.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((-200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 6), (c12, 5), &[WireLayoutCommand::MoveTo((-25.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 7), (c12, 6), &[WireLayoutCommand::MoveTo((-15.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((-200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 8), (c12, 7), &[WireLayoutCommand::MoveTo((-5.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((-200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 9), (c12, 8), &[WireLayoutCommand::MoveTo((5.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 10), (c12, 9), &[WireLayoutCommand::MoveTo((15.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 11), (c12, 10), &[WireLayoutCommand::MoveTo((25.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 12), (c12, 11), &[WireLayoutCommand::MoveTo((35.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 13), (c12, 12), &[WireLayoutCommand::MoveTo((45.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 14), (c12, 13), &[WireLayoutCommand::MoveTo((55.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 15), (c12, 14), &[WireLayoutCommand::MoveTo((65.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 16), (c12, 15), &[WireLayoutCommand::MoveTo((75.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 1), (c13, 0), &[WireLayoutCommand::MoveTo((925.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((800.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 2), (c13, 1), &[WireLayoutCommand::MoveTo((935.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((800.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 3), (c13, 2), &[WireLayoutCommand::MoveTo((945.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((800.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 4), (c13, 3), &[WireLayoutCommand::MoveTo((955.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((800.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 5), (c13, 4), &[WireLayoutCommand::MoveTo((965.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((800.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 6), (c13, 5), &[WireLayoutCommand::MoveTo((975.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((800.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 7), (c13, 6), &[WireLayoutCommand::MoveTo((985.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((800.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 8), (c13, 7), &[WireLayoutCommand::MoveTo((995.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((800.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 9), (c13, 8), &[WireLayoutCommand::MoveTo((1005.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((1200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 10), (c13, 9), &[WireLayoutCommand::MoveTo((1015.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((1200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 11), (c13, 10), &[WireLayoutCommand::MoveTo((1025.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((1200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 12), (c13, 11), &[WireLayoutCommand::MoveTo((1035.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((1200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 13), (c13, 12), &[WireLayoutCommand::MoveTo((1045.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((1200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 14), (c13, 13), &[WireLayoutCommand::MoveTo((1055.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((1200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 15), (c13, 14), &[WireLayoutCommand::MoveTo((1065.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((1200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 16), (c13, 15), &[WireLayoutCommand::MoveTo((1075.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((1200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 1), (c14, 0), &[WireLayoutCommand::MoveTo((1925.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((1800.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 2), (c14, 1), &[WireLayoutCommand::MoveTo((1935.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((1800.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 3), (c14, 2), &[WireLayoutCommand::MoveTo((1945.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((1800.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 4), (c14, 3), &[WireLayoutCommand::MoveTo((1955.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((1800.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 5), (c14, 4), &[WireLayoutCommand::MoveTo((1965.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((1800.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 6), (c14, 5), &[WireLayoutCommand::MoveTo((1975.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((1800.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 7), (c14, 6), &[WireLayoutCommand::MoveTo((1985.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((1800.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 8), (c14, 7), &[WireLayoutCommand::MoveTo((1995.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((1800.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 9), (c14, 8), &[WireLayoutCommand::MoveTo((2005.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((2200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 10), (c14, 9), &[WireLayoutCommand::MoveTo((2015.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((2200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 11), (c14, 10), &[WireLayoutCommand::MoveTo((2025.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((2200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 12), (c14, 11), &[WireLayoutCommand::MoveTo((2035.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((2200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 13), (c14, 12), &[WireLayoutCommand::MoveTo((2045.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((2200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 14), (c14, 13), &[WireLayoutCommand::MoveTo((2055.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((2200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 15), (c14, 14), &[WireLayoutCommand::MoveTo((2065.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((2200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 16), (c14, 15), &[WireLayoutCommand::MoveTo((2075.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((2200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c15, 0), (c11, 0), &[WireLayoutCommand::MoveTo((-1625.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 1), (c11, 1), &[WireLayoutCommand::MoveTo((-1575.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 2), (c11, 2), &[WireLayoutCommand::MoveTo((-1525.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 3), (c11, 3), &[WireLayoutCommand::MoveTo((-1475.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 4), (c11, 4), &[WireLayoutCommand::MoveTo((-1425.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 5), (c11, 5), &[WireLayoutCommand::MoveTo((-1375.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 6), (c11, 6), &[WireLayoutCommand::MoveTo((-1325.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 7), (c11, 7), &[WireLayoutCommand::MoveTo((-1275.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 8), (c11, 8), &[WireLayoutCommand::MoveTo((-1225.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 9), (c11, 9), &[WireLayoutCommand::MoveTo((-1175.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 10), (c11, 10), &[WireLayoutCommand::MoveTo((-1125.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 11), (c11, 11), &[WireLayoutCommand::MoveTo((-1075.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 12), (c11, 12), &[WireLayoutCommand::MoveTo((-1025.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 13), (c11, 13), &[WireLayoutCommand::MoveTo((-975.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 14), (c11, 14), &[WireLayoutCommand::MoveTo((-925.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 15), (c11, 15), &[WireLayoutCommand::MoveTo((-875.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 17), (c2, 1), &[WireLayoutCommand::MoveTo((0.000, 1287.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 18), (c2, 4), &[WireLayoutCommand::MoveTo((0.000, 1302.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 19), (c2, 7), &[WireLayoutCommand::MoveTo((0.000, 1317.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 20), (c2, 10), &[WireLayoutCommand::MoveTo((0.000, 1332.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 21), (c2, 13), &[WireLayoutCommand::MoveTo((0.000, 1347.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 22), (c2, 16), &[WireLayoutCommand::MoveTo((0.000, 1362.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 23), (c2, 19), &[WireLayoutCommand::MoveTo((0.000, 1377.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 24), (c2, 22), &[WireLayoutCommand::MoveTo((0.000, 1392.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 25), (c2, 25), &[WireLayoutCommand::MoveTo((0.000, 1407.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 26), (c2, 28), &[WireLayoutCommand::MoveTo((0.000, 1422.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 27), (c2, 31), &[WireLayoutCommand::MoveTo((0.000, 1437.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 28), (c2, 34), &[WireLayoutCommand::MoveTo((0.000, 1452.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 29), (c2, 37), &[WireLayoutCommand::MoveTo((0.000, 1467.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 30), (c2, 40), &[WireLayoutCommand::MoveTo((0.000, 1482.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 31), (c2, 43), &[WireLayoutCommand::MoveTo((0.000, 1497.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c11, 32), (c2, 46), &[WireLayoutCommand::MoveTo((0.000, 1512.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 2), (c9, 1), &[]);
	circuit.connect((c2, 5), (c9, 4), &[]);
	circuit.connect((c2, 8), (c9, 7), &[]);
	circuit.connect((c2, 11), (c9, 10), &[]);
	circuit.connect((c2, 14), (c9, 13), &[]);
	circuit.connect((c2, 17), (c9, 16), &[]);
	circuit.connect((c2, 20), (c9, 19), &[]);
	circuit.connect((c2, 23), (c9, 22), &[]);
	circuit.connect((c2, 26), (c9, 25), &[]);
	circuit.connect((c2, 29), (c9, 28), &[]);
	circuit.connect((c2, 32), (c9, 31), &[]);
	circuit.connect((c2, 35), (c9, 34), &[]);
	circuit.connect((c2, 38), (c9, 37), &[]);
	circuit.connect((c2, 41), (c9, 40), &[]);
	circuit.connect((c2, 44), (c9, 43), &[]);
	circuit.connect((c2, 47), (c9, 46), &[]);
	circuit.connect((c9, 2), (c6, 1), &[]);
	circuit.connect((c9, 5), (c6, 4), &[]);
	circuit.connect((c9, 8), (c6, 7), &[]);
	circuit.connect((c9, 11), (c6, 10), &[]);
	circuit.connect((c9, 14), (c6, 13), &[]);
	circuit.connect((c9, 17), (c6, 16), &[]);
	circuit.connect((c9, 20), (c6, 19), &[]);
	circuit.connect((c9, 23), (c6, 22), &[]);
	circuit.connect((c9, 26), (c6, 25), &[]);
	circuit.connect((c9, 29), (c6, 28), &[]);
	circuit.connect((c9, 32), (c6, 31), &[]);
	circuit.connect((c9, 35), (c6, 34), &[]);
	circuit.connect((c9, 38), (c6, 37), &[]);
	circuit.connect((c9, 41), (c6, 40), &[]);
	circuit.connect((c9, 44), (c6, 43), &[]);
	circuit.connect((c9, 47), (c6, 46), &[]);
	circuit.connect((c6, 2), (c7, 1), &[]);
	circuit.connect((c6, 5), (c7, 4), &[]);
	circuit.connect((c6, 8), (c7, 7), &[]);
	circuit.connect((c6, 11), (c7, 10), &[]);
	circuit.connect((c6, 14), (c7, 13), &[]);
	circuit.connect((c6, 17), (c7, 16), &[]);
	circuit.connect((c6, 20), (c7, 19), &[]);
	circuit.connect((c6, 23), (c7, 22), &[]);
	circuit.connect((c6, 26), (c7, 25), &[]);
	circuit.connect((c6, 29), (c7, 28), &[]);
	circuit.connect((c6, 32), (c7, 31), &[]);
	circuit.connect((c6, 35), (c7, 34), &[]);
	circuit.connect((c6, 38), (c7, 37), &[]);
	circuit.connect((c6, 41), (c7, 40), &[]);
	circuit.connect((c6, 44), (c7, 43), &[]);
	circuit.connect((c6, 47), (c7, 46), &[]);
	circuit.connect((c19, 1), (c0, 33), &[WireLayoutCommand::MoveTo((-600.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c19, 2), (c16, 0), &[]);
	circuit.connect((c16, 1), (c1, 33), &[WireLayoutCommand::MoveTo((500.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 2), (c17, 0), &[]);
	circuit.connect((c17, 1), (c3, 33), &[WireLayoutCommand::MoveTo((1500.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c17, 2), (c18, 0), &[]);
	circuit.connect((c18, 1), (c4, 33), &[WireLayoutCommand::MoveTo((3250.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c18, 2), (c5, 33), &[WireLayoutCommand::MoveTo((3450.000, -1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((3450.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4250.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4250.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c20, 0), (c11, 16), &[]);
	circuit.connect((c21, 0), (c1, 0), &[WireLayoutCommand::MoveTo((550.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c22, 0), (c1, 34), &[WireLayoutCommand::MoveTo((550.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c24, 0), (c0, 0), &[WireLayoutCommand::MoveTo((-450.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c25, 0), (c0, 34), &[WireLayoutCommand::MoveTo((-450.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c23, 0), (c3, 0), &[WireLayoutCommand::MoveTo((1550.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c26, 0), (c3, 34), &[WireLayoutCommand::MoveTo((1550.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c27, 0), (c10, 16), &[]);
	circuit.connect((c28, 0), (c4, 0), &[WireLayoutCommand::MoveTo((3300.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c29, 0), (c4, 34), &[WireLayoutCommand::MoveTo((3300.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c30, 0), (c5, 0), &[WireLayoutCommand::MoveTo((4300.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c31, 0), (c5, 34), &[WireLayoutCommand::MoveTo((4300.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c7, 2), (c32, 0), &[]);
	circuit.connect((c7, 5), (c32, 3), &[]);
	circuit.connect((c7, 8), (c32, 6), &[]);
	circuit.connect((c7, 11), (c32, 9), &[]);
	circuit.connect((c7, 14), (c32, 12), &[]);
	circuit.connect((c7, 17), (c32, 15), &[]);
	circuit.connect((c7, 20), (c32, 18), &[]);
	circuit.connect((c7, 23), (c32, 21), &[]);
	circuit.connect((c7, 26), (c32, 24), &[]);
	circuit.connect((c7, 29), (c32, 27), &[]);
	circuit.connect((c7, 32), (c32, 30), &[]);
	circuit.connect((c7, 35), (c32, 33), &[]);
	circuit.connect((c7, 38), (c32, 36), &[]);
	circuit.connect((c7, 41), (c32, 39), &[]);
	circuit.connect((c7, 44), (c32, 42), &[]);
	circuit.connect((c7, 47), (c32, 45), &[]);
	circuit.connect((c5, 17), (c32, 1), &[WireLayoutCommand::MoveTo((4675.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((4630.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 18), (c32, 4), &[WireLayoutCommand::MoveTo((4685.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((4630.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 19), (c32, 7), &[WireLayoutCommand::MoveTo((4695.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((4630.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 20), (c32, 10), &[WireLayoutCommand::MoveTo((4705.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((4630.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 21), (c32, 13), &[WireLayoutCommand::MoveTo((4715.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((4630.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 22), (c32, 16), &[WireLayoutCommand::MoveTo((4725.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((4630.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 23), (c32, 19), &[WireLayoutCommand::MoveTo((4735.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((4630.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 24), (c32, 22), &[WireLayoutCommand::MoveTo((4745.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((4630.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 25), (c32, 25), &[WireLayoutCommand::MoveTo((4755.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((4870.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 26), (c32, 28), &[WireLayoutCommand::MoveTo((4765.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((4870.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 27), (c32, 31), &[WireLayoutCommand::MoveTo((4775.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((4870.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 28), (c32, 34), &[WireLayoutCommand::MoveTo((4785.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((4870.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 29), (c32, 37), &[WireLayoutCommand::MoveTo((4795.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((4870.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 30), (c32, 40), &[WireLayoutCommand::MoveTo((4805.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((4870.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 31), (c32, 43), &[WireLayoutCommand::MoveTo((4815.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((4870.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 32), (c32, 46), &[WireLayoutCommand::MoveTo((4825.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((4870.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c32, 2), (c33, 0), &[]);
	circuit.connect((c32, 5), (c33, 3), &[]);
	circuit.connect((c32, 8), (c33, 6), &[]);
	circuit.connect((c32, 11), (c33, 9), &[]);
	circuit.connect((c32, 14), (c33, 12), &[]);
	circuit.connect((c32, 17), (c33, 15), &[]);
	circuit.connect((c32, 20), (c33, 18), &[]);
	circuit.connect((c32, 23), (c33, 21), &[]);
	circuit.connect((c32, 26), (c33, 24), &[]);
	circuit.connect((c32, 29), (c33, 27), &[]);
	circuit.connect((c32, 32), (c33, 30), &[]);
	circuit.connect((c32, 35), (c33, 33), &[]);
	circuit.connect((c32, 38), (c33, 36), &[]);
	circuit.connect((c32, 41), (c33, 39), &[]);
	circuit.connect((c32, 44), (c33, 42), &[]);
	circuit.connect((c32, 47), (c33, 45), &[]);
	circuit.connect((c10, 17), (c33, 1), &[WireLayoutCommand::MoveTo((6700.000, -1412.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 18), (c33, 4), &[WireLayoutCommand::MoveTo((6700.000, -1397.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 19), (c33, 7), &[WireLayoutCommand::MoveTo((6700.000, -1382.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 20), (c33, 10), &[WireLayoutCommand::MoveTo((6700.000, -1367.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 21), (c33, 13), &[WireLayoutCommand::MoveTo((6700.000, -1352.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 22), (c33, 16), &[WireLayoutCommand::MoveTo((6700.000, -1337.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 23), (c33, 19), &[WireLayoutCommand::MoveTo((6700.000, -1322.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 24), (c33, 22), &[WireLayoutCommand::MoveTo((6700.000, -1307.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 25), (c33, 25), &[WireLayoutCommand::MoveTo((6700.000, -1292.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 26), (c33, 28), &[WireLayoutCommand::MoveTo((6700.000, -1277.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 27), (c33, 31), &[WireLayoutCommand::MoveTo((6700.000, -1262.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 28), (c33, 34), &[WireLayoutCommand::MoveTo((6700.000, -1247.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 29), (c33, 37), &[WireLayoutCommand::MoveTo((6700.000, -1232.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 30), (c33, 40), &[WireLayoutCommand::MoveTo((6700.000, -1217.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 31), (c33, 43), &[WireLayoutCommand::MoveTo((6700.000, -1202.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 32), (c33, 46), &[WireLayoutCommand::MoveTo((6700.000, -1187.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c33, 2), (c34, 0), &[]);
	circuit.connect((c33, 5), (c34, 3), &[]);
	circuit.connect((c33, 8), (c34, 6), &[]);
	circuit.connect((c33, 11), (c34, 9), &[]);
	circuit.connect((c33, 14), (c34, 12), &[]);
	circuit.connect((c33, 17), (c34, 15), &[]);
	circuit.connect((c33, 20), (c34, 18), &[]);
	circuit.connect((c33, 23), (c34, 21), &[]);
	circuit.connect((c33, 26), (c34, 24), &[]);
	circuit.connect((c33, 29), (c34, 27), &[]);
	circuit.connect((c33, 32), (c34, 30), &[]);
	circuit.connect((c33, 35), (c34, 33), &[]);
	circuit.connect((c33, 38), (c34, 36), &[]);
	circuit.connect((c33, 41), (c34, 39), &[]);
	circuit.connect((c33, 44), (c34, 42), &[]);
	circuit.connect((c33, 47), (c34, 45), &[]);
	circuit.connect((c36, 17), (c34, 1), &[WireLayoutCommand::MoveTo((8075.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((8030.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 18), (c34, 4), &[WireLayoutCommand::MoveTo((8085.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((8030.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 19), (c34, 7), &[WireLayoutCommand::MoveTo((8095.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((8030.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 20), (c34, 10), &[WireLayoutCommand::MoveTo((8105.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((8030.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 21), (c34, 13), &[WireLayoutCommand::MoveTo((8115.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((8030.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 22), (c34, 16), &[WireLayoutCommand::MoveTo((8125.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((8030.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 23), (c34, 19), &[WireLayoutCommand::MoveTo((8135.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((8030.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 24), (c34, 22), &[WireLayoutCommand::MoveTo((8145.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((8030.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 25), (c34, 25), &[WireLayoutCommand::MoveTo((8155.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((8270.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 26), (c34, 28), &[WireLayoutCommand::MoveTo((8165.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((8270.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 27), (c34, 31), &[WireLayoutCommand::MoveTo((8175.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((8270.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 28), (c34, 34), &[WireLayoutCommand::MoveTo((8185.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((8270.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 29), (c34, 37), &[WireLayoutCommand::MoveTo((8195.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((8270.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 30), (c34, 40), &[WireLayoutCommand::MoveTo((8205.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((8270.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 31), (c34, 43), &[WireLayoutCommand::MoveTo((8215.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((8270.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 32), (c34, 46), &[WireLayoutCommand::MoveTo((8225.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((8270.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c34, 2), (c35, 0), &[]);
	circuit.connect((c34, 5), (c35, 3), &[]);
	circuit.connect((c34, 8), (c35, 6), &[]);
	circuit.connect((c34, 11), (c35, 9), &[]);
	circuit.connect((c34, 14), (c35, 12), &[]);
	circuit.connect((c34, 17), (c35, 15), &[]);
	circuit.connect((c34, 20), (c35, 18), &[]);
	circuit.connect((c34, 23), (c35, 21), &[]);
	circuit.connect((c34, 26), (c35, 24), &[]);
	circuit.connect((c34, 29), (c35, 27), &[]);
	circuit.connect((c34, 32), (c35, 30), &[]);
	circuit.connect((c34, 35), (c35, 33), &[]);
	circuit.connect((c34, 38), (c35, 36), &[]);
	circuit.connect((c34, 41), (c35, 39), &[]);
	circuit.connect((c34, 44), (c35, 42), &[]);
	circuit.connect((c34, 47), (c35, 45), &[]);
	circuit.connect((c37, 20), (c35, 1), &[WireLayoutCommand::MoveTo((9975.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((9930.000, 465.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 21), (c35, 4), &[WireLayoutCommand::MoveTo((9985.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((9930.000, 475.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 22), (c35, 7), &[WireLayoutCommand::MoveTo((9995.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((9930.000, 485.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 23), (c35, 10), &[WireLayoutCommand::MoveTo((10005.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((9930.000, 495.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 24), (c35, 13), &[WireLayoutCommand::MoveTo((10015.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((9930.000, 505.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 25), (c35, 16), &[WireLayoutCommand::MoveTo((10025.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((9930.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 26), (c35, 19), &[WireLayoutCommand::MoveTo((10035.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((9930.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 27), (c35, 22), &[WireLayoutCommand::MoveTo((10045.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((9930.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 28), (c35, 25), &[WireLayoutCommand::MoveTo((10055.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((10170.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 29), (c35, 28), &[WireLayoutCommand::MoveTo((10065.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((10170.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 30), (c35, 31), &[WireLayoutCommand::MoveTo((10075.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((10170.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 31), (c35, 34), &[WireLayoutCommand::MoveTo((10085.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((10170.000, 505.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 32), (c35, 37), &[WireLayoutCommand::MoveTo((10095.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((10170.000, 495.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 33), (c35, 40), &[WireLayoutCommand::MoveTo((10105.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((10170.000, 485.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 34), (c35, 43), &[WireLayoutCommand::MoveTo((10115.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((10170.000, 475.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c37, 35), (c35, 46), &[WireLayoutCommand::MoveTo((10125.000, 500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((10170.000, 465.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 1), (c37, 4), &[WireLayoutCommand::MoveTo((8075.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((10050.000, -925.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 2), (c37, 5), &[WireLayoutCommand::MoveTo((8085.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -65.000)), WireLayoutCommand::MoveTo((10050.000, -915.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 3), (c37, 6), &[WireLayoutCommand::MoveTo((8095.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -55.000)), WireLayoutCommand::MoveTo((10050.000, -905.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 4), (c37, 7), &[WireLayoutCommand::MoveTo((8105.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((10050.000, -895.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 5), (c37, 8), &[WireLayoutCommand::MoveTo((8115.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((10050.000, -885.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 6), (c37, 9), &[WireLayoutCommand::MoveTo((8125.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((10050.000, -875.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 7), (c37, 10), &[WireLayoutCommand::MoveTo((8135.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((10050.000, -865.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 8), (c37, 11), &[WireLayoutCommand::MoveTo((8145.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((10050.000, -855.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 9), (c37, 12), &[WireLayoutCommand::MoveTo((8155.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((10050.000, -845.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 10), (c37, 13), &[WireLayoutCommand::MoveTo((8165.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((10050.000, -835.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 11), (c37, 14), &[WireLayoutCommand::MoveTo((8175.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((10050.000, -825.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 12), (c37, 15), &[WireLayoutCommand::MoveTo((8185.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((10050.000, -815.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 13), (c37, 16), &[WireLayoutCommand::MoveTo((8195.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((10050.000, -805.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 14), (c37, 17), &[WireLayoutCommand::MoveTo((8205.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 55.000)), WireLayoutCommand::MoveTo((10050.000, -795.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 15), (c37, 18), &[WireLayoutCommand::MoveTo((8215.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 65.000)), WireLayoutCommand::MoveTo((10050.000, -785.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c36, 16), (c37, 19), &[WireLayoutCommand::MoveTo((8225.000, -850.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((10050.000, -775.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c39, 0), (c38, 0), &[]);
	circuit.connect((c38, 1), (c19, 0), &[]);
	circuit.connect((c38, 2), (c40, 0), &[WireLayoutCommand::MoveTo((-700.000, -2600.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((7450.000, -2600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c40, 1), (c36, 33), &[WireLayoutCommand::MoveTo((7450.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c40, 2), (c37, 1), &[WireLayoutCommand::MoveTo((8600.000, -1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((8600.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c42, 0), (c36, 0), &[WireLayoutCommand::MoveTo((7700.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c41, 0), (c36, 34), &[WireLayoutCommand::MoveTo((7700.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c37, 3), (c43, 0), &[WireLayoutCommand::MoveTo((9000.000, -90.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c44, 0), (c37, 2), &[WireLayoutCommand::MoveTo((8970.000, -100.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((8970.000, -130.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c45, 0), (c37, 0), &[WireLayoutCommand::MoveTo((9000.000, -170.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	
	circuit
}

pub fn get_computer_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, RegisterFile, (-500.000, -50.000));
	let c1 = add!(circuit, Ram, (4950.000, -300.000));
	let c2 = add!(circuit, Register, (3350.000, -200.000));
	let c3 = add!(circuit, Register, (750.000, -200.000));
	let c4 = add!(circuit, MultiJunction, (-2050.000, 600.000), 16);
	let c5 = add!(circuit, MultiJunction, (-500.000, 600.000), 16);
	let c6 = add!(circuit, MultiJunction, (750.000, 600.000), 16);
	let c7 = add!(circuit, MultiJunction, (3350.000, 600.000), 16);
	let c8 = add!(circuit, MultiJunction, (4950.000, 600.000), 16);
	let c9 = add!(circuit, Adder, (1650.000, -1300.000), 16);
	let c10 = add!(circuit, MultiTriStateBuffer, (2250.000, -1300.000), 16);
	let c11 = add!(circuit, MultiJunction, (2700.000, 600.000), 16);
	let c12 = add!(circuit, Switch, (1300.000, 1950.000));
	let c13 = add!(circuit, Junction, (1300.000, 1600.000), 3);
	let c14 = add!(circuit, Junction, (300.000, 1600.000), 3);
	let c15 = add!(circuit, Junction, (2950.000, 1600.000), 3);
	let c16 = add!(circuit, ControlUnit, (-2050.000, -450.000));
	let c17 = add!(circuit, Switch, (1100.000, 1950.000));
	
	let code = assemble(include_str!("./aasm/test.aasm").to_string()).unwrap()
		.chunks(2)
		.into_iter()
		.map(|bs| u16::from_be_bytes([bs[0], bs[1]]))
		.collect::<Vec<_>>();

	circuit.components[c1].internals
		.get_circuit_mut()
		.unwrap()
		.set_memory(0, &code);
	
	circuit.connect((c16, 0), (c4, 0), &[WireLayoutCommand::MoveTo((-2125.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((-2170.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 1), (c4, 3), &[WireLayoutCommand::MoveTo((-2115.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((-2170.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 2), (c4, 6), &[WireLayoutCommand::MoveTo((-2105.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((-2170.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 3), (c4, 9), &[WireLayoutCommand::MoveTo((-2095.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((-2170.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 4), (c4, 12), &[WireLayoutCommand::MoveTo((-2085.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((-2170.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 5), (c4, 15), &[WireLayoutCommand::MoveTo((-2075.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((-2170.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 6), (c4, 18), &[WireLayoutCommand::MoveTo((-2065.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((-2170.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 7), (c4, 21), &[WireLayoutCommand::MoveTo((-2055.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((-2170.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 8), (c4, 24), &[WireLayoutCommand::MoveTo((-2045.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((-1930.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 9), (c4, 27), &[WireLayoutCommand::MoveTo((-2035.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((-1930.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 10), (c4, 30), &[WireLayoutCommand::MoveTo((-2025.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((-1930.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 11), (c4, 33), &[WireLayoutCommand::MoveTo((-2015.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((-1930.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 12), (c4, 36), &[WireLayoutCommand::MoveTo((-2005.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((-1930.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 13), (c4, 39), &[WireLayoutCommand::MoveTo((-1995.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((-1930.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 14), (c4, 42), &[WireLayoutCommand::MoveTo((-1985.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((-1930.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 15), (c4, 45), &[WireLayoutCommand::MoveTo((-1975.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((-1930.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 0), (c5, 0), &[WireLayoutCommand::MoveTo((-612.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -52.500)), WireLayoutCommand::MoveTo((-620.000, 247.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 1), (c5, 3), &[WireLayoutCommand::MoveTo((-597.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -37.500)), WireLayoutCommand::MoveTo((-620.000, 262.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 2), (c5, 6), &[WireLayoutCommand::MoveTo((-582.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -22.500)), WireLayoutCommand::MoveTo((-620.000, 277.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 3), (c5, 9), &[WireLayoutCommand::MoveTo((-567.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -7.500)), WireLayoutCommand::MoveTo((-620.000, 292.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 4), (c5, 12), &[WireLayoutCommand::MoveTo((-552.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 7.500)), WireLayoutCommand::MoveTo((-620.000, 307.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 5), (c5, 15), &[WireLayoutCommand::MoveTo((-537.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 22.500)), WireLayoutCommand::MoveTo((-620.000, 322.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 6), (c5, 18), &[WireLayoutCommand::MoveTo((-522.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 37.500)), WireLayoutCommand::MoveTo((-620.000, 337.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 7), (c5, 21), &[WireLayoutCommand::MoveTo((-507.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 52.500)), WireLayoutCommand::MoveTo((-620.000, 352.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 8), (c5, 24), &[WireLayoutCommand::MoveTo((-492.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 52.500)), WireLayoutCommand::MoveTo((-380.000, 352.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 9), (c5, 27), &[WireLayoutCommand::MoveTo((-477.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 37.500)), WireLayoutCommand::MoveTo((-380.000, 337.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 10), (c5, 30), &[WireLayoutCommand::MoveTo((-462.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 22.500)), WireLayoutCommand::MoveTo((-380.000, 322.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 11), (c5, 33), &[WireLayoutCommand::MoveTo((-447.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 7.500)), WireLayoutCommand::MoveTo((-380.000, 307.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 12), (c5, 36), &[WireLayoutCommand::MoveTo((-432.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -7.500)), WireLayoutCommand::MoveTo((-380.000, 292.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 13), (c5, 39), &[WireLayoutCommand::MoveTo((-417.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -22.500)), WireLayoutCommand::MoveTo((-380.000, 277.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 14), (c5, 42), &[WireLayoutCommand::MoveTo((-402.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -37.500)), WireLayoutCommand::MoveTo((-380.000, 262.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 15), (c5, 45), &[WireLayoutCommand::MoveTo((-387.500, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -52.500)), WireLayoutCommand::MoveTo((-380.000, 247.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 17), (c6, 0), &[WireLayoutCommand::MoveTo((675.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((630.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 18), (c6, 3), &[WireLayoutCommand::MoveTo((685.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((630.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 19), (c6, 6), &[WireLayoutCommand::MoveTo((695.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((630.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 20), (c6, 9), &[WireLayoutCommand::MoveTo((705.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((630.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 21), (c6, 12), &[WireLayoutCommand::MoveTo((715.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((630.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 22), (c6, 15), &[WireLayoutCommand::MoveTo((725.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((630.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 23), (c6, 18), &[WireLayoutCommand::MoveTo((735.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((630.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 24), (c6, 21), &[WireLayoutCommand::MoveTo((745.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((630.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 25), (c6, 24), &[WireLayoutCommand::MoveTo((755.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((870.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 26), (c6, 27), &[WireLayoutCommand::MoveTo((765.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((870.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 27), (c6, 30), &[WireLayoutCommand::MoveTo((775.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((870.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 28), (c6, 33), &[WireLayoutCommand::MoveTo((785.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((870.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 29), (c6, 36), &[WireLayoutCommand::MoveTo((795.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((870.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 30), (c6, 39), &[WireLayoutCommand::MoveTo((805.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((870.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 31), (c6, 42), &[WireLayoutCommand::MoveTo((815.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((870.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 32), (c6, 45), &[WireLayoutCommand::MoveTo((825.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((870.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 17), (c7, 0), &[WireLayoutCommand::MoveTo((3275.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((3230.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 18), (c7, 3), &[WireLayoutCommand::MoveTo((3285.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((3230.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 19), (c7, 6), &[WireLayoutCommand::MoveTo((3295.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((3230.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 20), (c7, 9), &[WireLayoutCommand::MoveTo((3305.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((3230.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 21), (c7, 12), &[WireLayoutCommand::MoveTo((3315.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((3230.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 22), (c7, 15), &[WireLayoutCommand::MoveTo((3325.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((3230.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 23), (c7, 18), &[WireLayoutCommand::MoveTo((3335.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((3230.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 24), (c7, 21), &[WireLayoutCommand::MoveTo((3345.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((3230.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 25), (c7, 24), &[WireLayoutCommand::MoveTo((3355.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((3470.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 26), (c7, 27), &[WireLayoutCommand::MoveTo((3365.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((3470.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 27), (c7, 30), &[WireLayoutCommand::MoveTo((3375.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((3470.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 28), (c7, 33), &[WireLayoutCommand::MoveTo((3385.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((3470.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 29), (c7, 36), &[WireLayoutCommand::MoveTo((3395.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((3470.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 30), (c7, 39), &[WireLayoutCommand::MoveTo((3405.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((3470.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 31), (c7, 42), &[WireLayoutCommand::MoveTo((3415.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((3470.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 32), (c7, 45), &[WireLayoutCommand::MoveTo((3425.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((3470.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 20), (c8, 0), &[WireLayoutCommand::MoveTo((4875.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((4830.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 21), (c8, 3), &[WireLayoutCommand::MoveTo((4885.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((4830.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 22), (c8, 6), &[WireLayoutCommand::MoveTo((4895.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((4830.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 23), (c8, 9), &[WireLayoutCommand::MoveTo((4905.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((4830.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 24), (c8, 12), &[WireLayoutCommand::MoveTo((4915.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((4830.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 25), (c8, 15), &[WireLayoutCommand::MoveTo((4925.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((4830.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 26), (c8, 18), &[WireLayoutCommand::MoveTo((4935.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((4830.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 27), (c8, 21), &[WireLayoutCommand::MoveTo((4945.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((4830.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 28), (c8, 24), &[WireLayoutCommand::MoveTo((4955.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((5070.000, 335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 29), (c8, 27), &[WireLayoutCommand::MoveTo((4965.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((5070.000, 325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 30), (c8, 30), &[WireLayoutCommand::MoveTo((4975.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((5070.000, 315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 31), (c8, 33), &[WireLayoutCommand::MoveTo((4985.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((5070.000, 305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 32), (c8, 36), &[WireLayoutCommand::MoveTo((4995.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((5070.000, 295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 33), (c8, 39), &[WireLayoutCommand::MoveTo((5005.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((5070.000, 285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 34), (c8, 42), &[WireLayoutCommand::MoveTo((5015.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((5070.000, 275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 35), (c8, 45), &[WireLayoutCommand::MoveTo((5025.000, 300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((5070.000, 265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 1), (c5, 1), &[]);
	circuit.connect((c4, 4), (c5, 4), &[]);
	circuit.connect((c4, 7), (c5, 7), &[]);
	circuit.connect((c4, 10), (c5, 10), &[]);
	circuit.connect((c4, 13), (c5, 13), &[]);
	circuit.connect((c4, 16), (c5, 16), &[]);
	circuit.connect((c4, 19), (c5, 19), &[]);
	circuit.connect((c4, 22), (c5, 22), &[]);
	circuit.connect((c4, 25), (c5, 25), &[]);
	circuit.connect((c4, 28), (c5, 28), &[]);
	circuit.connect((c4, 31), (c5, 31), &[]);
	circuit.connect((c4, 34), (c5, 34), &[]);
	circuit.connect((c4, 37), (c5, 37), &[]);
	circuit.connect((c4, 40), (c5, 40), &[]);
	circuit.connect((c4, 43), (c5, 43), &[]);
	circuit.connect((c4, 46), (c5, 46), &[]);
	circuit.connect((c5, 2), (c6, 1), &[]);
	circuit.connect((c5, 5), (c6, 4), &[]);
	circuit.connect((c5, 8), (c6, 7), &[]);
	circuit.connect((c5, 11), (c6, 10), &[]);
	circuit.connect((c5, 14), (c6, 13), &[]);
	circuit.connect((c5, 17), (c6, 16), &[]);
	circuit.connect((c5, 20), (c6, 19), &[]);
	circuit.connect((c5, 23), (c6, 22), &[]);
	circuit.connect((c5, 26), (c6, 25), &[]);
	circuit.connect((c5, 29), (c6, 28), &[]);
	circuit.connect((c5, 32), (c6, 31), &[]);
	circuit.connect((c5, 35), (c6, 34), &[]);
	circuit.connect((c5, 38), (c6, 37), &[]);
	circuit.connect((c5, 41), (c6, 40), &[]);
	circuit.connect((c5, 44), (c6, 43), &[]);
	circuit.connect((c5, 47), (c6, 46), &[]);
	circuit.connect((c9, 47), (c10, 0), &[]);
	circuit.connect((c9, 46), (c10, 1), &[]);
	circuit.connect((c9, 45), (c10, 2), &[]);
	circuit.connect((c9, 44), (c10, 3), &[]);
	circuit.connect((c9, 43), (c10, 4), &[]);
	circuit.connect((c9, 42), (c10, 5), &[]);
	circuit.connect((c9, 41), (c10, 6), &[]);
	circuit.connect((c9, 40), (c10, 7), &[]);
	circuit.connect((c9, 39), (c10, 8), &[]);
	circuit.connect((c9, 38), (c10, 9), &[]);
	circuit.connect((c9, 37), (c10, 10), &[]);
	circuit.connect((c9, 36), (c10, 11), &[]);
	circuit.connect((c9, 35), (c10, 12), &[]);
	circuit.connect((c9, 34), (c10, 13), &[]);
	circuit.connect((c9, 33), (c10, 14), &[]);
	circuit.connect((c9, 32), (c10, 15), &[]);
	circuit.connect((c10, 17), (c11, 0), &[WireLayoutCommand::MoveTo((2700.000, -1412.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 18), (c11, 3), &[WireLayoutCommand::MoveTo((2700.000, -1397.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 19), (c11, 6), &[WireLayoutCommand::MoveTo((2700.000, -1382.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 20), (c11, 9), &[WireLayoutCommand::MoveTo((2700.000, -1367.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 21), (c11, 12), &[WireLayoutCommand::MoveTo((2700.000, -1352.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 22), (c11, 15), &[WireLayoutCommand::MoveTo((2700.000, -1337.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 23), (c11, 18), &[WireLayoutCommand::MoveTo((2700.000, -1322.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 24), (c11, 21), &[WireLayoutCommand::MoveTo((2700.000, -1307.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 25), (c11, 24), &[WireLayoutCommand::MoveTo((2700.000, -1292.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 26), (c11, 27), &[WireLayoutCommand::MoveTo((2700.000, -1277.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 27), (c11, 30), &[WireLayoutCommand::MoveTo((2700.000, -1262.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 28), (c11, 33), &[WireLayoutCommand::MoveTo((2700.000, -1247.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 29), (c11, 36), &[WireLayoutCommand::MoveTo((2700.000, -1232.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 30), (c11, 39), &[WireLayoutCommand::MoveTo((2700.000, -1217.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 31), (c11, 42), &[WireLayoutCommand::MoveTo((2700.000, -1202.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 32), (c11, 45), &[WireLayoutCommand::MoveTo((2700.000, -1187.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 2), (c11, 1), &[]);
	circuit.connect((c6, 5), (c11, 4), &[]);
	circuit.connect((c6, 8), (c11, 7), &[]);
	circuit.connect((c6, 11), (c11, 10), &[]);
	circuit.connect((c6, 14), (c11, 13), &[]);
	circuit.connect((c6, 17), (c11, 16), &[]);
	circuit.connect((c6, 20), (c11, 19), &[]);
	circuit.connect((c6, 23), (c11, 22), &[]);
	circuit.connect((c6, 26), (c11, 25), &[]);
	circuit.connect((c6, 29), (c11, 28), &[]);
	circuit.connect((c6, 32), (c11, 31), &[]);
	circuit.connect((c6, 35), (c11, 34), &[]);
	circuit.connect((c6, 38), (c11, 37), &[]);
	circuit.connect((c6, 41), (c11, 40), &[]);
	circuit.connect((c6, 44), (c11, 43), &[]);
	circuit.connect((c6, 47), (c11, 46), &[]);
	circuit.connect((c11, 2), (c7, 1), &[]);
	circuit.connect((c11, 5), (c7, 4), &[]);
	circuit.connect((c11, 8), (c7, 7), &[]);
	circuit.connect((c11, 11), (c7, 10), &[]);
	circuit.connect((c11, 14), (c7, 13), &[]);
	circuit.connect((c11, 17), (c7, 16), &[]);
	circuit.connect((c11, 20), (c7, 19), &[]);
	circuit.connect((c11, 23), (c7, 22), &[]);
	circuit.connect((c11, 26), (c7, 25), &[]);
	circuit.connect((c11, 29), (c7, 28), &[]);
	circuit.connect((c11, 32), (c7, 31), &[]);
	circuit.connect((c11, 35), (c7, 34), &[]);
	circuit.connect((c11, 38), (c7, 37), &[]);
	circuit.connect((c11, 41), (c7, 40), &[]);
	circuit.connect((c11, 44), (c7, 43), &[]);
	circuit.connect((c11, 47), (c7, 46), &[]);
	circuit.connect((c7, 2), (c8, 1), &[]);
	circuit.connect((c7, 5), (c8, 4), &[]);
	circuit.connect((c7, 8), (c8, 7), &[]);
	circuit.connect((c7, 11), (c8, 10), &[]);
	circuit.connect((c7, 14), (c8, 13), &[]);
	circuit.connect((c7, 17), (c8, 16), &[]);
	circuit.connect((c7, 20), (c8, 19), &[]);
	circuit.connect((c7, 23), (c8, 22), &[]);
	circuit.connect((c7, 26), (c8, 25), &[]);
	circuit.connect((c7, 29), (c8, 28), &[]);
	circuit.connect((c7, 32), (c8, 31), &[]);
	circuit.connect((c7, 35), (c8, 34), &[]);
	circuit.connect((c7, 38), (c8, 37), &[]);
	circuit.connect((c7, 41), (c8, 40), &[]);
	circuit.connect((c7, 44), (c8, 43), &[]);
	circuit.connect((c7, 47), (c8, 46), &[]);
	circuit.connect((c2, 1), (c1, 4), &[WireLayoutCommand::MoveTo((3275.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((4950.000, -1075.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 2), (c1, 5), &[WireLayoutCommand::MoveTo((3285.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -65.000)), WireLayoutCommand::MoveTo((4950.000, -1065.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 3), (c1, 6), &[WireLayoutCommand::MoveTo((3295.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -55.000)), WireLayoutCommand::MoveTo((4950.000, -1055.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 4), (c1, 7), &[WireLayoutCommand::MoveTo((3305.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((4950.000, -1045.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 5), (c1, 8), &[WireLayoutCommand::MoveTo((3315.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((4950.000, -1035.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 6), (c1, 9), &[WireLayoutCommand::MoveTo((3325.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((4950.000, -1025.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 7), (c1, 10), &[WireLayoutCommand::MoveTo((3335.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((4950.000, -1015.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 8), (c1, 11), &[WireLayoutCommand::MoveTo((3345.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((4950.000, -1005.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 9), (c1, 12), &[WireLayoutCommand::MoveTo((3355.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((4950.000, -995.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 10), (c1, 13), &[WireLayoutCommand::MoveTo((3365.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((4950.000, -985.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 11), (c1, 14), &[WireLayoutCommand::MoveTo((3375.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((4950.000, -975.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 12), (c1, 15), &[WireLayoutCommand::MoveTo((3385.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((4950.000, -965.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 13), (c1, 16), &[WireLayoutCommand::MoveTo((3395.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((4950.000, -955.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 14), (c1, 17), &[WireLayoutCommand::MoveTo((3405.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 55.000)), WireLayoutCommand::MoveTo((4950.000, -945.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 15), (c1, 18), &[WireLayoutCommand::MoveTo((3415.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 65.000)), WireLayoutCommand::MoveTo((4950.000, -935.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 16), (c1, 19), &[WireLayoutCommand::MoveTo((3425.000, -1000.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((4950.000, -925.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 1), (c9, 31), &[WireLayoutCommand::MoveTo((675.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 2), (c9, 30), &[WireLayoutCommand::MoveTo((685.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 3), (c9, 29), &[WireLayoutCommand::MoveTo((695.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 4), (c9, 28), &[WireLayoutCommand::MoveTo((705.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 5), (c9, 27), &[WireLayoutCommand::MoveTo((715.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 6), (c9, 26), &[WireLayoutCommand::MoveTo((725.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 7), (c9, 25), &[WireLayoutCommand::MoveTo((735.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 8), (c9, 24), &[WireLayoutCommand::MoveTo((745.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 9), (c9, 23), &[WireLayoutCommand::MoveTo((755.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 10), (c9, 22), &[WireLayoutCommand::MoveTo((765.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 11), (c9, 21), &[WireLayoutCommand::MoveTo((775.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 12), (c9, 20), &[WireLayoutCommand::MoveTo((785.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 13), (c9, 19), &[WireLayoutCommand::MoveTo((795.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 14), (c9, 18), &[WireLayoutCommand::MoveTo((805.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 15), (c9, 17), &[WireLayoutCommand::MoveTo((815.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 16), (c9, 16), &[WireLayoutCommand::MoveTo((825.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 36), (c9, 2), &[WireLayoutCommand::MoveTo((-1930.000, -1502.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 37), (c9, 1), &[WireLayoutCommand::MoveTo((-1920.000, -1502.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 32), (c0, 16), &[WireLayoutCommand::MoveTo((-1880.000, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-1150.000, -1465.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, 0.000)), WireLayoutCommand::MoveTo((-1135.000, -125.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 33), (c0, 17), &[WireLayoutCommand::MoveTo((-1870.000, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((-1150.000, -1455.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, 0.000)), WireLayoutCommand::MoveTo((-1145.000, -125.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 34), (c0, 18), &[WireLayoutCommand::MoveTo((-1860.000, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((-1150.000, -1445.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, -0.000)), WireLayoutCommand::MoveTo((-1155.000, -125.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 35), (c0, 19), &[WireLayoutCommand::MoveTo((-1850.000, -1450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-1150.000, -1435.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, -0.000)), WireLayoutCommand::MoveTo((-1165.000, -125.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 28), (c0, 20), &[WireLayoutCommand::MoveTo((-1810.000, -1350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-1810.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-1250.000, -1415.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, 0.000)), WireLayoutCommand::MoveTo((-1235.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 29), (c0, 21), &[WireLayoutCommand::MoveTo((-1800.000, -1350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-1800.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((-1250.000, -1405.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, 0.000)), WireLayoutCommand::MoveTo((-1245.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 30), (c0, 22), &[WireLayoutCommand::MoveTo((-1790.000, -1350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-1790.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((-1250.000, -1395.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, -0.000)), WireLayoutCommand::MoveTo((-1255.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 31), (c0, 23), &[WireLayoutCommand::MoveTo((-1780.000, -1350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-1780.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-1250.000, -1385.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, -0.000)), WireLayoutCommand::MoveTo((-1265.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 20), (c1, 2), &[WireLayoutCommand::MoveTo((-2280.000, -2400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((3900.000, -2400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((3900.000, -330.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 16), (c0, 24), &[WireLayoutCommand::MoveTo((-2320.000, -1350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-1300.000, -1350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1300.000, 25.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 43), (c0, 25), &[WireLayoutCommand::MoveTo((-2170.000, -1300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-1350.000, -1300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1350.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 17), (c3, 0), &[WireLayoutCommand::MoveTo((-2310.000, -1650.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((300.000, -1650.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((300.000, -250.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 44), (c3, 34), &[WireLayoutCommand::MoveTo((-2160.000, -1600.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((250.000, -1600.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((250.000, -200.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 18), (c10, 16), &[WireLayoutCommand::MoveTo((-2300.000, -2350.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((2250.000, -2350.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 19), (c2, 0), &[WireLayoutCommand::MoveTo((-2290.000, -2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((1150.000, -2300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((1150.000, -250.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 46), (c2, 34), &[WireLayoutCommand::MoveTo((-2140.000, -2250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((1200.000, -2250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((1200.000, -200.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 47), (c1, 3), &[WireLayoutCommand::MoveTo((-2130.000, -2450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((3850.000, -2450.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((3850.000, -290.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c14, 1), (c13, 0), &[]);
	circuit.connect((c13, 1), (c15, 0), &[]);
	circuit.connect((c12, 0), (c13, 2), &[]);
	circuit.connect((c14, 2), (c3, 33), &[WireLayoutCommand::MoveTo((300.000, -150.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 1), (c2, 33), &[WireLayoutCommand::MoveTo((2950.000, -150.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c15, 2), (c1, 1), &[WireLayoutCommand::MoveTo((3850.000, 1600.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((3850.000, -250.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c14, 0), (c0, 26), &[WireLayoutCommand::MoveTo((-1100.000, 1600.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-1100.000, 75.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c17, 0), (c16, 55), &[WireLayoutCommand::MoveTo((1100.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-2750.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-2750.000, -310.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	
	circuit
}

pub fn editor_example() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Switch, (0.000, 0.000));
	let c1 = add!(circuit, TriStateBuffer, (150.000, 0.000));
	let c2 = add!(circuit, Switch, (150.000, -150.000));
	let c3 = add!(circuit, TriStateBuffer, (150.000, 350.000));
	let c4 = add!(circuit, Switch, (0.000, 350.000));
	let c5 = add!(circuit, Switch, (150.000, 200.000));
	let c6 = add!(circuit, Junction, (350.000, 150.000), 3);
	let c7 = add!(circuit, Bulb, (500.000, 150.000));
	
	circuit.connect((c0, 0), (c1, 0), &[]);
	circuit.connect((c2, 0), (c1, 1), &[]);
	circuit.connect((c5, 0), (c3, 1), &[]);
	circuit.connect((c4, 0), (c3, 0), &[]);
	circuit.connect((c1, 2), (c6, 0), &[WireLayoutCommand::MoveTo((350.000, 0.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 2), (c6, 1), &[WireLayoutCommand::MoveTo((350.000, 350.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 2), (c7, 0), &[]);
	
	circuit
}

#[wasm_bindgen(start)]
pub fn start() {
	log!("ATLAS has started!");
	set_panic_hook();

	let program = include_str!("./aasm/test.aasm").to_string();
	crate::log!("{}", program);

	let mut vm = LowLevelAtlasVM::new();
	vm.run(program);

	crate::log!("{}", vm.memory.read_screen());
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use test::Bencher;

// 	#[test]
// 	fn test() {
// 		let mut circuit = bidirectional_example();
// 		circuit.toggle_switch(0);
// 		circuit.toggle_switch(1);
// 		circuit.toggle_switch(0);
// 	}

// 	#[bench]
// 	fn bench_simple_switch_circuit(b: &mut Bencher) {
// 		let mut circuit = example2();
// 		b.iter(|| circuit.toggle_switch(0));
// 	}

// 	#[bench]
// 	fn bench_not_gate(b: &mut Bencher) {
// 		let mut circuit = not_gate_example();
// 		b.iter(|| circuit.toggle_switch(0));
// 	}
// }
