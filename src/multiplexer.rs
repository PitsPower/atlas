//! Multiplexer components.

use crate::add;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;

pub fn get_multiplexer_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, AndGate, (0.000, -100.000));
	let c1 = add!(circuit, AndGate, (0.000, 100.000));
	let c2 = add!(circuit, NotGate, (-200.000, 0.000));
	let c3 = add!(circuit, Junction, (-300.000, 100.000), 3);
	let c4 = add!(circuit, Pin, (-400.000, -200.000));
	let c5 = add!(circuit, Pin, (-400.000, 200.000));
	let c6 = add!(circuit, Pin, (400.000, 0.000));
	let c7 = add!(circuit, OrGate, (200.000, 0.000));
	let c8 = add!(circuit, Pin, (0.000, 350.000));
	
	circuit.connect((c2, 1), (c0, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 0), (c3, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 1), (c2, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 0), (c1, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 0), (c0, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 2), (c7, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 2), (c7, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c7, 2), (c6, 0), &[]);
	circuit.connect((c8, 0), (c3, 2), &[WireLayoutCommand::MoveTo((0.000, 231.313)), WireLayoutCommand::AlignVertical]);

	circuit
}

pub fn get_two_bit_multiplexer_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Multiplexer, (-250.000, -200.000));
	let c1 = add!(circuit, Multiplexer, (-250.000, 200.000));
	let c2 = add!(circuit, Multiplexer, (250.000, 0.000));
	let c3 = add!(circuit, Pin, (-550.000, -100.000));
	let c4 = add!(circuit, Pin, (-550.000, 100.000));
	let c5 = add!(circuit, Pin, (-550.000, -300.000));
	let c6 = add!(circuit, Pin, (-550.000, 300.000));
	let c7 = add!(circuit, Pin, (550.000, 0.000));
	let c8 = add!(circuit, Pin, (100.000, 500.000));
	let c9 = add!(circuit, Pin, (-100.000, 500.000));
	let c10 = add!(circuit, Junction, (50.000, 400.000), 3);
	
	circuit.connect((c0, 2), (c2, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 2), (c2, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 0), (c0, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 0), (c0, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c4, 0), (c1, 0), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 0), (c1, 1), &[WireLayoutCommand::CenterHorizontal, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c2, 2), (c7, 0), &[]);
	circuit.connect((c8, 0), (c10, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c10, 1), (c1, 3), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((c10, 2), (c0, 3), &[WireLayoutCommand::MoveTo((50.000, 0.000)), WireLayoutCommand::AlignVertical]);
	circuit.connect((c9, 0), (c2, 3), &[WireLayoutCommand::MoveTo((-100.000, 450.000)), WireLayoutCommand::AlignVertical]);

	circuit
}
