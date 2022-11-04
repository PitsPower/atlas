//! Latch and flip-flop components.

use crate::add;
use crate::core::{ChipInternals, Circuit, Junction, Pin, RectangleChip, TextInfo};
use crate::gates::{AndGate, NorGate, NotGate};
use crate::graphics::WireLayoutCommand;

pub struct SRLatch;

impl SRLatch {
	pub fn new(pos: (f64, f64)) -> RectangleChip {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-250.0, -100.0));
		let input2 = add!(circuit, Pin, (-250.0, 100.0));
		
		let nor1 = add!(circuit, NorGate, (0.0, -100.0));
		let nor2 = add!(circuit, NorGate, (0.0, 100.0));
	
		let junction1 = add!(circuit, Junction, (150.0, -100.0), 3);
		let junction2 = add!(circuit, Junction, (150.0, 100.0), 3);
	
		let output1 = add!(circuit, Pin, (250.0, -100.0));
		let output2 = add!(circuit, Pin, (250.0, 100.0));
	
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
	
		// Order matters here. This order ensures that the latch is off by default.
		circuit.connect((junction2, 2), (nor1, 1), vec![
			WireLayoutCommand::MoveVertical(-35.0),
			WireLayoutCommand::Move((-250.0, -100.0)),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((junction1, 2), (nor2, 0), vec![
			WireLayoutCommand::MoveVertical(35.0),
			WireLayoutCommand::Move((-250.0, 100.0)),
			WireLayoutCommand::AlignHorizontal,
		]);

		RectangleChip {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.8,
			},
			position: pos,
			size: (400.0, 400.0),
			text: Some(TextInfo {
				text: String::from("SR Latch"),
				size: 70,
			}),
		}
	}
}

pub struct DLatch;

impl DLatch {
	pub fn new(pos: (f64, f64)) -> RectangleChip {
		let mut circuit = Circuit::new();

		let offset = 200.0;

		let latch = add!(circuit, SRLatch, (offset, 0.0));
	
		let latch_offset = circuit.get_components()[latch].as_ref().get_pin_positions()[1].1;
	
		let not = add!(circuit, NotGate, (offset - 570.0, -latch_offset));
		let and1 = add!(circuit, AndGate, (offset - 320.0, -latch_offset));
		let and2 = add!(circuit, AndGate, (offset - 320.0, latch_offset));
	
		let and_offset = circuit.get_components()[and1].as_ref().get_pin_positions()[1].1;
	
		let input = add!(circuit, Pin, (offset - 800.0, -200.0));
		let clock = add!(circuit, Pin, (offset - 800.0, 200.0));
	
		let input_junc = add!(circuit, Junction, (offset - 680.0, -latch_offset), 3);
		let clock_junc = add!(circuit, Junction, (offset - 420.0, latch_offset + and_offset), 3);
	
		let output1 = add!(circuit, Pin, (offset + 400.0, -200.0));
		let output2 = add!(circuit, Pin, (offset + 400.0, 200.0));
	
		circuit.connect((input, 0), (input_junc, 0), vec![
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((input_junc, 1), (not, 0), vec![]);
		circuit.connect((not, 1), (and1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input_junc, 2), (and2, 0), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
	
		circuit.connect((clock, 0), (clock_junc, 0), vec![
			WireLayoutCommand::AlignVertical,
		]);
		circuit.connect((clock_junc, 1), (and1, 1), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock_junc, 2), (and2, 1), vec![]);
		
		circuit.connect((and1, 2), (latch, 0), vec![]);
		circuit.connect((and2, 2), (latch, 1), vec![]);
		
		circuit.connect((latch, 2), (output1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((latch, 3), (output2, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);

		RectangleChip {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.5,
			},
			position: pos,
			size: (600.0, 400.0),
			text: Some(TextInfo {
				text: String::from("D Latch"),
				size: 70,
			}),
		}
	}
}

pub struct DFlipFlop;

impl DFlipFlop {
	pub fn new(pos: (f64, f64)) -> RectangleChip {
		let mut circuit = Circuit::new();
		
		let offset = 100.0;
		
		let latch1 = add!(circuit, DLatch, (offset - 400.0, 0.0));
		let latch2 = add!(circuit, DLatch, (offset + 400.0, 0.0));

		let offset = circuit.get_components()[latch1].as_ref().get_pin_positions()[1].1;

		let input = add!(circuit, Pin, (offset - 1100.0, -300.0));
		let clock = add!(circuit, Pin, (offset - 1100.0, 300.0));

		let clock_junc = add!(circuit, Junction, (offset - 965.0, 300.0), 3);

		let not = add!(circuit, NotGate, (offset - 840.0, offset));

		let output1 = add!(circuit, Pin, (offset + 900.0, -300.0));
		let output2 = add!(circuit, Pin, (offset + 900.0, 300.0));

		circuit.connect((input, 0), (latch1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock, 0), (clock_junc, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		
		circuit.connect((clock_junc, 1), (latch2, 1), vec![
			WireLayoutCommand::MoveHorizontal(960.0),
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((clock_junc, 2), (not, 0), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((not, 1), (latch1, 1), vec![]);

		circuit.connect((latch1, 2), (latch2, 0), vec![]);
		
		circuit.connect((latch2, 2), (output1, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((latch2, 3), (output2, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);

		RectangleChip {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.3,
			},
			position: pos,
			size: (600.0, 400.0),
			text: Some(TextInfo {
				text: String::from("D Flip-Flop"),
				size: 70,
			}),
		}
	}
}

pub struct MultiDFlipFlop;

impl MultiDFlipFlop {
	pub fn new(pos: (f64, f64), size: usize) -> RectangleChip {
		let mut circuit = Circuit::new();

		let scale = 0.19;
		let width = 400.0;
		let height = size as f64 * 100.0;
		let wire_spacing = 90.0;

		let flip_flops: Vec<_> = (0..size)
			.map(|i| add!(circuit, DFlipFlop, (0.0, -(i as f64 - (size as f64) * 0.5) * 500.0 - 250.0)))
			.collect();

		let offset = circuit.get_components()[flip_flops[0]].as_ref().get_pin_positions()[1].1;

		let input_group: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (-width * 0.5 / scale, -((i as f64 - (size as f64) * 0.5 + 0.5) * wire_spacing))))
			.collect();

		let clock = add!(circuit, Pin, (0.0, height * 0.5 / scale));

		let clock_juncs: Vec<_> = (0..size - 1)
		.map(|i| add!(circuit, Junction, (-450.0, offset - (i as f64 - (size as f64) * 0.5) * 500.0 - 250.0), 3))
		.collect();

		let output_group: Vec<_> = (0..size)
			.map(|i| add!(circuit, Pin, (width * 0.5 / scale, -((i as f64 - (size as f64) * 0.5 + 0.5) * wire_spacing))))
			.collect();

		circuit.connect((clock_juncs[size-2], 1), (flip_flops[size-1], 1), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((clock, 0), (clock_juncs[0], 0), vec![
			WireLayoutCommand::MoveVertical(-70.0),
			WireLayoutCommand::AlignVertical,
		]);

		for i in 0..size {
			circuit.connect((input_group[i], 0), (flip_flops[i], 0), vec![
				WireLayoutCommand::CenterHorizontal,
				WireLayoutCommand::MoveHorizontal((if i < size/2 { i } else { size - i - 1 }) as f64 * 30.0),
				WireLayoutCommand::AlignHorizontal,
			]);
			circuit.connect((flip_flops[i], 2), (output_group[i], 0), vec![
				WireLayoutCommand::MoveHorizontal(150.0),
				WireLayoutCommand::MoveHorizontal((if i < size/2 { size - i - 1 } else { i }) as f64 * 30.0),
				WireLayoutCommand::AlignHorizontal,
			]);
		}

		for i in 0..size-1 {
			circuit.connect((clock_juncs[i], 2), (flip_flops[i], 1), vec![]);
		}
		for i in 0..size-2 {
			circuit.connect((clock_juncs[i], 1), (clock_juncs[i+1], 0), vec![]);
		}

		RectangleChip {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.19,
			},
			position: pos,
			size: (400.0, size as f64 * 100.0),
			text: Some(TextInfo {
				text: format!("{}-bit D Flip-Flop", size),
				size: 40,
			}),
		}
	}
}
