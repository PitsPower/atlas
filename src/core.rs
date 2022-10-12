use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::graphics::{Drawable, Viewport};

pub trait Component: Drawable {
    fn as_chip(&self) -> Option<&Chip> {
        None
    }

    fn contains(&self, viewport: &Viewport) -> bool;
}

#[wasm_bindgen]
pub struct Circuit {
    components: Vec<Box<dyn Component>>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            components: vec![],
        }
    }

	pub fn get_components(&self) -> &Vec<Box<dyn Component>> {
		&self.components
	}

	pub fn add(&mut self, component: Box<dyn Component>) {
		self.components.push(component);
	}
}

impl Drawable for Circuit {
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        for component in &self.components {
            ctx.save();
            component.draw(ctx);
            ctx.restore();
        }
    }
}

pub struct Chip {
    pub circuit: Circuit,

    pub position: (f64, f64),
    pub size: (f64, f64),
    pub inner_scale: f64,
}

impl Drawable for Chip {
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_line_width(10.0);
        
        ctx.set_stroke_style(&"#fff".into());
        ctx.set_fill_style(&"#000".into());

        let x = self.position.0;
        let y = self.position.1;
        let width = self.size.0;
        let height = self.size.1;

        ctx.stroke_rect(x - width * 0.5, y - height * 0.5, width, height);
        ctx.fill_rect(x - width * 0.5, y - height * 0.5, width, height);

        ctx.translate(self.position.0, self.position.1).unwrap();
        ctx.scale(self.inner_scale, self.inner_scale).unwrap();

        self.circuit.draw(ctx);
    }
}

impl Component for Chip {
    fn as_chip(&self) -> Option<&Chip> {
        Some(&self)
    }

    fn contains(&self, viewport: &Viewport) -> bool {
        let contains_x =
            self.position.0 + self.size.0 * 0.5 >= viewport.position.0 + viewport.size.0 * 0.5 &&
            self.position.0 - self.size.0 * 0.5 <= viewport.position.0 - viewport.size.0 * 0.5;

        let contains_y =
            self.position.1 + self.size.1 * 0.5 >= viewport.position.1 + viewport.size.1 * 0.5 &&
            self.position.1 - self.size.1 * 0.5 <= viewport.position.1 - viewport.size.1 * 0.5;

        contains_x && contains_y
    }
}