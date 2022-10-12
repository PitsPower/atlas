#[allow(dead_code)]

mod core;
mod graphics;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::*;

use utils::set_panic_hook;

use crate::core::{Chip, Circuit};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ($($arg:tt)*) => {
        console::log_1(&format!($($arg)*).into());
    };
}

#[wasm_bindgen]
pub fn create_test_circuit(n: u32) -> Circuit {
    fn iter(n: u32) -> Circuit {
        let mut result = Circuit::new();
        
        if n > 0 {
            let chip1 = Chip {
                circuit: iter(n-1),
        
                position: (-180.0, 0.0),
                size: (300.0, 300.0),
                inner_scale: 0.4,
            };
            let chip2 = Chip {
                circuit: iter(n-1),
        
                position: (180.0, 0.0),
                size: (300.0, 300.0),
                inner_scale: 0.4,
            };

            result.add(Box::new(chip1));
            result.add(Box::new(chip2));
        }

        result
    }

    let chip = Chip {
        circuit: iter(n),

        position: (0.0, 10.0),
        size: (850.0, 500.0),
        inner_scale: 1.0,
    };

    let mut circuit = Circuit::new();
    circuit.add(Box::new(chip));
    circuit
}

#[wasm_bindgen(start)]
pub fn start() {
    log!("Stuff has started!");
    set_panic_hook();
}
