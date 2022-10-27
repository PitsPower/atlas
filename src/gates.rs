use std::f64::consts::PI;

use crate::add;
use crate::core::{Chip, ChipInternals, Circuit, Pin, Junction, Switch};
use crate::graphics::{Viewport, WireLayoutCommand};
use crate::transistor::{NTransistor, PTransistor};

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
		let output = add!(circuit, Pin, (1080.0 + offset_x, 0.0));

		circuit.connect((input, 0), (input_junc, 0), vec![]);
		circuit.connect((input_junc, 1), (n_transistor, 0), vec![WireLayoutCommand::AlignHorizontal]);
		circuit.connect((input_junc, 2), (p_transistor, 0), vec![WireLayoutCommand::AlignHorizontal]);

		circuit.connect((on_source, 0), (p_transistor, 1), vec![]);
		circuit.connect((off_source, 0), (n_transistor, 1), vec![]);

		circuit.connect((n_transistor, 2), (output_junc, 1), vec![WireLayoutCommand::AlignVertical]);
		circuit.connect((p_transistor, 2), (output_junc, 2), vec![WireLayoutCommand::AlignVertical]);
		
		circuit.connect((output_junc, 0), (output, 0), vec![]);

		NotGate {
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

    fn contains_chip(&self, viewport: &Viewport) -> bool {
        // TODO: Implement
		false
    }

    fn intersects_chip(&self, viewport: &Viewport) -> bool {
		let size = self.get_chip_size();

		let intersects_x =
			self.position.0 + size.0 * 0.5 >= viewport.get_position().0 - viewport.get_size().0 * 0.5 &&
			self.position.0 - size.0 * 0.5 <= viewport.get_position().0 + viewport.get_size().0 * 0.5;

		let intersects_y =
			self.position.1 + size.1 * 0.5 >= viewport.get_position().1 - viewport.get_size().1 * 0.5 &&
			self.position.1 - size.1 * 0.5 <= viewport.get_position().1 + viewport.get_size().1 * 0.5;

		intersects_x && intersects_y
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

    fn draw_back(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(10.0);

		ctx.set_stroke_style(&"#fff".into());
		ctx.set_fill_style(&"#000".into());
		
		let width = self.get_chip_size().0;
		let height = self.get_chip_size().1;

		ctx.begin_path();
		ctx.move_to(-width * 0.5, -height * 0.5);
		ctx.line_to(-width * 0.5, height * 0.5);
		ctx.line_to(width * 0.5, 0.0);
		ctx.close_path();

		ctx.stroke();
		ctx.fill();

		ctx.begin_path();
		ctx.arc(width * 0.5 + 15.0, 0.0, 7.0, 0.0, 2.0 * PI).unwrap();
		ctx.stroke();
		ctx.fill();
    }
}
