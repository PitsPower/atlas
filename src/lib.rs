#![feature(test)]

extern crate test;

pub mod adder;
pub mod assembler;
pub mod bus;
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
	let c8 = add!(circuit, MultiJunction, (4750.000, 850.000), 16);
	let c9 = add!(circuit, Adder, (5450.000, -1299.109), 16);
	let c10 = add!(circuit, MultiJunction, (1000.000, 850.000), 16);
	let c11 = add!(circuit, MultiTriStateBuffer, (6050.000, -1300.000), 16);
	let c12 = add!(circuit, MultiTriStateBuffer, (-650.000, 1400.000), 16);
	let c13 = add!(circuit, MultiBulb, (0.000, -850.000), 16);
	let c14 = add!(circuit, MultiBulb, (1000.000, -850.000), 16);
	let c15 = add!(circuit, MultiBulb, (2000.000, -850.000), 16);
	let c16 = add!(circuit, MultiSwitch, (-1250.000, 1100.000), 16);
	let c17 = add!(circuit, Junction, (500.000, -1500.000), 3);
	let c18 = add!(circuit, Junction, (1500.000, -1500.000), 3);
	let c19 = add!(circuit, Junction, (3250.000, -1500.000), 3);
	let c20 = add!(circuit, Switch, (-900.000, -1500.000));
	let c21 = add!(circuit, Junction, (-600.000, -1500.000), 3);
	let c22 = add!(circuit, Switch, (-650.000, 800.000));
	let c23 = add!(circuit, Switch, (550.000, -150.000));
	let c24 = add!(circuit, Switch, (550.000, 150.000));
	let c25 = add!(circuit, Switch, (1550.000, -150.000));
	let c26 = add!(circuit, Switch, (-450.000, -150.000));
	let c27 = add!(circuit, Switch, (-450.000, 150.000));
	let c28 = add!(circuit, Switch, (1550.000, 150.000));
	let c29 = add!(circuit, Switch, (6050.000, -1900.000));
	let c30 = add!(circuit, Switch, (3300.000, -150.000));
	let c31 = add!(circuit, Switch, (3300.000, 150.000));
	let c32 = add!(circuit, Switch, (4300.000, -150.000));
	let c33 = add!(circuit, Switch, (4300.000, 150.000));
	
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
	circuit.connect((c1, 17), (c10, 0), &[WireLayoutCommand::MoveTo((925.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((880.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 18), (c10, 3), &[WireLayoutCommand::MoveTo((935.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((880.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 19), (c10, 6), &[WireLayoutCommand::MoveTo((945.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((880.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 20), (c10, 9), &[WireLayoutCommand::MoveTo((955.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((880.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 21), (c10, 12), &[WireLayoutCommand::MoveTo((965.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((880.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 22), (c10, 15), &[WireLayoutCommand::MoveTo((975.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((880.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 23), (c10, 18), &[WireLayoutCommand::MoveTo((985.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((880.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 24), (c10, 21), &[WireLayoutCommand::MoveTo((995.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((880.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 25), (c10, 24), &[WireLayoutCommand::MoveTo((1005.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((1120.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 26), (c10, 27), &[WireLayoutCommand::MoveTo((1015.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((1120.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 27), (c10, 30), &[WireLayoutCommand::MoveTo((1025.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((1120.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 28), (c10, 33), &[WireLayoutCommand::MoveTo((1035.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((1120.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 29), (c10, 36), &[WireLayoutCommand::MoveTo((1045.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((1120.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 30), (c10, 39), &[WireLayoutCommand::MoveTo((1055.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((1120.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 31), (c10, 42), &[WireLayoutCommand::MoveTo((1065.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((1120.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 32), (c10, 45), &[WireLayoutCommand::MoveTo((1075.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((1120.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 1), (c9, 31), &[WireLayoutCommand::MoveTo((4675.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 2), (c9, 30), &[WireLayoutCommand::MoveTo((4685.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 3), (c9, 29), &[WireLayoutCommand::MoveTo((4695.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 4), (c9, 28), &[WireLayoutCommand::MoveTo((4705.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 5), (c9, 27), &[WireLayoutCommand::MoveTo((4715.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 6), (c9, 26), &[WireLayoutCommand::MoveTo((4725.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 7), (c9, 25), &[WireLayoutCommand::MoveTo((4735.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 8), (c9, 24), &[WireLayoutCommand::MoveTo((4745.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 9), (c9, 23), &[WireLayoutCommand::MoveTo((4755.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 10), (c9, 22), &[WireLayoutCommand::MoveTo((4765.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 11), (c9, 21), &[WireLayoutCommand::MoveTo((4775.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 12), (c9, 20), &[WireLayoutCommand::MoveTo((4785.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 13), (c9, 19), &[WireLayoutCommand::MoveTo((4795.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 14), (c9, 18), &[WireLayoutCommand::MoveTo((4805.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 15), (c9, 17), &[WireLayoutCommand::MoveTo((4815.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 16), (c9, 16), &[WireLayoutCommand::MoveTo((4825.000, -992.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 1), (c9, 15), &[WireLayoutCommand::MoveTo((3675.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 2), (c9, 14), &[WireLayoutCommand::MoveTo((3685.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 3), (c9, 13), &[WireLayoutCommand::MoveTo((3695.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 4), (c9, 12), &[WireLayoutCommand::MoveTo((3705.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 5), (c9, 11), &[WireLayoutCommand::MoveTo((3715.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 6), (c9, 10), &[WireLayoutCommand::MoveTo((3725.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 7), (c9, 9), &[WireLayoutCommand::MoveTo((3735.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 8), (c9, 8), &[WireLayoutCommand::MoveTo((3745.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 9), (c9, 7), &[WireLayoutCommand::MoveTo((3755.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 10), (c9, 6), &[WireLayoutCommand::MoveTo((3765.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 11), (c9, 5), &[WireLayoutCommand::MoveTo((3775.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 12), (c9, 4), &[WireLayoutCommand::MoveTo((3785.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 13), (c9, 3), &[WireLayoutCommand::MoveTo((3795.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 14), (c9, 2), &[WireLayoutCommand::MoveTo((3805.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 15), (c9, 1), &[WireLayoutCommand::MoveTo((3815.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 16), (c9, 0), &[WireLayoutCommand::MoveTo((3825.000, -1592.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 47), (c11, 0), &[]);
	circuit.connect((c9, 46), (c11, 1), &[]);
	circuit.connect((c9, 45), (c11, 2), &[]);
	circuit.connect((c9, 44), (c11, 3), &[]);
	circuit.connect((c9, 43), (c11, 4), &[]);
	circuit.connect((c9, 42), (c11, 5), &[]);
	circuit.connect((c9, 41), (c11, 6), &[]);
	circuit.connect((c9, 40), (c11, 7), &[]);
	circuit.connect((c9, 39), (c11, 8), &[]);
	circuit.connect((c9, 38), (c11, 9), &[]);
	circuit.connect((c9, 37), (c11, 10), &[]);
	circuit.connect((c9, 36), (c11, 11), &[]);
	circuit.connect((c9, 35), (c11, 12), &[]);
	circuit.connect((c9, 34), (c11, 13), &[]);
	circuit.connect((c9, 33), (c11, 14), &[]);
	circuit.connect((c9, 32), (c11, 15), &[]);
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
	circuit.connect((c5, 17), (c8, 0), &[WireLayoutCommand::MoveTo((4675.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((4630.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 18), (c8, 3), &[WireLayoutCommand::MoveTo((4685.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((4630.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 19), (c8, 6), &[WireLayoutCommand::MoveTo((4695.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((4630.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 20), (c8, 9), &[WireLayoutCommand::MoveTo((4705.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((4630.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 21), (c8, 12), &[WireLayoutCommand::MoveTo((4715.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((4630.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 22), (c8, 15), &[WireLayoutCommand::MoveTo((4725.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((4630.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 23), (c8, 18), &[WireLayoutCommand::MoveTo((4735.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((4630.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 24), (c8, 21), &[WireLayoutCommand::MoveTo((4745.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((4630.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 25), (c8, 24), &[WireLayoutCommand::MoveTo((4755.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((4870.000, 585.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 26), (c8, 27), &[WireLayoutCommand::MoveTo((4765.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((4870.000, 575.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 27), (c8, 30), &[WireLayoutCommand::MoveTo((4775.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((4870.000, 565.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 28), (c8, 33), &[WireLayoutCommand::MoveTo((4785.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((4870.000, 555.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 29), (c8, 36), &[WireLayoutCommand::MoveTo((4795.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((4870.000, 545.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 30), (c8, 39), &[WireLayoutCommand::MoveTo((4805.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((4870.000, 535.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 31), (c8, 42), &[WireLayoutCommand::MoveTo((4815.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((4870.000, 525.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 32), (c8, 45), &[WireLayoutCommand::MoveTo((4825.000, 550.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((4870.000, 515.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 1), (c13, 0), &[WireLayoutCommand::MoveTo((-75.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((-200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 2), (c13, 1), &[WireLayoutCommand::MoveTo((-65.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((-200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 3), (c13, 2), &[WireLayoutCommand::MoveTo((-55.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 4), (c13, 3), &[WireLayoutCommand::MoveTo((-45.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((-200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 5), (c13, 4), &[WireLayoutCommand::MoveTo((-35.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((-200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 6), (c13, 5), &[WireLayoutCommand::MoveTo((-25.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 7), (c13, 6), &[WireLayoutCommand::MoveTo((-15.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((-200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 8), (c13, 7), &[WireLayoutCommand::MoveTo((-5.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((-200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 9), (c13, 8), &[WireLayoutCommand::MoveTo((5.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 10), (c13, 9), &[WireLayoutCommand::MoveTo((15.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 11), (c13, 10), &[WireLayoutCommand::MoveTo((25.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 12), (c13, 11), &[WireLayoutCommand::MoveTo((35.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 13), (c13, 12), &[WireLayoutCommand::MoveTo((45.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 14), (c13, 13), &[WireLayoutCommand::MoveTo((55.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 15), (c13, 14), &[WireLayoutCommand::MoveTo((65.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 16), (c13, 15), &[WireLayoutCommand::MoveTo((75.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 1), (c14, 0), &[WireLayoutCommand::MoveTo((925.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((800.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 2), (c14, 1), &[WireLayoutCommand::MoveTo((935.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((800.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 3), (c14, 2), &[WireLayoutCommand::MoveTo((945.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((800.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 4), (c14, 3), &[WireLayoutCommand::MoveTo((955.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((800.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 5), (c14, 4), &[WireLayoutCommand::MoveTo((965.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((800.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 6), (c14, 5), &[WireLayoutCommand::MoveTo((975.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((800.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 7), (c14, 6), &[WireLayoutCommand::MoveTo((985.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((800.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 8), (c14, 7), &[WireLayoutCommand::MoveTo((995.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((800.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 9), (c14, 8), &[WireLayoutCommand::MoveTo((1005.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((1200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 10), (c14, 9), &[WireLayoutCommand::MoveTo((1015.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((1200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 11), (c14, 10), &[WireLayoutCommand::MoveTo((1025.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((1200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 12), (c14, 11), &[WireLayoutCommand::MoveTo((1035.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((1200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 13), (c14, 12), &[WireLayoutCommand::MoveTo((1045.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((1200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 14), (c14, 13), &[WireLayoutCommand::MoveTo((1055.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((1200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 15), (c14, 14), &[WireLayoutCommand::MoveTo((1065.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((1200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 16), (c14, 15), &[WireLayoutCommand::MoveTo((1075.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((1200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 1), (c15, 0), &[WireLayoutCommand::MoveTo((1925.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((1800.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 2), (c15, 1), &[WireLayoutCommand::MoveTo((1935.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((1800.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 3), (c15, 2), &[WireLayoutCommand::MoveTo((1945.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((1800.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 4), (c15, 3), &[WireLayoutCommand::MoveTo((1955.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((1800.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 5), (c15, 4), &[WireLayoutCommand::MoveTo((1965.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((1800.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 6), (c15, 5), &[WireLayoutCommand::MoveTo((1975.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((1800.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 7), (c15, 6), &[WireLayoutCommand::MoveTo((1985.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((1800.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 8), (c15, 7), &[WireLayoutCommand::MoveTo((1995.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((1800.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 9), (c15, 8), &[WireLayoutCommand::MoveTo((2005.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -35.000)), WireLayoutCommand::MoveTo((2200.000, -610.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 10), (c15, 9), &[WireLayoutCommand::MoveTo((2015.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -25.000)), WireLayoutCommand::MoveTo((2200.000, -600.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 11), (c15, 10), &[WireLayoutCommand::MoveTo((2025.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((2200.000, -590.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 12), (c15, 11), &[WireLayoutCommand::MoveTo((2035.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -5.000)), WireLayoutCommand::MoveTo((2200.000, -580.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 13), (c15, 12), &[WireLayoutCommand::MoveTo((2045.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 5.000)), WireLayoutCommand::MoveTo((2200.000, -570.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 14), (c15, 13), &[WireLayoutCommand::MoveTo((2055.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((2200.000, -560.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 15), (c15, 14), &[WireLayoutCommand::MoveTo((2065.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 25.000)), WireLayoutCommand::MoveTo((2200.000, -550.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c3, 16), (c15, 15), &[WireLayoutCommand::MoveTo((2075.000, -575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 35.000)), WireLayoutCommand::MoveTo((2200.000, -540.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c16, 0), (c12, 0), &[WireLayoutCommand::MoveTo((-1625.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 1), (c12, 1), &[WireLayoutCommand::MoveTo((-1575.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 2), (c12, 2), &[WireLayoutCommand::MoveTo((-1525.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 3), (c12, 3), &[WireLayoutCommand::MoveTo((-1475.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 4), (c12, 4), &[WireLayoutCommand::MoveTo((-1425.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 5), (c12, 5), &[WireLayoutCommand::MoveTo((-1375.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 6), (c12, 6), &[WireLayoutCommand::MoveTo((-1325.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 7), (c12, 7), &[WireLayoutCommand::MoveTo((-1275.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 8), (c12, 8), &[WireLayoutCommand::MoveTo((-1225.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 9), (c12, 9), &[WireLayoutCommand::MoveTo((-1175.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 10), (c12, 10), &[WireLayoutCommand::MoveTo((-1125.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 11), (c12, 11), &[WireLayoutCommand::MoveTo((-1075.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 12), (c12, 12), &[WireLayoutCommand::MoveTo((-1025.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 13), (c12, 13), &[WireLayoutCommand::MoveTo((-975.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 14), (c12, 14), &[WireLayoutCommand::MoveTo((-925.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c16, 15), (c12, 15), &[WireLayoutCommand::MoveTo((-875.000, 1000.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c12, 17), (c2, 1), &[WireLayoutCommand::MoveTo((0.000, 1287.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 18), (c2, 4), &[WireLayoutCommand::MoveTo((0.000, 1302.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 19), (c2, 7), &[WireLayoutCommand::MoveTo((0.000, 1317.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 20), (c2, 10), &[WireLayoutCommand::MoveTo((0.000, 1332.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 21), (c2, 13), &[WireLayoutCommand::MoveTo((0.000, 1347.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 22), (c2, 16), &[WireLayoutCommand::MoveTo((0.000, 1362.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 23), (c2, 19), &[WireLayoutCommand::MoveTo((0.000, 1377.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 24), (c2, 22), &[WireLayoutCommand::MoveTo((0.000, 1392.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 25), (c2, 25), &[WireLayoutCommand::MoveTo((0.000, 1407.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 26), (c2, 28), &[WireLayoutCommand::MoveTo((0.000, 1422.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 27), (c2, 31), &[WireLayoutCommand::MoveTo((0.000, 1437.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 28), (c2, 34), &[WireLayoutCommand::MoveTo((0.000, 1452.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 29), (c2, 37), &[WireLayoutCommand::MoveTo((0.000, 1467.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 30), (c2, 40), &[WireLayoutCommand::MoveTo((0.000, 1482.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 31), (c2, 43), &[WireLayoutCommand::MoveTo((0.000, 1497.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c12, 32), (c2, 46), &[WireLayoutCommand::MoveTo((0.000, 1512.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 2), (c10, 1), &[]);
	circuit.connect((c2, 5), (c10, 4), &[]);
	circuit.connect((c2, 8), (c10, 7), &[]);
	circuit.connect((c2, 11), (c10, 10), &[]);
	circuit.connect((c2, 14), (c10, 13), &[]);
	circuit.connect((c2, 17), (c10, 16), &[]);
	circuit.connect((c2, 20), (c10, 19), &[]);
	circuit.connect((c2, 23), (c10, 22), &[]);
	circuit.connect((c2, 26), (c10, 25), &[]);
	circuit.connect((c2, 29), (c10, 28), &[]);
	circuit.connect((c2, 32), (c10, 31), &[]);
	circuit.connect((c2, 35), (c10, 34), &[]);
	circuit.connect((c2, 38), (c10, 37), &[]);
	circuit.connect((c2, 41), (c10, 40), &[]);
	circuit.connect((c2, 44), (c10, 43), &[]);
	circuit.connect((c2, 47), (c10, 46), &[]);
	circuit.connect((c10, 2), (c6, 1), &[]);
	circuit.connect((c10, 5), (c6, 4), &[]);
	circuit.connect((c10, 8), (c6, 7), &[]);
	circuit.connect((c10, 11), (c6, 10), &[]);
	circuit.connect((c10, 14), (c6, 13), &[]);
	circuit.connect((c10, 17), (c6, 16), &[]);
	circuit.connect((c10, 20), (c6, 19), &[]);
	circuit.connect((c10, 23), (c6, 22), &[]);
	circuit.connect((c10, 26), (c6, 25), &[]);
	circuit.connect((c10, 29), (c6, 28), &[]);
	circuit.connect((c10, 32), (c6, 31), &[]);
	circuit.connect((c10, 35), (c6, 34), &[]);
	circuit.connect((c10, 38), (c6, 37), &[]);
	circuit.connect((c10, 41), (c6, 40), &[]);
	circuit.connect((c10, 44), (c6, 43), &[]);
	circuit.connect((c10, 47), (c6, 46), &[]);
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
	circuit.connect((c11, 17), (c8, 2), &[WireLayoutCommand::MoveTo((6400.000, -1412.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, 0.000)), WireLayoutCommand::MoveTo((6512.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 18), (c8, 5), &[WireLayoutCommand::MoveTo((6400.000, -1397.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, 0.000)), WireLayoutCommand::MoveTo((6497.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 19), (c8, 8), &[WireLayoutCommand::MoveTo((6400.000, -1382.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, 0.000)), WireLayoutCommand::MoveTo((6482.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 20), (c8, 11), &[WireLayoutCommand::MoveTo((6400.000, -1367.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, 0.000)), WireLayoutCommand::MoveTo((6467.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 21), (c8, 14), &[WireLayoutCommand::MoveTo((6400.000, -1352.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((6452.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 22), (c8, 17), &[WireLayoutCommand::MoveTo((6400.000, -1337.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((6437.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 23), (c8, 20), &[WireLayoutCommand::MoveTo((6400.000, -1322.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((6422.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 24), (c8, 23), &[WireLayoutCommand::MoveTo((6400.000, -1307.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((6407.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 25), (c8, 26), &[WireLayoutCommand::MoveTo((6400.000, -1292.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((6392.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 26), (c8, 29), &[WireLayoutCommand::MoveTo((6400.000, -1277.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((6377.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 27), (c8, 32), &[WireLayoutCommand::MoveTo((6400.000, -1262.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((6362.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 28), (c8, 35), &[WireLayoutCommand::MoveTo((6400.000, -1247.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((6347.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 29), (c8, 38), &[WireLayoutCommand::MoveTo((6400.000, -1232.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, -0.000)), WireLayoutCommand::MoveTo((6332.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 30), (c8, 41), &[WireLayoutCommand::MoveTo((6400.000, -1217.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, -0.000)), WireLayoutCommand::MoveTo((6317.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 31), (c8, 44), &[WireLayoutCommand::MoveTo((6400.000, -1202.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, -0.000)), WireLayoutCommand::MoveTo((6302.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c11, 32), (c8, 47), &[WireLayoutCommand::MoveTo((6400.000, -1187.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, -0.000)), WireLayoutCommand::MoveTo((6287.500, 850.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c20, 0), (c21, 0), &[]);
	circuit.connect((c21, 1), (c0, 33), &[WireLayoutCommand::MoveTo((-600.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c21, 2), (c17, 0), &[]);
	circuit.connect((c17, 1), (c1, 33), &[WireLayoutCommand::MoveTo((500.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c17, 2), (c18, 0), &[]);
	circuit.connect((c18, 1), (c3, 33), &[WireLayoutCommand::MoveTo((1500.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c18, 2), (c19, 0), &[]);
	circuit.connect((c19, 1), (c4, 33), &[WireLayoutCommand::MoveTo((3250.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c19, 2), (c5, 33), &[WireLayoutCommand::MoveTo((3450.000, -1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((3450.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4250.000, -1400.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((4250.000, 50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c22, 0), (c12, 16), &[]);
	circuit.connect((c23, 0), (c1, 0), &[WireLayoutCommand::MoveTo((550.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c24, 0), (c1, 34), &[WireLayoutCommand::MoveTo((550.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c26, 0), (c0, 0), &[WireLayoutCommand::MoveTo((-450.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c27, 0), (c0, 34), &[WireLayoutCommand::MoveTo((-450.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c25, 0), (c3, 0), &[WireLayoutCommand::MoveTo((1550.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c28, 0), (c3, 34), &[WireLayoutCommand::MoveTo((1550.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c29, 0), (c11, 16), &[]);
	circuit.connect((c30, 0), (c4, 0), &[WireLayoutCommand::MoveTo((3300.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c31, 0), (c4, 34), &[WireLayoutCommand::MoveTo((3300.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c32, 0), (c5, 0), &[WireLayoutCommand::MoveTo((4300.000, -50.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c33, 0), (c5, 34), &[WireLayoutCommand::MoveTo((4300.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	
	circuit
}

pub fn editor_example() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Rom, (0.000, 0.000), 3);
	let c1 = add!(circuit, MultiBulb, (700.000, -200.000), 16);
	let c2 = add!(circuit, MultiSwitch, (-700.000, -200.000), 16);
	
	circuit.connect((c0, 3), (c1, 0), &[WireLayoutCommand::MoveTo((700.000, -75.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 4), (c1, 1), &[WireLayoutCommand::MoveTo((700.000, -65.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 5), (c1, 2), &[WireLayoutCommand::MoveTo((700.000, -55.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 6), (c1, 3), &[WireLayoutCommand::MoveTo((700.000, -45.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 7), (c1, 4), &[WireLayoutCommand::MoveTo((700.000, -35.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 8), (c1, 5), &[WireLayoutCommand::MoveTo((700.000, -25.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 9), (c1, 6), &[WireLayoutCommand::MoveTo((700.000, -15.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 10), (c1, 7), &[WireLayoutCommand::MoveTo((700.000, -5.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 11), (c1, 8), &[WireLayoutCommand::MoveTo((700.000, 5.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 12), (c1, 9), &[WireLayoutCommand::MoveTo((700.000, 15.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 13), (c1, 10), &[WireLayoutCommand::MoveTo((700.000, 25.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 14), (c1, 11), &[WireLayoutCommand::MoveTo((700.000, 35.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 15), (c1, 12), &[WireLayoutCommand::MoveTo((700.000, 45.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 16), (c1, 13), &[WireLayoutCommand::MoveTo((700.000, 55.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 17), (c1, 14), &[WireLayoutCommand::MoveTo((700.000, 65.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 18), (c1, 15), &[WireLayoutCommand::MoveTo((700.000, 75.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 13), (c0, 0), &[WireLayoutCommand::MoveTo((-425.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 14), (c0, 1), &[WireLayoutCommand::MoveTo((-375.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 15), (c0, 2), &[WireLayoutCommand::MoveTo((-325.000, 0.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	
	// Puts some memory in the ROM
	circuit.set_memory(c0, &[1, 1, 2, 3, 5, 8, 13, 21]);

	circuit
}

#[wasm_bindgen(start)]
pub fn start() {
	log!("ATLAS has started!");
	set_panic_hook();

	let mut vm = AtlasVM::new();

	crate::log!("{:?}", generate_control_rom_data());

	vm.run(include_str!("./aasm/helloworld.aasm").to_string());

	crate::log!("{:?}", vm.registers);
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
