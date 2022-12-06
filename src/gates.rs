//! Various logic gate components.

use std::f64::consts::PI;

use crate::add;
use crate::bus::*;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::{ChipDrawer, WireLayoutCommand};

pub fn get_and_gate_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Pin, (-370.0, -200.0));
	let input2 = add!(circuit, Pin, (-370.0, 200.0));

	let nand_gate = add!(circuit, NandGate, (-100.0, 0.0));
	let not_gate = add!(circuit, NotGate, (100.0, 0.0));

	let output = add!(circuit, Pin, (370.0, 0.0));

	circuit.connect((input1, 0), (nand_gate, 0), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (nand_gate, 1), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((nand_gate, 2), (not_gate, 0), &[]);
	circuit.connect((not_gate, 1), (output, 0), &[]);

	circuit
}

pub fn get_nand_gate_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Pin, (-800.0, -400.0));
	let input2 = add!(circuit, Pin, (-800.0, 400.0));

	let input_junction_1 = add!(circuit, Junction, (-400.0, -200.0), 3);
	let input_junction_2 = add!(circuit, Junction, (-300.0, 400.0), 3);
	
	let n_transistor_1 = add!(circuit, NTransistor, (0.0, 400.0));
	let n_transistor_2 = add!(circuit, NTransistor, (0.0, 200.0));

	let p_transistor_1 = add!(circuit, PTransistor, (-200.0, -200.0));
	let p_transistor_2 = add!(circuit, PTransistor, (200.0, -200.0));

	let offset = circuit.components[p_transistor_1].get_pin_positions()[1].0;

	let on_source = add!(circuit, Switch, (offset, -500.0));
	let off_source = add!(circuit, Switch, (offset, 600.0));

	circuit.toggle_switch(0);

	let on_junction = add!(circuit, Junction, (offset, -350.0), 3);
	let junction_1 = add!(circuit, Junction, (offset + 200.0, 0.0), 3);
	let junction_2 = add!(circuit, Junction, (offset, 0.0), 3);

	let output = add!(circuit, Pin, (790.0, 0.0));

	circuit.connect((input1, 0), (input_junction_1, 0), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((input_junction_1, 1), (n_transistor_2, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((input_junction_1, 2), (p_transistor_1, 0), &[]);
	
	circuit.connect((input2, 0), (input_junction_2, 0), &[]);
	circuit.connect((input_junction_2, 1), (n_transistor_1, 0), &[]);
	circuit.connect((input_junction_2, 2), (p_transistor_2, 0), &[
		WireLayoutCommand::MoveVertical(-350.0),
		WireLayoutCommand::MoveHorizontal(200.0),
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((on_source, 0), (on_junction, 0), &[]);
	circuit.connect((on_junction, 1), (p_transistor_1, 1), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((on_junction, 2), (p_transistor_2, 1), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((p_transistor_1, 2), (junction_2, 1), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((p_transistor_2, 2), (junction_1, 1), &[]);

	circuit.connect((off_source, 0), (n_transistor_1, 1), &[]);
	circuit.connect((n_transistor_1, 2), (n_transistor_2, 1), &[]);
	circuit.connect((n_transistor_2, 2), (junction_2, 0), &[]);

	circuit.connect((junction_2, 2), (junction_1, 0), &[]);
	circuit.connect((junction_1, 2), (output, 0), &[]);

	circuit
}

pub fn get_nor_gate_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Pin, (-630.0, -400.0));
	let input2 = add!(circuit, Pin, (-630.0, 400.0));

	let input_junction_1 = add!(circuit, Junction, (-400.0, -400.0), 3);
	let input_junction_2 = add!(circuit, Junction, (-300.0, 400.0), 3);

	let n_transistor_1 = add!(circuit, NTransistor, (-200.0, 200.0));
	let n_transistor_2 = add!(circuit, NTransistor, (200.0, 200.0));
	
	let p_transistor_1 = add!(circuit, PTransistor, (0.0, -400.0));
	let p_transistor_2 = add!(circuit, PTransistor, (0.0, -200.0));

	let offset = circuit.components[p_transistor_1].get_pin_positions()[1].0;

	let on_source = add!(circuit, Switch, (offset, -600.0));
	let off_source = add!(circuit, Switch, (offset, 500.0));

	circuit.toggle_switch(0);

	let off_junction = add!(circuit, Junction, (offset, 350.0), 3);
	let junction_1 = add!(circuit, Junction, (offset, 0.0), 3);
	let junction_2 = add!(circuit, Junction, (offset + 200.0, 0.0), 3);

	let output = add!(circuit, Pin, (790.0, 0.0));

	circuit.connect((input1, 0), (input_junction_1, 0), &[]);
	circuit.connect((input_junction_1, 1), (n_transistor_1, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((input_junction_1, 2), (p_transistor_1, 0), &[]);
	
	circuit.connect((input2, 0), (input_junction_2, 0), &[]);
	circuit.connect((input_junction_2, 1), (n_transistor_2, 0), &[
		WireLayoutCommand::MoveHorizontal(200.0),
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input_junction_2, 2), (p_transistor_2, 0), &[
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((on_source, 0), (p_transistor_1, 1), &[]);
	circuit.connect((p_transistor_1, 2), (p_transistor_2, 1), &[]);
	circuit.connect((p_transistor_2, 2), (junction_1, 0), &[]);

	circuit.connect((off_source, 0), (off_junction, 0), &[]);
	circuit.connect((off_junction, 1), (n_transistor_1, 1), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((off_junction, 2), (n_transistor_2, 1), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((n_transistor_1, 2), (junction_1, 1), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((n_transistor_2, 2), (junction_2, 1), &[]);

	circuit.connect((junction_1, 2), (junction_2, 0), &[]);
	circuit.connect((junction_2, 2), (output, 0), &[]);

	circuit
}

pub fn get_not_gate_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let offset_x = -300.0;
	
	let input = add!(circuit, Pin, (-487.0 + offset_x, 0.0));
	let input_junc = add!(circuit, Junction, (-230.0 + offset_x, 0.0), 3);

	let n_transistor = add!(circuit, NTransistor, (0.0 + offset_x, 150.0));
	let p_transistor = add!(circuit, PTransistor, (0.0 + offset_x, -150.0));

	let offset = circuit.components[n_transistor].get_pin_positions()[1].0;

	let on_source = add!(circuit, Switch, (offset + offset_x, -400.0));
	let off_source = add!(circuit, Switch, (offset + offset_x, 400.0));

	circuit.toggle_switch(0);
	
	let output_junc = add!(circuit, Junction, (230.0 + offset_x, 0.0), 3);
	let output = add!(circuit, Pin, (1150.0 + offset_x, 0.0));

	circuit.connect((input, 0), (input_junc, 0), &[]);
	circuit.connect((input_junc, 1), (n_transistor, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((input_junc, 2), (p_transistor, 0), &[WireLayoutCommand::AlignHorizontal]);

	circuit.connect((on_source, 0), (p_transistor, 1), &[]);
	circuit.connect((off_source, 0), (n_transistor, 1), &[]);

	circuit.connect((n_transistor, 2), (output_junc, 1), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((p_transistor, 2), (output_junc, 2), &[WireLayoutCommand::AlignVertical]);
	
	circuit.connect((output_junc, 0), (output, 0), &[]);

	circuit
}

pub fn get_or_gate_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Pin, (-300.0, -200.0));
	let input2 = add!(circuit, Pin, (-300.0, 200.0));

	let nor_gate = add!(circuit, NorGate, (-100.0, 0.0));
	let not_gate = add!(circuit, NotGate, (100.0, 0.0));

	let output = add!(circuit, Pin, (370.0, 0.0));

	circuit.connect((input1, 0), (nor_gate, 0), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((input2, 0), (nor_gate, 1), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((nor_gate, 2), (not_gate, 0), &[]);
	circuit.connect((not_gate, 1), (output, 0), &[]);

	circuit
}

pub fn get_tri_state_buffer_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let scale = 0.04;

	let offset = 300.0;

	let input = add!(circuit, Pin, (-55.0 / scale, 0.0));
	let enable = add!(circuit, Pin, (-offset + 300.0, -700.0));

	let input_junc = add!(circuit, Junction, (-offset - 600.0, 0.0), 3);
	let enable_junc = add!(circuit, Junction, (-offset - 500.0, -200.0), 3);

	let input_not = add!(circuit, NotGate, (-offset - 800.0, 0.0));
	let enable_not = add!(circuit, NotGate, (-offset - 300.0, -200.0));

	let p_transistor_1 = add!(circuit, PTransistor, (-offset, -200.0));
	let p_transistor_2 = add!(circuit, PTransistor, (-offset, -400.0));

	let n_transistor_1 = add!(circuit, NTransistor, (-offset, 200.0));
	let n_transistor_2 = add!(circuit, NTransistor, (-offset, 400.0));

	let transistor_offset = circuit.components[p_transistor_1].get_pin_positions()[1].0;

	let on_source = add!(circuit, Switch, (-offset + transistor_offset, -600.0));
	let off_source = add!(circuit, Switch, (-offset + transistor_offset, 600.0));

	circuit.toggle_switch(0);

	let output_junc = add!(circuit, Junction, (-offset + transistor_offset, 0.0), 3);
	let output = add!(circuit, Pin, (55.0 / scale - 50.0, 0.0));

	circuit.connect((input, 0), (input_not, 0), &[]);
	circuit.connect((input_not, 1), (input_junc, 0), &[]);
	circuit.connect((input_junc, 1), (p_transistor_2, 0), &[WireLayoutCommand::AlignHorizontal]);
	circuit.connect((input_junc, 2), (n_transistor_2, 0), &[WireLayoutCommand::AlignHorizontal]);

	circuit.connect((enable, 0), (enable_junc, 0), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((enable_junc, 1), (enable_not, 0), &[]);
	circuit.connect((enable_not, 1), (p_transistor_1, 0), &[]);
	circuit.connect((enable_junc, 2), (n_transistor_1, 0), &[WireLayoutCommand::AlignHorizontal]);

	circuit.connect((on_source, 0), (p_transistor_2, 1), &[]);
	circuit.connect((p_transistor_2, 2), (p_transistor_1, 1), &[]);
	circuit.connect((p_transistor_1, 2), (output_junc, 0), &[]);

	circuit.connect((off_source, 0), (n_transistor_2, 1), &[]);
	circuit.connect((n_transistor_2, 2), (n_transistor_1, 1), &[]);
	circuit.connect((n_transistor_1, 2), (output_junc, 1), &[]);

	circuit.connect((output_junc, 2), (output, 0), &[]);

	circuit
}

pub fn get_multi_tri_state_buffer_circuit(size: usize) -> Circuit {
	let mut circuit = Circuit::new();

	let chip_width = 450.0;
	let scale = 0.3;

	let input_group: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (-chip_width * 0.3 / scale, (i as f64 - size as f64 * 0.5 - 0.5) * 50.0)))
		.collect();

	let enable = add!(circuit, Pin, (0.0, -100.0 * size as f64));

	let junctions: Vec<_> = (0..size-1)
		.map(|i| add!(circuit, Junction, (100.0, (i as f64 - size as f64 * 0.5 + 0.125) * 150.0 - 70.0), 3))
		.collect();

	let buffers: Vec<_> = (0..size)
		.map(|i| add!(circuit, TriStateBuffer, (0.0, (i as f64 - size as f64 * 0.5 + 0.125) * 150.0)))
		.collect();
		
	let output_group: Vec<_> = (0..size)
		.map(|i| add!(circuit, Pin, (chip_width * 0.3 / scale, (i as f64 - size as f64 * 0.5 - 0.5) * 50.0)))
		.collect();

	let input_positions: Vec<_> = input_group.iter().map(|i| circuit.components[*i].position).collect();

	let buffer_input_positions: Vec<_> = buffers.iter()
		.map(|i| {
			let pos = circuit.components[*i].position;
			let pin_pos = circuit.components[*i].get_pin_positions()[0];
			(pos.0 + pin_pos.0, pos.1 + pin_pos.1)
		})
		.collect();

	let buffer_output_positions: Vec<_> = buffers.iter()
		.map(|i| {
			let pos = circuit.components[*i].position;
			let pin_pos = circuit.components[*i].get_pin_positions()[2];
			(pos.0 + pin_pos.0, pos.1 + pin_pos.1)
		})
		.collect();

	let output_positions: Vec<_> = output_group.iter().map(|i| circuit.components[*i].position).collect();
		
	let commands1 = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&input_positions[..size/2],
		&buffer_input_positions[..size/2],
	);
	let commands2 = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&input_positions[size/2..],
		&buffer_input_positions[size/2..],
	);
	let commands3 = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&output_positions[..size/2],
		&buffer_output_positions[..size/2],
	);
	let commands4 = compute_wire_commands(
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
		&output_positions[size/2..],
		&buffer_output_positions[size/2..],
	);

	for idx in 0..size/2 {
		circuit.connect((input_group[idx], 0), (buffers[idx], 0), &commands1[idx]);
		circuit.connect((output_group[idx], 0), (buffers[idx], 2), &commands3[idx]);
	}
	for idx in size/2..size {
		circuit.connect((input_group[idx], 0), (buffers[idx], 0), &commands2[idx - size/2]);
		circuit.connect((output_group[idx], 0), (buffers[idx], 2), &commands4[idx - size/2]);
	}

	for idx in 0..size-2 {
		circuit.connect((junctions[idx], 1), (junctions[idx + 1], 0), &[]);
		circuit.connect((junctions[idx], 2), (buffers[idx], 1), &[WireLayoutCommand::AlignVertical]);
	}
	
	circuit.connect((enable, 0), (junctions[0], 0), &[
		WireLayoutCommand::CenterVertical,
		WireLayoutCommand::AlignVertical,
	]);
	circuit.connect((junctions[size-2], 2), (buffers[size-2], 1), &[WireLayoutCommand::AlignVertical]);
	circuit.connect((junctions[size-2], 1), (buffers[size-1], 1), &[
		WireLayoutCommand::MoveVertical(150.0),
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}

pub fn get_xor_gate_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let input1 = add!(circuit, Pin, (-300.0, -200.0));
	let input2 = add!(circuit, Pin, (-300.0, 200.0));

	let junction1 = add!(circuit, Junction, (-220.0, -200.0), 3);
	let junction2 = add!(circuit, Junction, (-190.0, 200.0), 3);

	let or_gate = add!(circuit, OrGate, (-100.0, -150.0));
	let nand_gate = add!(circuit, NandGate, (-100.0, 150.0));
	let and_gate = add!(circuit, AndGate, (100.0, 0.0));

	let output = add!(circuit, Pin, (370.0, 0.0));
	
	circuit.connect((input1, 0), (junction1, 0), &[]);
	circuit.connect((junction1, 1), (or_gate, 0), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((junction1, 2), (nand_gate, 0), &[
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((input2, 0), (junction2, 0), &[]);
	circuit.connect((junction2, 1), (or_gate, 1), &[
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((junction2, 2), (nand_gate, 1), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((or_gate, 2), (and_gate, 0), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((nand_gate, 2), (and_gate, 1), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	
	circuit.connect((and_gate, 2), (output, 0), &[]);

	circuit
}

pub struct AndGateDrawer;

impl AndGateDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for AndGateDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for AndGateDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc(0.0, 0.0, width * 0.5, -PI * 0.5, PI * 0.5).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());

		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc_with_anticlockwise(0.0, 0.0, width * 0.5, PI * 0.5, -PI * 0.5, true).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component)
	}
}

pub struct NandGateDrawer;

impl NandGateDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for NandGateDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for NandGateDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc(0.0, 0.0, width * 0.5, -PI * 0.5, PI * 0.5).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc_with_anticlockwise(0.0, 0.0, width * 0.5, PI * 0.5, -PI * 0.5, true).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component);

		let width = component.size.0;

		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());

		ctx.begin_path();
		ctx.arc(width * 0.5 + 12.8, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
	}
}

pub struct NorGateDrawer;

impl NorGateDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for NorGateDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for NorGateDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component);

		let width = component.size.0;

		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		ctx.begin_path();
		ctx.arc(width * 0.5 + 10.0, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
	}
}

pub struct NotGateDrawer;

impl NotGateDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for NotGateDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for NotGateDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component);

		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());
		
		let width = component.size.0;

		ctx.begin_path();
		ctx.arc(width * 0.5 + 12.8, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
	}
}

pub struct OrGateDrawer;

impl OrGateDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for OrGateDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for OrGateDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component);
	}
}

pub struct TriStateBufferDrawer;

impl TriStateBufferDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for TriStateBufferDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for TriStateBufferDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component);
	}
}

pub struct XorGateDrawer;

impl XorGateDrawer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for XorGateDrawer {
	fn default() -> Self {
		Self::new()
	}
}

impl ChipDrawer for XorGateDrawer {
	fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_fill_style(&"#000".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.fill();
	}

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

	fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d, component: &crate::core::Component) {
		self.draw_front(ctx, component);

		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());

		let xor_line_offset = 20.0;
		let (width, height) = component.size;

		ctx.begin_path();
		ctx.move_to(-0.5 * width - xor_line_offset, 0.5 * height);
		ctx.bezier_curve_to(
			-0.3 * width - xor_line_offset,
			0.25 * height,
			-0.3 * width - xor_line_offset,
			-0.25 * height,
			-0.5 * width - xor_line_offset,
			-0.5 * height,
		);

		ctx.stroke();
	}
}
