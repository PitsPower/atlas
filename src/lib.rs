#![feature(test)]

extern crate test;

pub mod adder;
pub mod bus;
pub mod core;
pub mod editor;
pub mod gates;
pub mod graphics;
pub mod latches;
pub mod register;
pub mod transistor;
pub mod utils;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use crate::adder::Adder;
use crate::core::{Bulb, Circuit, Junction, MultiBulb, MultiSwitch, PinState, Switch};
use crate::gates::{AndGate, NandGate, NorGate, NotGate, OrGate, TriStateBuffer, XorGate};
use crate::graphics::WireLayoutCommand;
use crate::latches::MultiDFlipFlop;
use crate::transistor::{NTransistor, PTransistor};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		web_sys::console::log_1(&format!($($arg)*).into());
	};
}

#[macro_export]
macro_rules! add {
	($circ:expr, $type:ident, $($arg:tt)*) => {
		$circ.add(Box::new($type::new($($arg)*)))
	};
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

#[wasm_bindgen]
pub fn example2() -> Circuit {
	let mut circuit = Circuit::new();

	let switch = circuit.add(Box::new(Switch::new((-200.0, 0.0))));
	let junction = circuit.add(Box::new(Junction::new((0.0, 0.0), 3)));
	let bulb1 = circuit.add(Box::new(Bulb::new((200.0, 0.0))));
	let bulb2 = circuit.add(Box::new(Bulb::new((0.0, -200.0))));

	circuit.connect((switch, 0), (junction, 0), vec![]);
	circuit.connect((junction, 1), (bulb1, 0), vec![]);
	circuit.connect((junction, 2), (bulb2, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn transistor_example() -> Circuit {
	let mut circuit = Circuit::new();

	let transistor = circuit.add(Box::new(NTransistor::new((0.0, 0.0))));
	
	let gate = circuit.add(Box::new(Switch::new((-200.0, 0.0))));

	let offset = circuit.get_components()[transistor].get_pin_positions()[1].0;

	let source = circuit.add(Box::new(Switch::new((offset, 200.0))));
	let drain = circuit.add(Box::new(Bulb::new((offset, -200.0))));

	circuit.connect((gate, 0), (transistor, 0), vec![]);
	circuit.connect((source, 0), (transistor, 1), vec![]);
	circuit.connect((transistor, 2), (drain, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn transistor_example2() -> Circuit {
	let mut circuit = Circuit::new();

	let transistor = circuit.add(Box::new(PTransistor::new((0.0, 0.0))));
	
	let gate = circuit.add(Box::new(Switch::new((-200.0, 0.0))));

	let offset = circuit.get_components()[transistor].get_pin_positions()[1].0;

	let source = circuit.add(Box::new(Switch::new((offset, -200.0))));
	let drain = circuit.add(Box::new(Bulb::new((offset, 200.0))));

	circuit.connect((gate, 0), (transistor, 0), vec![]);
	circuit.connect((source, 0), (transistor, 1), vec![]);
	circuit.connect((transistor, 2), (drain, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn bidirectional_example() -> Circuit {
	let mut circuit = Circuit::new();

	let transistor1 = add!(circuit, NTransistor, (-400.0, 200.0));
	let transistor2 = add!(circuit, NTransistor, (400.0, 200.0));

	let switch1 = add!(circuit, Switch, (-600.0, 200.0));
	let switch2 = add!(circuit, Switch, (200.0, 200.0));

	let offset = circuit.get_components()[transistor1].get_pin_positions()[1].0;

	let source1 = add!(circuit, Switch, (-400.0 + offset, 400.0));
	let source2 = add!(circuit, Switch, (400.0 + offset, 400.0));

	circuit.toggle_switch(2);
	circuit.toggle_switch(3);

	let junction1 = add!(circuit, Junction, (-400.0 + offset, 0.0), 3);
	let junction2 = add!(circuit, Junction, (400.0 + offset, 0.0), 3);

	let bulb1 = add!(circuit, Bulb, (-400.0 + offset, -200.0));
	let bulb2 = add!(circuit, Bulb, (400.0 + offset, -200.0));

	circuit.connect((switch1, 0), (transistor1, 0), vec![]);
	circuit.connect((source1, 0), (transistor1, 1), vec![]);
	circuit.connect((transistor1, 2), (junction1, 0), vec![]);
	circuit.connect((junction1, 1), (bulb1, 0), vec![]);

	circuit.connect((switch2, 0), (transistor2, 0), vec![]);
	circuit.connect((source2, 0), (transistor2, 1), vec![]);
	circuit.connect((transistor2, 2), (junction2, 0), vec![]);
	circuit.connect((junction2, 1), (bulb2, 0), vec![]);
	
	circuit.connect((junction1, 2), (junction2, 2), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn not_gate_example() -> Circuit {
	let mut circuit = Circuit::new();
	
	let input = add!(circuit, Switch, (-300.0, 0.0));
	let not_gate_1 = add!(circuit, NotGate, (0.0, -150.0));
	let not_gate_2 = add!(circuit, NotGate, (0.0, 150.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input, 0), (not_gate_1, 0), vec![
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((not_gate_1, 1), (not_gate_2, 0), vec![
		WireLayoutCommand::MoveHorizontal(100.0),
		WireLayoutCommand::CenterVertical,
		WireLayoutCommand::MoveHorizontal(-300.0),
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((not_gate_2, 1), (output, 0), vec![
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}

#[wasm_bindgen]
pub fn nor_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, NorGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn or_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, OrGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn nand_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, NandGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn and_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, AndGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn xor_gate_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	let gate = add!(circuit, XorGate, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input1, 0), (gate, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (gate, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((gate, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn nor_latch_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Switch, (-300.0, -100.0));
	let input2 = add!(circuit, Switch, (-300.0, 100.0));
	
	let nor1 = add!(circuit, NorGate, (0.0, -100.0));
	let nor2 = add!(circuit, NorGate, (0.0, 100.0));

	let junction1 = add!(circuit, Junction, (150.0, -100.0), 3);
	let junction2 = add!(circuit, Junction, (150.0, 100.0), 3);

	let output1 = add!(circuit, Bulb, (300.0, -100.0));
	let output2 = add!(circuit, Bulb, (300.0, 100.0));

	circuit.connect((input1, 0), (nor1, 0), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (nor2, 1), vec![
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	
	circuit.connect((nor1, 2), (junction1, 0), vec![]);
	circuit.connect((junction1, 1), (output1, 0), vec![]);
	
	circuit.connect((nor2, 2), (junction2, 0), vec![]);
	circuit.connect((junction2, 1), (output2, 0), vec![]);

	circuit.connect((junction1, 2), (nor2, 0), vec![
		WireLayoutCommand::MoveVertical(35.0),
		WireLayoutCommand::Move((-250.0, 100.0)),
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((junction2, 2), (nor1, 1), vec![
		WireLayoutCommand::MoveVertical(-35.0),
		WireLayoutCommand::Move((-250.0, -100.0)),
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit
}

#[wasm_bindgen]
pub fn test_example() -> Circuit {
	let mut circuit = Circuit::new();

	let size = 8;
	
	let input1 = add!(circuit, MultiSwitch, (-600.0, -500.0), size);
	let input2 = add!(circuit, MultiSwitch, (-600.0, 0.0), size);

	let adder = add!(circuit, Adder, (0.0, 0.0), size);

	let output = add!(circuit, MultiBulb, (600.0, -200.0), size);

	for i in 0..size {
		circuit.connect((input1, i), (adder, size - i - 1), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input2, i), (adder, size + (size - i - 1)), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((adder, 2 * (size as usize) + (size - i - 1) as usize), (output, i), vec![
			WireLayoutCommand::AlignVertical,
		]);
	}

	circuit
}

#[wasm_bindgen]
pub fn bus_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input = add!(circuit, MultiSwitch, (-300.0, 0.0), 8);
	let output = add!(circuit, MultiBulb, (300.0, 0.0), 8);

	for i in 0..8 {
		circuit.connect((input, i), (output, i), vec![
			WireLayoutCommand::MoveVertical(i as f64 * 50.0 + 50.0),
			WireLayoutCommand::AlignVertical,
		]);
	}

	circuit
}

#[wasm_bindgen]
pub fn latch_example() -> Circuit {
	let mut circuit = Circuit::new();

	let size = 8;

	let clock = add!(circuit, Switch, (-600.0, 550.0));
	let increment = add!(circuit, MultiSwitch, (0.0, -700.0), size);

	circuit.toggle_switch(size);

	let dff = add!(circuit, MultiDFlipFlop, (-600.0, 0.0), size);
	let adder = add!(circuit, Adder, (600.0, 0.0), size);
	
	let juncs: Vec<_> = (0..size)
		.map(|i| add!(circuit, Junction, (1100.0 + i as f64 * 30.0, (-(size as f64) * 0.5 + i as f64 + 0.5) * 30.0), 3))
		.collect();

	let output = add!(circuit, MultiBulb, (1800.0, -500.0), 8);

	circuit.connect((clock, 0), (dff, size), vec![]);

	let fsize = size as f64;

	for i in 0..size {
		let fi = i as f64;

		circuit.connect((increment, size - i - 1), (adder, i), vec![
			WireLayoutCommand::MoveVertical((if i < size/2 { fsize * 0.5 - 1.0 - fi } else { fi - fsize * 0.5 }) * 30.0 + 30.0),
			WireLayoutCommand::MoveHorizontal((if i < size/2 { -(fsize * 0.5 - 1.0 - fi + 0.5) } else { fi - fsize * 0.5 + 0.5 }) * 30.0),
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((dff, size+1 + i), (adder, size + i), vec![
			WireLayoutCommand::MoveHorizontal(50.0),
			WireLayoutCommand::MoveHorizontal(200.0 - (fsize - fi - 1.0) * 15.0),
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((adder, 2*size + i), (juncs[size - i - 1], 0), vec![
			WireLayoutCommand::MoveHorizontal((if i < size/2 { -(fsize * 0.5 - 1.0 - fi) } else { -(fi - fsize * 0.5) }) * 30.0 + 120.0),
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((juncs[i], 2), (output, i), vec![
			WireLayoutCommand::AlignVertical,
		]);

		circuit.connect((juncs[i], 1), (dff, size - i - 1), vec![
			WireLayoutCommand::MoveVertical(900.0),
			WireLayoutCommand::MoveHorizontal(-2300.0),
			WireLayoutCommand::AlignHorizontal,
		]);
	}

	circuit
}

#[wasm_bindgen]
pub fn register_example() -> Circuit {
	let mut circuit = Circuit::new();

	let input = add!(circuit, Switch, (-300.0, 0.0));
	let enable = add!(circuit, Switch, (0.0, -300.0));
	let tsb = add!(circuit, TriStateBuffer, (0.0, 0.0));
	let output = add!(circuit, Bulb, (300.0, 0.0));

	circuit.connect((input, 0), (tsb, 0), vec![]);
	circuit.connect((enable, 0), (tsb, 1), vec![]);
	circuit.connect((tsb, 2), (output, 0), vec![]);

	circuit
}

#[wasm_bindgen]
pub fn editor_example() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Switch, (-598.952, -57.888));
	let c1 = add!(circuit, Switch, (-599.088, -11.126));
	let c2 = add!(circuit, Bulb, (157.160, -50.728));
	let c3 = add!(circuit, Bulb, (159.456, -15.286));
	let c4 = add!(circuit, Bulb, (161.592, 17.748));
	let c5 = add!(circuit, Switch, (-596.224, 32.772));
	
	circuit.get_components_mut()[c1].set_pin_state_external(0, PinState::On).unwrap();
	
	circuit.connect((c0, 0), (c2, 0), vec![WireLayoutCommand::MoveTo((-397.103, -48.174)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.652, -1.235)), WireLayoutCommand::MoveTo((-365.944, -130.081)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-11.325, 16.081)), WireLayoutCommand::MoveTo((-399.456, -233.562)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((1.787, 9.630)), WireLayoutCommand::MoveTo((-468.059, -327.524)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((6.794, 9.999)), WireLayoutCommand::MoveTo((-597.967, -386.540)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.545, 4.314)), WireLayoutCommand::MoveTo((-783.209, -394.508)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((13.473, 0.854)), WireLayoutCommand::MoveTo((-899.650, -322.792)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.763, -4.780)), WireLayoutCommand::MoveTo((-985.080, -203.774)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((4.093, -5.380)), WireLayoutCommand::MoveTo((-1048.754, -30.749)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((2.828, -7.446)), WireLayoutCommand::MoveTo((-1057.180, 407.777)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((1.244, -49.295)), WireLayoutCommand::MoveTo((-807.376, 351.312)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-51.029, 1.472)), WireLayoutCommand::MoveTo((-879.629, 172.934)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-6.540, -55.414)), WireLayoutCommand::MoveTo((-516.867, 139.443)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((44.496, 2.641)), WireLayoutCommand::MoveTo((-475.788, 417.203)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.543, -43.717)), WireLayoutCommand::MoveTo((-100.694, 402.988)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-63.512, -5.003)), WireLayoutCommand::MoveTo((-214.320, 180.827)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-17.021, -73.759)), WireLayoutCommand::MoveTo((337.682, 245.213)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.759, -5.525)), WireLayoutCommand::MoveTo((745.087, -116.262)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.006, 30.622)), WireLayoutCommand::MoveTo((417.527, -417.518)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((13.142, 15.011)), WireLayoutCommand::MoveTo((-129.348, -427.396)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((41.230, 1.832)), WireLayoutCommand::MoveTo((-149.952, -61.004)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((6.443, -37.985)), WireLayoutCommand::MoveTo((33.790, -32.354))]);
	circuit.connect((c1, 0), (c3, 0), vec![WireLayoutCommand::MoveTo((-399.349, -1.513)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-327.750, -103.184)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-353.526, -242.085)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((-429.420, -353.779)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((-576.914, -428.241)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((-780.254, -441.129)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((-922.019, -363.803)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-1022.257, -232.061)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-1092.424, -47.337)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-1103.880, 406.599)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-806.029, 398.007)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((-833.237, 167.459)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-519.635, 186.075)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-522.499, 416.623)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((-104.363, 449.558)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-168.801, 170.323)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((326.661, 290.609)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((774.869, -80.272)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((452.675, -448.289)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((-127.274, -474.064)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -0.000)), WireLayoutCommand::MoveTo((-196.009, -68.816)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 0.000)), WireLayoutCommand::MoveTo((17.355, 11.374))]);
	circuit.connect((c5, 0), (c4, 0), vec![WireLayoutCommand::MoveTo((-401.450, 42.146)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((24.002, 1.155)), WireLayoutCommand::MoveTo((-292.013, -78.016)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((10.597, -15.047)), WireLayoutCommand::MoveTo((-310.550, -250.060)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-1.672, -9.011)), WireLayoutCommand::MoveTo((-393.267, -378.344)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-6.357, -9.356)), WireLayoutCommand::MoveTo((-557.215, -467.260)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.995, -4.037)), WireLayoutCommand::MoveTo((-777.489, -484.751)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-12.606, -0.799)), WireLayoutCommand::MoveTo((-942.950, -402.175)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.199, 4.472)), WireLayoutCommand::MoveTo((-1057.043, -258.528)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-3.830, 5.034)), WireLayoutCommand::MoveTo((-1133.285, -62.857)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-2.647, 6.967)), WireLayoutCommand::MoveTo((-1147.575, 405.496)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-1.164, 46.125)), WireLayoutCommand::MoveTo((-804.769, 441.698)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((47.746, -1.377)), WireLayoutCommand::MoveTo((-789.828, 162.337)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((6.119, 51.849)), WireLayoutCommand::MoveTo((-522.225, 229.708)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-41.634, -2.471)), WireLayoutCommand::MoveTo((-566.205, 416.080)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.508, 40.905)), WireLayoutCommand::MoveTo((-107.795, 493.133)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.426, 4.681)), WireLayoutCommand::MoveTo((-126.211, 160.495)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.926, 69.014)), WireLayoutCommand::MoveTo((316.349, 333.085)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((21.295, 5.170)), WireLayoutCommand::MoveTo((802.734, -46.597)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((34.626, -28.652)), WireLayoutCommand::MoveTo((485.562, -477.081)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-12.297, -14.046)), WireLayoutCommand::MoveTo((-125.333, -517.731)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-38.577, -1.715)), WireLayoutCommand::MoveTo((-239.103, -76.125)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-6.028, 35.542)), WireLayoutCommand::MoveTo((1.978, 52.290))]);	
	
	circuit
}

#[wasm_bindgen(start)]
pub fn start() {
	log!("Stuff has started!");
	set_panic_hook();
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn test() {
		let mut circuit = bidirectional_example();
		circuit.toggle_switch(0);
		circuit.toggle_switch(1);
		circuit.toggle_switch(0);
	}

	#[bench]
	fn bench_simple_switch_circuit(b: &mut Bencher) {
		let mut circuit = example2();
		b.iter(|| circuit.toggle_switch(0));
	}

	#[bench]
	fn bench_not_gate(b: &mut Bencher) {
		let mut circuit = not_gate_example();
		b.iter(|| circuit.toggle_switch(0));
	}
}
