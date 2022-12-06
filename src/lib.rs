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
pub mod multiplexer;
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

pub fn editor_example() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, MultiTriStateBuffer, (650.000, 0.000), 16);
	let c1 = add!(circuit, MultiDFlipFlop, (-650.000, 0.000), 16);
	let c2 = add!(circuit, Switch, (-1750.000, 0.000));
	let c3 = add!(circuit, Switch, (-1750.000, 250.000));
	let c4 = add!(circuit, Switch, (-1750.000, -250.000));
	let c5 = add!(circuit, MultiSwitch, (0.000, 2000.000), 16);
	let c6 = add!(circuit, MultiJunction, (0.000, 0.000), 16);
	let c7 = add!(circuit, MultiBulb, (0.000, -2000.000), 16);
	let c8 = add!(circuit, MultiJunction, (0.000, 1300.000), 16, true);
	let c9 = add!(circuit, AndGate, (-1550.000, 150.000));
	
	circuit.connect((c4, 0), (c0, 16), &[WireLayoutCommand::MoveTo((-950.000, -250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-950.000, -900.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((650.000, -900.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 32), (c6, 0), &[WireLayoutCommand::MoveTo((-350.000, -128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-59.850, -0.000)), WireLayoutCommand::MoveTo((-409.850, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 31), (c6, 3), &[WireLayoutCommand::MoveTo((-350.000, -111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-42.750, -0.000)), WireLayoutCommand::MoveTo((-392.750, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 30), (c6, 6), &[WireLayoutCommand::MoveTo((-350.000, -94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.650, -0.000)), WireLayoutCommand::MoveTo((-375.650, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 29), (c6, 9), &[WireLayoutCommand::MoveTo((-350.000, -76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.550, -0.000)), WireLayoutCommand::MoveTo((-358.550, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 28), (c6, 12), &[WireLayoutCommand::MoveTo((-350.000, -59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.550, 0.000)), WireLayoutCommand::MoveTo((-341.450, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 27), (c6, 15), &[WireLayoutCommand::MoveTo((-350.000, -42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.650, 0.000)), WireLayoutCommand::MoveTo((-324.350, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 26), (c6, 18), &[WireLayoutCommand::MoveTo((-350.000, -25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((42.750, 0.000)), WireLayoutCommand::MoveTo((-307.250, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 25), (c6, 21), &[WireLayoutCommand::MoveTo((-350.000, -8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.850, 0.000)), WireLayoutCommand::MoveTo((-290.150, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 24), (c6, 24), &[WireLayoutCommand::MoveTo((-350.000, 8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.850, 0.000)), WireLayoutCommand::MoveTo((-290.150, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 23), (c6, 27), &[WireLayoutCommand::MoveTo((-350.000, 25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((42.750, 0.000)), WireLayoutCommand::MoveTo((-307.250, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 22), (c6, 30), &[WireLayoutCommand::MoveTo((-350.000, 42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.650, 0.000)), WireLayoutCommand::MoveTo((-324.350, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 21), (c6, 33), &[WireLayoutCommand::MoveTo((-350.000, 59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.550, 0.000)), WireLayoutCommand::MoveTo((-341.450, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 20), (c6, 36), &[WireLayoutCommand::MoveTo((-350.000, 76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.550, -0.000)), WireLayoutCommand::MoveTo((-358.550, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 19), (c6, 39), &[WireLayoutCommand::MoveTo((-350.000, 94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.650, -0.000)), WireLayoutCommand::MoveTo((-375.650, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 18), (c6, 42), &[WireLayoutCommand::MoveTo((-350.000, 111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-42.750, -0.000)), WireLayoutCommand::MoveTo((-392.750, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 17), (c6, 45), &[WireLayoutCommand::MoveTo((-350.000, 128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-59.850, -0.000)), WireLayoutCommand::MoveTo((-409.850, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 0), (c6, 1), &[WireLayoutCommand::MoveTo((400.000, -112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((452.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 1), (c6, 4), &[WireLayoutCommand::MoveTo((400.000, -97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((437.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 2), (c6, 7), &[WireLayoutCommand::MoveTo((400.000, -82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((422.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 3), (c6, 10), &[WireLayoutCommand::MoveTo((400.000, -67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((407.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 4), (c6, 13), &[WireLayoutCommand::MoveTo((400.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((392.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 5), (c6, 16), &[WireLayoutCommand::MoveTo((400.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((377.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 6), (c6, 19), &[WireLayoutCommand::MoveTo((400.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((362.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 7), (c6, 22), &[WireLayoutCommand::MoveTo((400.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((347.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 8), (c6, 25), &[WireLayoutCommand::MoveTo((400.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((347.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 9), (c6, 28), &[WireLayoutCommand::MoveTo((400.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((362.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 10), (c6, 31), &[WireLayoutCommand::MoveTo((400.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((377.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 11), (c6, 34), &[WireLayoutCommand::MoveTo((400.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((392.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 12), (c6, 37), &[WireLayoutCommand::MoveTo((400.000, 67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((407.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 13), (c6, 40), &[WireLayoutCommand::MoveTo((400.000, 82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((422.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 14), (c6, 43), &[WireLayoutCommand::MoveTo((400.000, 97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((437.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 15), (c6, 46), &[WireLayoutCommand::MoveTo((400.000, 112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((452.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 2), (c7, 0), &[WireLayoutCommand::MoveTo((-225.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 105.000)), WireLayoutCommand::MoveTo((-200.000, -1045.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 5), (c7, 1), &[WireLayoutCommand::MoveTo((-195.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((-200.000, -1075.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 8), (c7, 2), &[WireLayoutCommand::MoveTo((-165.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((-200.000, -1105.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 11), (c7, 3), &[WireLayoutCommand::MoveTo((-135.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-200.000, -1135.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 14), (c7, 4), &[WireLayoutCommand::MoveTo((-105.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-200.000, -1165.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 17), (c7, 5), &[WireLayoutCommand::MoveTo((-75.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((-200.000, -1195.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 20), (c7, 6), &[WireLayoutCommand::MoveTo((-45.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((-200.000, -1225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 23), (c7, 7), &[WireLayoutCommand::MoveTo((-15.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -105.000)), WireLayoutCommand::MoveTo((-200.000, -1255.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 26), (c7, 8), &[WireLayoutCommand::MoveTo((15.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -105.000)), WireLayoutCommand::MoveTo((200.000, -1255.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 29), (c7, 9), &[WireLayoutCommand::MoveTo((45.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((200.000, -1225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 32), (c7, 10), &[WireLayoutCommand::MoveTo((75.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((200.000, -1195.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 35), (c7, 11), &[WireLayoutCommand::MoveTo((105.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((200.000, -1165.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 38), (c7, 12), &[WireLayoutCommand::MoveTo((135.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((200.000, -1135.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 41), (c7, 13), &[WireLayoutCommand::MoveTo((165.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((200.000, -1105.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 44), (c7, 14), &[WireLayoutCommand::MoveTo((195.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((200.000, -1075.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 47), (c7, 15), &[WireLayoutCommand::MoveTo((225.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 105.000)), WireLayoutCommand::MoveTo((200.000, -1045.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 15), (c8, 0), &[WireLayoutCommand::MoveTo((-1100.000, -128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-128.250, 0.000)), WireLayoutCommand::MoveTo((-1228.250, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 14), (c8, 3), &[WireLayoutCommand::MoveTo((-1100.000, -111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-111.150, 0.000)), WireLayoutCommand::MoveTo((-1211.150, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 13), (c8, 6), &[WireLayoutCommand::MoveTo((-1100.000, -94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-94.050, 0.000)), WireLayoutCommand::MoveTo((-1194.050, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 12), (c8, 9), &[WireLayoutCommand::MoveTo((-1100.000, -76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-76.950, 0.000)), WireLayoutCommand::MoveTo((-1176.950, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 11), (c8, 12), &[WireLayoutCommand::MoveTo((-1100.000, -59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-59.850, 0.000)), WireLayoutCommand::MoveTo((-1159.850, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 10), (c8, 15), &[WireLayoutCommand::MoveTo((-1100.000, -42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-42.750, 0.000)), WireLayoutCommand::MoveTo((-1142.750, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 9), (c8, 18), &[WireLayoutCommand::MoveTo((-1100.000, -25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.650, 0.000)), WireLayoutCommand::MoveTo((-1125.650, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 8), (c8, 21), &[WireLayoutCommand::MoveTo((-1100.000, -8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.550, 0.000)), WireLayoutCommand::MoveTo((-1108.550, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 7), (c8, 24), &[WireLayoutCommand::MoveTo((-1100.000, 8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.550, -0.000)), WireLayoutCommand::MoveTo((-1091.450, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 6), (c8, 27), &[WireLayoutCommand::MoveTo((-1100.000, 25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.650, -0.000)), WireLayoutCommand::MoveTo((-1074.350, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 5), (c8, 30), &[WireLayoutCommand::MoveTo((-1100.000, 42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((42.750, -0.000)), WireLayoutCommand::MoveTo((-1057.250, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 4), (c8, 33), &[WireLayoutCommand::MoveTo((-1100.000, 59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.850, -0.000)), WireLayoutCommand::MoveTo((-1040.150, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 3), (c8, 36), &[WireLayoutCommand::MoveTo((-1100.000, 76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((76.950, -0.000)), WireLayoutCommand::MoveTo((-1023.050, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 2), (c8, 39), &[WireLayoutCommand::MoveTo((-1100.000, 94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((94.050, -0.000)), WireLayoutCommand::MoveTo((-1005.950, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 1), (c8, 42), &[WireLayoutCommand::MoveTo((-1100.000, 111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((111.150, -0.000)), WireLayoutCommand::MoveTo((-988.850, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 0), (c8, 45), &[WireLayoutCommand::MoveTo((-1100.000, 128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((128.250, -0.000)), WireLayoutCommand::MoveTo((-971.750, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 17), (c8, 1), &[WireLayoutCommand::MoveTo((1050.000, -112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, 0.000)), WireLayoutCommand::MoveTo((1162.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 18), (c8, 4), &[WireLayoutCommand::MoveTo((1050.000, -97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, 0.000)), WireLayoutCommand::MoveTo((1147.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 19), (c8, 7), &[WireLayoutCommand::MoveTo((1050.000, -82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, 0.000)), WireLayoutCommand::MoveTo((1132.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 20), (c8, 10), &[WireLayoutCommand::MoveTo((1050.000, -67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, 0.000)), WireLayoutCommand::MoveTo((1117.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 21), (c8, 13), &[WireLayoutCommand::MoveTo((1050.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((1102.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 22), (c8, 16), &[WireLayoutCommand::MoveTo((1050.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((1087.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 23), (c8, 19), &[WireLayoutCommand::MoveTo((1050.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((1072.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 24), (c8, 22), &[WireLayoutCommand::MoveTo((1050.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((1057.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 25), (c8, 25), &[WireLayoutCommand::MoveTo((1050.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((1042.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 26), (c8, 28), &[WireLayoutCommand::MoveTo((1050.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((1027.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 27), (c8, 31), &[WireLayoutCommand::MoveTo((1050.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((1012.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 28), (c8, 34), &[WireLayoutCommand::MoveTo((1050.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((997.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 29), (c8, 37), &[WireLayoutCommand::MoveTo((1050.000, 67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, -0.000)), WireLayoutCommand::MoveTo((982.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 30), (c8, 40), &[WireLayoutCommand::MoveTo((1050.000, 82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, -0.000)), WireLayoutCommand::MoveTo((967.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 31), (c8, 43), &[WireLayoutCommand::MoveTo((1050.000, 97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, -0.000)), WireLayoutCommand::MoveTo((952.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 32), (c8, 46), &[WireLayoutCommand::MoveTo((1050.000, 112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, -0.000)), WireLayoutCommand::MoveTo((937.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c8, 2), (c5, 0), &[WireLayoutCommand::MoveTo((-225.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -105.000)), WireLayoutCommand::MoveTo((-200.000, 1595.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 5), (c5, 1), &[WireLayoutCommand::MoveTo((-195.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -75.000)), WireLayoutCommand::MoveTo((-200.000, 1625.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 8), (c5, 2), &[WireLayoutCommand::MoveTo((-165.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -45.000)), WireLayoutCommand::MoveTo((-200.000, 1655.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 11), (c5, 3), &[WireLayoutCommand::MoveTo((-135.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((-200.000, 1685.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 14), (c5, 4), &[WireLayoutCommand::MoveTo((-105.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((-200.000, 1715.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 17), (c5, 5), &[WireLayoutCommand::MoveTo((-75.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 45.000)), WireLayoutCommand::MoveTo((-200.000, 1745.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 20), (c5, 6), &[WireLayoutCommand::MoveTo((-45.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 75.000)), WireLayoutCommand::MoveTo((-200.000, 1775.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 23), (c5, 7), &[WireLayoutCommand::MoveTo((-15.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 105.000)), WireLayoutCommand::MoveTo((-200.000, 1805.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 26), (c5, 8), &[WireLayoutCommand::MoveTo((15.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 105.000)), WireLayoutCommand::MoveTo((200.000, 1805.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 29), (c5, 9), &[WireLayoutCommand::MoveTo((45.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 75.000)), WireLayoutCommand::MoveTo((200.000, 1775.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 32), (c5, 10), &[WireLayoutCommand::MoveTo((75.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 45.000)), WireLayoutCommand::MoveTo((200.000, 1745.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 35), (c5, 11), &[WireLayoutCommand::MoveTo((105.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((200.000, 1715.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 38), (c5, 12), &[WireLayoutCommand::MoveTo((135.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((200.000, 1685.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 41), (c5, 13), &[WireLayoutCommand::MoveTo((165.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -45.000)), WireLayoutCommand::MoveTo((200.000, 1655.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 44), (c5, 14), &[WireLayoutCommand::MoveTo((195.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -75.000)), WireLayoutCommand::MoveTo((200.000, 1625.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c8, 47), (c5, 15), &[WireLayoutCommand::MoveTo((225.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -105.000)), WireLayoutCommand::MoveTo((200.000, 1595.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 0), (c9, 0), &[WireLayoutCommand::MoveTo((-1677.750, 0.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1677.750, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 0), (c9, 1), &[WireLayoutCommand::MoveTo((-1677.750, 250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1677.750, 180.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c9, 2), (c1, 16), &[WireLayoutCommand::MoveTo((-1400.000, 150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1400.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-650.000, 950.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	
	circuit
}

#[wasm_bindgen(start)]
pub fn start() {
	log!("ATLAS has started!");
	set_panic_hook();

	let mut vm = LowLevelAtlasVM::new();

	crate::log!("{:?}", generate_control_rom_data());

	vm.run(include_str!("./aasm/test.aasm").to_string());

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
