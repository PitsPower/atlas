use crate::{core::Component, graphics::Drawable};

pub struct NTransistor {
	position: (f64, f64),
}

impl NTransistor {
	pub fn new(pos: (f64, f64)) -> Self {
		Self {
			position: pos,
		}
	}
}

impl Drawable for NTransistor {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
		ctx.set_line_width(7.0);
		ctx.set_line_cap("square");
		ctx.set_stroke_style(&"#fff".into());

		ctx.begin_path();
		ctx.move_to(50.0, 80.0);
		ctx.line_to(-50.0, 80.0);
		ctx.line_to(-50.0, -80.0);
		ctx.line_to(50.0, -80.0);
		ctx.stroke();

		ctx.begin_path();
		ctx.move_to(-65.0, 80.0);
		ctx.line_to(-65.0, -80.0);
		ctx.stroke();
    }

    fn get_pin_positions(&self) -> Vec<(f64, f64)> {
        vec![
			(-65.0, 0.0),
			(50.0, 80.0),
			(50.0, -80.0),
		]
    }
}

impl Component for NTransistor {
	fn get_position(&self) -> (f64, f64) {
		self.position
	}
}
