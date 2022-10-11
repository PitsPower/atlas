#[allow(dead_code)]

mod utils;

use std::f64;

use wasm_bindgen::prelude::*;
use web_sys::*;

use utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! console_log {
    ($($arg:tt)*) => {
        console::log_1(&format!($($arg)*).into());
    };
}

trait Drawable {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
}

trait Component: Drawable {
    fn as_chip(&self) -> Option<&Chip> {
        None
    }

    fn contains(&self, viewport: &Viewport) -> bool;
    fn intersects(&self, viewport: &Viewport) -> bool;
}

struct Chip {
    circuit: Circuit,

    position: (f64, f64),
    size: (f64, f64),
    inner_scale: f64,
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

    fn intersects(&self, viewport: &Viewport) -> bool {
        let intersects_x =
            self.position.0 + self.size.0 * 0.5 >= viewport.position.0 - viewport.size.0 * 0.5 &&
            self.position.0 - self.size.0 * 0.5 <= viewport.position.0 + viewport.size.0 * 0.5;

        let intersects_y =
            self.position.1 + self.size.1 * 0.5 >= viewport.position.1 - viewport.size.1 * 0.5 &&
            self.position.1 - self.size.1 * 0.5 <= viewport.position.1 + viewport.size.1 * 0.5; 

        intersects_x && intersects_y
    }
}

#[wasm_bindgen]
pub struct Circuit {
    components: Vec<Box<dyn Component>>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            components: vec![],
        }
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

#[wasm_bindgen]
pub fn create_test_circuit(n: u32) -> Circuit {
    let mut result = Circuit::new();
    
    if n > 0 {
        let chip1 = Chip {
            circuit: create_test_circuit(n-1),
    
            position: (-180.0, 0.0),
            size: (300.0, 300.0),
            inner_scale: 0.4,
        };
        let chip2 = Chip {
            circuit: create_test_circuit(n-1),
    
            position: (180.0, 0.0),
            size: (300.0, 300.0),
            inner_scale: 0.4,
        };

        result.components.push(Box::new(chip1));
        result.components.push(Box::new(chip2));
    }

    result
}

#[derive(Debug, Clone, Copy)]
struct Viewport {
    position: (f64, f64),
    size: (f64, f64),
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
        self.viewport.size.0 *= zoom;
        self.viewport.size.1 *= zoom;
        
        let (width, height) = self.get_canvas_size();

        let cursor_vec = (
            cursor_x * self.viewport.size.0 / width - self.viewport.size.0 * 0.5,
            cursor_y * self.viewport.size.1 / height - self.viewport.size.1 * 0.5,
        );

        self.viewport.position.0 += cursor_vec.0 - cursor_vec.0 * zoom;
        self.viewport.position.1 += cursor_vec.1 - cursor_vec.1 * zoom;
    }

    pub fn switch_viewport_mode(&mut self) {
        self.show_viewport = !self.show_viewport;
    }

    fn get_current_circuit<'a>(&self, circuit: &'a Circuit) -> &'a Circuit {
        let mut result = circuit;

        for index in &self.chip_stack {
            result = &result.components[*index].as_chip().unwrap().circuit;
        }
        
        result
    }

    fn get_parent_chip<'a>(&self, circuit: &'a Circuit) -> Option<&'a Chip> {
        if self.chip_stack.len() == 0 {
            return None;
        }

        let mut result = &circuit.components[self.chip_stack[0]];

        for i in 1..self.chip_stack.len() {
            let index = self.chip_stack[i];
            result = &result.as_chip().unwrap().circuit.components[index];
        }
        
        Some(result.as_chip().unwrap())
    }

    pub fn render(&mut self, root_circuit: &Circuit) {
        let ctx = &self.ctx;
        let mut circuit = self.get_current_circuit(root_circuit);

        for i in 0..circuit.components.len() {
            if circuit.components[i].contains(&self.viewport) {
                let chip = circuit.components[i].as_chip().unwrap();

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
            ctx.scale(0.3, 0.3).unwrap();
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

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("Started!");
    set_panic_hook();
}
