//! Various logic gate components.

use std::f64::consts::PI;

use crate::add;
use crate::core::{Chip, ChipInternals, Circuit, Pin, Junction, Switch, TextInfo};
use crate::graphics::{Viewport, WireLayoutCommand};
use crate::transistor::{NTransistor, PTransistor};

pub struct AndGate {
	internals: ChipInternals,
	position: (f64, f64),
}

impl AndGate {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-370.0, -200.0));
		let input2 = add!(circuit, Pin, (-370.0, 200.0));

		let nand_gate = add!(circuit, NandGate, (-100.0, 0.0));
		let not_gate = add!(circuit, NotGate, (100.0, 0.0));

		let output = add!(circuit, Pin, (370.0, 0.0));

		circuit.connect((input1, 0), (nand_gate, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input2, 0), (nand_gate, 1), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((nand_gate, 2), (not_gate, 0), vec![]);
		circuit.connect((not_gate, 1), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.15,
			},
			position: pos,
		}
	}
}

impl Chip for AndGate {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc(0.0, 0.0, width * 0.5, -PI * 0.5, PI * 0.5).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.fill();
    }

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc_with_anticlockwise(0.0, 0.0, width * 0.5, PI * 0.5, -PI * 0.5, true).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

    fn draw_back(&self, _ctx: &web_sys::CanvasRenderingContext2d) {

    }
}

pub struct NandGate {
	internals: ChipInternals,
	position: (f64, f64),
}

impl NandGate {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-800.0, -400.0));
		let input2 = add!(circuit, Pin, (-800.0, 400.0));

		let input_junction_1 = add!(circuit, Junction, (-400.0, -200.0), 3);
		let input_junction_2 = add!(circuit, Junction, (-300.0, 400.0), 3);
		
		let n_transistor_1 = add!(circuit, NTransistor, (0.0, 400.0));
		let n_transistor_2 = add!(circuit, NTransistor, (0.0, 200.0));

		let p_transistor_1 = add!(circuit, PTransistor, (-200.0, -200.0));
		let p_transistor_2 = add!(circuit, PTransistor, (200.0, -200.0));

		let offset = circuit.get_components()[p_transistor_1].get_pin_positions()[1].0;

		let on_source = add!(circuit, Switch, (offset, -500.0));
		let off_source = add!(circuit, Switch, (offset, 600.0));

		circuit.toggle_switch(0);

		let on_junction = add!(circuit, Junction, (offset, -350.0), 3);
		let junction_1 = add!(circuit, Junction, (offset + 200.0, 0.0), 3);
		let junction_2 = add!(circuit, Junction, (offset, 0.0), 3);

		let output = add!(circuit, Pin, (790.0, 0.0));

		circuit.connect((input1, 0), (input_junction_1, 0), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((input_junction_1, 1), (n_transistor_2, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((input_junction_1, 2), (p_transistor_1, 0), vec![]);
		
		circuit.connect((input2, 0), (input_junction_2, 0), vec![]);
		circuit.connect((input_junction_2, 1), (n_transistor_1, 0), vec![]);
		circuit.connect((input_junction_2, 2), (p_transistor_2, 0), vec![
			WireLayoutCommand::MoveVertical(-350.0),
			WireLayoutCommand::MoveHorizontal(200.0),
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((on_source, 0), (on_junction, 0), vec![]);
		circuit.connect((on_junction, 1), (p_transistor_1, 1), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((on_junction, 2), (p_transistor_2, 1), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((p_transistor_1, 2), (junction_2, 1), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((p_transistor_2, 2), (junction_1, 1), vec![]);

		circuit.connect((off_source, 0), (n_transistor_1, 1), vec![]);
		circuit.connect((n_transistor_1, 2), (n_transistor_2, 1), vec![]);
		circuit.connect((n_transistor_2, 2), (junction_2, 0), vec![]);

		circuit.connect((junction_2, 2), (junction_1, 0), vec![]);
		circuit.connect((junction_1, 2), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.07,
			},
			position: pos,
		}
	}
}

impl Chip for NandGate {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc(0.0, 0.0, width * 0.5, -PI * 0.5, PI * 0.5).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.fill();
    }

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = self.get_chip_size();

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.line_to(0.0 * width, 0.5 * height);
		ctx.arc_with_anticlockwise(0.0, 0.0, width * 0.5, PI * 0.5, -PI * 0.5, true).unwrap();
		ctx.line_to(0.0, -0.5 * height);
		ctx.line_to(-0.5 * width, -0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

    fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		let width = self.get_chip_size().0;

		ctx.begin_path();
		ctx.arc(width * 0.5 + 12.8, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
    }
}

pub struct NorGate {
	internals: ChipInternals,
	position: (f64, f64),
}

impl NorGate {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-630.0, -400.0));
		let input2 = add!(circuit, Pin, (-630.0, 400.0));

		let input_junction_1 = add!(circuit, Junction, (-400.0, -400.0), 3);
		let input_junction_2 = add!(circuit, Junction, (-300.0, 400.0), 3);

		let n_transistor_1 = add!(circuit, NTransistor, (-200.0, 200.0));
		let n_transistor_2 = add!(circuit, NTransistor, (200.0, 200.0));
		
		let p_transistor_1 = add!(circuit, PTransistor, (0.0, -400.0));
		let p_transistor_2 = add!(circuit, PTransistor, (0.0, -200.0));

		let offset = circuit.get_components()[p_transistor_1].get_pin_positions()[1].0;

		let on_source = add!(circuit, Switch, (offset, -600.0));
		let off_source = add!(circuit, Switch, (offset, 500.0));

		circuit.toggle_switch(0);

		let off_junction = add!(circuit, Junction, (offset, 350.0), 3);
		let junction_1 = add!(circuit, Junction, (offset, 0.0), 3);
		let junction_2 = add!(circuit, Junction, (offset + 200.0, 0.0), 3);

		let output = add!(circuit, Pin, (790.0, 0.0));

		circuit.connect((input1, 0), (input_junction_1, 0), vec![]);
		circuit.connect((input_junction_1, 1), (n_transistor_1, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((input_junction_1, 2), (p_transistor_1, 0), vec![]);
		
		circuit.connect((input2, 0), (input_junction_2, 0), vec![]);
		circuit.connect((input_junction_2, 1), (n_transistor_2, 0), vec![
			WireLayoutCommand::MoveHorizontal(200.0),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input_junction_2, 2), (p_transistor_2, 0), vec![
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((on_source, 0), (p_transistor_1, 1), vec![]);
		circuit.connect((p_transistor_1, 2), (p_transistor_2, 1), vec![]);
		circuit.connect((p_transistor_2, 2), (junction_1, 0), vec![]);

		circuit.connect((off_source, 0), (off_junction, 0), vec![]);
		circuit.connect((off_junction, 1), (n_transistor_1, 1), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((off_junction, 2), (n_transistor_2, 1), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((n_transistor_1, 2), (junction_1, 1), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((n_transistor_2, 2), (junction_2, 1), vec![]);

		circuit.connect((junction_1, 2), (junction_2, 0), vec![]);
		circuit.connect((junction_2, 2), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.07,
			},
			position: pos,
		}
	}
}

impl Chip for NorGate {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.fill();
    }

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = self.get_chip_size();

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

    fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		let width = self.get_chip_size().0;

		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());

		ctx.begin_path();
		ctx.arc(width * 0.5 + 10.0, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
    }
}

pub struct NotGate {
	internals: ChipInternals,
	position: (f64, f64),
}

impl NotGate {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let offset_x = -300.0;
		
		let input = add!(circuit, Pin, (-487.0 + offset_x, 0.0));
		let input_junc = add!(circuit, Junction, (-230.0 + offset_x, 0.0), 3);

		let n_transistor = add!(circuit, NTransistor, (0.0 + offset_x, 150.0));
		let p_transistor = add!(circuit, PTransistor, (0.0 + offset_x, -150.0));

		let offset = circuit.get_components()[n_transistor].get_pin_positions()[1].0;

		let on_source = add!(circuit, Switch, (offset + offset_x, -400.0));
		let off_source = add!(circuit, Switch, (offset + offset_x, 400.0));

		circuit.toggle_switch(0);
		
		let output_junc = add!(circuit, Junction, (230.0 + offset_x, 0.0), 3);
		let output = add!(circuit, Pin, (1150.0 + offset_x, 0.0));

		circuit.connect((input, 0), (input_junc, 0), vec![]);
		circuit.connect((input_junc, 1), (n_transistor, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((input_junc, 2), (p_transistor, 0), vec![WireLayoutCommand::AlignHorizontal]);

		circuit.connect((on_source, 0), (p_transistor, 1), vec![]);
		circuit.connect((off_source, 0), (n_transistor, 1), vec![]);

		circuit.connect((n_transistor, 2), (output_junc, 1), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((p_transistor, 2), (output_junc, 2), vec![WireLayoutCommand::AlignVertical]);
		
		circuit.connect((output_junc, 0), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.07,
			},
			position: pos,
		}
	}
}

impl Chip for NotGate {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
        self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.fill();
    }

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = self.get_chip_size();

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.stroke();
	}

    fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(10.0);
		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;

		ctx.begin_path();
		ctx.arc(width * 0.5 + 12.8, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
    }
}

pub struct OrGate {
	internals: ChipInternals,
	position: (f64, f64),
}

impl OrGate {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-300.0, -200.0));
		let input2 = add!(circuit, Pin, (-300.0, 200.0));

		let nor_gate = add!(circuit, NorGate, (-100.0, 0.0));
		let not_gate = add!(circuit, NotGate, (100.0, 0.0));

		let output = add!(circuit, Pin, (370.0, 0.0));

		circuit.connect((input1, 0), (nor_gate, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((input2, 0), (nor_gate, 1), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((nor_gate, 2), (not_gate, 0), vec![]);
		circuit.connect((not_gate, 1), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.15,
			},
			position: pos,
		}
	}
}

impl Chip for OrGate {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.fill();
    }

    fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.stroke();
    }
	
	fn draw_back(&self, _ctx: &web_sys::CanvasRenderingContext2d) {
		
	}
}

pub struct TriStateBuffer {
	internals: ChipInternals,
	position: (f64, f64),
}

impl TriStateBuffer {
	pub fn new(pos: (f64, f64)) -> Self {
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

		let transistor_offset = circuit.get_components()[p_transistor_1].get_pin_positions()[1].0;

		let on_source = add!(circuit, Switch, (-offset + transistor_offset, -600.0));
		let off_source = add!(circuit, Switch, (-offset + transistor_offset, 600.0));

		circuit.toggle_switch(0);

		let output_junc = add!(circuit, Junction, (-offset + transistor_offset, 0.0), 3);
		let output = add!(circuit, Pin, (55.0 / scale - 50.0, 0.0));

		circuit.connect((input, 0), (input_not, 0), vec![]);
		circuit.connect((input_not, 1), (input_junc, 0), vec![]);
		circuit.connect((input_junc, 1), (p_transistor_2, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((input_junc, 2), (n_transistor_2, 0), vec![WireLayoutCommand::AlignHorizontal]);

		circuit.connect((enable, 0), (enable_junc, 0), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((enable_junc, 1), (enable_not, 0), vec![]);
		circuit.connect((enable_not, 1), (p_transistor_1, 0), vec![]);
		circuit.connect((enable_junc, 2), (n_transistor_1, 0), vec![WireLayoutCommand::AlignHorizontal]);

		circuit.connect((on_source, 0), (p_transistor_2, 1), vec![]);
		circuit.connect((p_transistor_2, 2), (p_transistor_1, 1), vec![]);
		circuit.connect((p_transistor_1, 2), (output_junc, 0), vec![]);

		circuit.connect((off_source, 0), (n_transistor_2, 1), vec![]);
		circuit.connect((n_transistor_2, 2), (n_transistor_1, 1), vec![]);
		circuit.connect((n_transistor_1, 2), (output_junc, 1), vec![]);

		circuit.connect((output_junc, 2), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: scale,
			},
			position: pos,
		}
	}
}

impl Chip for TriStateBuffer {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.fill();
    }

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = self.get_chip_size();

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.stroke();
	}

    fn draw_back(&self, _ctx: &web_sys::CanvasRenderingContext2d) {
		
    }
}

pub struct XorGate {
	internals: ChipInternals,
	position: (f64, f64),
}

impl XorGate {
	pub fn new(pos: (f64, f64)) -> Self {
		let mut circuit = Circuit::new();

		let input1 = add!(circuit, Pin, (-300.0, -200.0));
		let input2 = add!(circuit, Pin, (-300.0, 200.0));

		let junction1 = add!(circuit, Junction, (-220.0, -200.0), 3);
		let junction2 = add!(circuit, Junction, (-190.0, 200.0), 3);

		let or_gate = add!(circuit, OrGate, (-100.0, -150.0));
		let nand_gate = add!(circuit, NandGate, (-100.0, 150.0));
		let and_gate = add!(circuit, AndGate, (100.0, 0.0));

		let output = add!(circuit, Pin, (370.0, 0.0));
		
		circuit.connect((input1, 0), (junction1, 0), vec![]);
		circuit.connect((junction1, 1), (or_gate, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((junction1, 2), (nand_gate, 0), vec![
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((input2, 0), (junction2, 0), vec![]);
		circuit.connect((junction2, 1), (or_gate, 1), vec![
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((junction2, 2), (nand_gate, 1), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);

		circuit.connect((or_gate, 2), (and_gate, 0), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((nand_gate, 2), (and_gate, 1), vec![
			WireLayoutCommand::CenterHorizontal,
			WireLayoutCommand::AlignHorizontal,
		]);
		
		circuit.connect((and_gate, 2), (output, 0), vec![]);

		Self {
			internals: ChipInternals {
				circuit,
				inner_scale: 0.15,
			},
			position: pos,
		}
	}
}

impl Chip for XorGate {
    fn get_chip_internals(&self) -> &ChipInternals {
        &self.internals
    }

    fn get_chip_internals_mut(&mut self) -> &mut ChipInternals {
        &mut self.internals
    }

    fn get_chip_position(&self) -> (f64, f64) {
		self.position
    }

    fn get_chip_size(&self) -> (f64, f64) {
        (110.0, 110.0)
    }

    fn get_text_info(&self) -> Option<&TextInfo> {
        todo!()
    }

    fn contains(&self, _viewport: &Viewport) -> bool {
		false
    }

    fn intersects(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
    }

	fn are_internals_visible(&self, _viewport: &Viewport) -> bool {
		true
	}

    fn draw_front(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.fill();
    }

	fn draw_edge(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());
		
		let (width, height) = self.get_chip_size();

		ctx.begin_path();
		ctx.move_to(-0.5 * width, 0.5 * height);
		ctx.bezier_curve_to(-0.33 * width, 0.25 * height, -0.33 * width, -0.25 * height, -0.5 * width, -0.5 * height);
		ctx.bezier_curve_to(0.0 * width, -0.5 * height, 0.25 * width, -0.5 * height, 0.5 * width, 0.0 * height);
		ctx.bezier_curve_to(0.25 * width, 0.5 * height, 0.0 * width, 0.5 * height, -0.5 * width, 0.5 * height);
		ctx.close_path();

		ctx.stroke();
	}

    fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(5.0);
		ctx.set_stroke_style(&"#fff".into());

		let xor_line_offset = 20.0;
		let (width, height) = self.get_chip_size();

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
