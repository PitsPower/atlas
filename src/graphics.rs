use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::core::{Chip, Circuit, Component};

pub trait Drawable {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
    fn get_pin_positions(&self) -> Vec<(f64, f64)>;
}

pub enum WireLayoutCommand {
    AlignHorizontal,
    AlignVertical,
    CenterHorizontal,
    CenterVertical,
    MoveHorizontal(f64),
    MoveVertical(f64),
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub position: (f64, f64),
    pub size: (f64, f64),
}

impl Viewport {
    fn new(width: f64, height: f64) -> Self {
        Self {
            position: (0.0, 0.0),
            size: (width, height),
        }
    }

    fn scale(&self, ctx: &CanvasRenderingContext2d) -> f64 {
        self.size.0 / ctx.canvas().unwrap().width() as f64
    }

    fn transform_in_to_chip(&self, chip: &Chip) -> Viewport {
        let mut result = *self;

        let scale = chip.inner_scale;
    
        result.position.0 -= chip.position.0;
        result.position.1 -= chip.position.1;

        result.position.0 /= scale;
        result.position.1 /= scale;
        result.size.0 /= scale;
        result.size.1 /= scale;

        result
    }
    
    fn transform_out_of_chip(&self, chip: &Chip) -> Viewport {
        let mut result = *self;
        
        let scale = chip.inner_scale;

        result.position.0 *= scale;
        result.position.1 *= scale;
        result.size.0 *= scale;
        result.size.1 *= scale;

        result.position.0 += chip.position.0;
        result.position.1 += chip.position.1;
        
        result
    }
}

#[wasm_bindgen]
pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    viewport: Viewport,
    show_viewport: bool,
    chip_stack: Vec<usize>,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        let width = ctx.canvas().unwrap().width() as f64;
        let height = ctx.canvas().unwrap().height() as f64;

        Self {
            ctx,
            viewport: Viewport::new(
                width,
                height,
            ),
            show_viewport: false,
            chip_stack: vec![],
        }
    }

    fn get_canvas_size(&self) -> (f64, f64) {
        let width = self.ctx.canvas().unwrap().width() as f64;
        let height = self.ctx.canvas().unwrap().height() as f64;

        (width, height)
    }

    pub fn update_size(&mut self) {
        let (width, height) = self.get_canvas_size();

        self.viewport.size = (
            width * self.viewport.scale(&self.ctx),
            height * self.viewport.scale(&self.ctx),
        );
    }

    pub fn pan(&mut self, x_diff: f64, y_diff: f64) {
        self.viewport.position.0 -= x_diff * self.viewport.scale(&self.ctx);
        self.viewport.position.1 -= y_diff * self.viewport.scale(&self.ctx);
    }

    pub fn zoom(&mut self, zoom: f64, cursor_x: f64, cursor_y: f64) {
        let (width, height) = self.get_canvas_size();

        let cursor_vec = (
            cursor_x * self.viewport.size.0 / width - self.viewport.size.0 * 0.5,
            cursor_y * self.viewport.size.1 / height - self.viewport.size.1 * 0.5,
        );

        self.viewport.position.0 += cursor_vec.0 * (1.0 - zoom);
        self.viewport.position.1 += cursor_vec.1 * (1.0 - zoom);

        self.viewport.size.0 *= zoom;
        self.viewport.size.1 *= zoom;
    }

    pub fn switch_viewport_mode(&mut self) {
        self.show_viewport = !self.show_viewport;
    }

    fn get_current_circuit<'a>(&self, circuit: &'a Circuit) -> &'a Circuit {
        let mut result = circuit;

        for index in &self.chip_stack {
            result = &result.get_components()[*index].as_chip().unwrap().circuit;
        }
        
        result
    }

    fn get_parent_chip<'a>(&self, circuit: &'a Circuit) -> Option<&'a Chip> {
        if self.chip_stack.len() == 0 {
            return None;
        }

        let mut result = &circuit.get_components()[self.chip_stack[0]];

        for i in 1..self.chip_stack.len() {
            let index = self.chip_stack[i];
            result = &result.as_chip().unwrap().circuit.get_components()[index];
        }
        
        Some(result.as_chip().unwrap())
    }

    pub fn render(&mut self, root_circuit: &Circuit) {
        let ctx = &self.ctx;
        let mut circuit = self.get_current_circuit(root_circuit);

        for i in 0..circuit.get_components().len() {
            if circuit.get_components()[i].contains(&self.viewport) {
                let chip = circuit.get_components()[i].as_chip().unwrap();

                self.chip_stack.push(i);
                self.viewport = self.viewport.transform_in_to_chip(chip);
                circuit = self.get_current_circuit(root_circuit);

                break;
            }
        }

        while let Some(parent_chip) = self.get_parent_chip(root_circuit) {
            if !parent_chip.contains(&self.viewport.transform_out_of_chip(parent_chip)) {
                self.chip_stack.pop();
                self.viewport = self.viewport.transform_out_of_chip(parent_chip);
                circuit = self.get_current_circuit(root_circuit);
            } else {
                break;
            }
        }

        ctx.save();

        let width = ctx.canvas().unwrap().width() as f64;
        let height = ctx.canvas().unwrap().height() as f64;

        let scaled_width = self.viewport.size.0;
        let scaled_height = self.viewport.size.1;

        ctx.translate(
            width * 0.5,
            height * 0.5,
        ).unwrap();

        if !self.show_viewport {
            ctx.scale(1.0 / self.viewport.scale(&self.ctx), 1.0 / self.viewport.scale(&self.ctx)).unwrap();
            ctx.translate(
                -self.viewport.position.0,
                -self.viewport.position.1,
            ).unwrap();
        } else {
            // ctx.scale(0.3, 0.3).unwrap();
        }
        
        circuit.draw(ctx);

        if self.show_viewport {
            ctx.set_line_width(3.0);
            ctx.set_stroke_style(&"#ff0".into());

            ctx.stroke_rect(
                self.viewport.position.0 - scaled_width * 0.5,
                self.viewport.position.1 - scaled_height * 0.5,
                scaled_width,
                scaled_height
            );
        }

        ctx.restore();
    }
}