use atlas::add;
use atlas::core::*;
use atlas::graphics::*;

fn main() {
	let mut circuit = Circuit::new();
	add!(circuit, Memory, (0.000, 0.000), 16);
	update_sim_modes_with_viewport(&mut circuit, BoundingBox::new(1920.0, 1080.0))
}
