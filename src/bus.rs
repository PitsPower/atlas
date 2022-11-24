//! Provides data structures and algorithms for laying out buses (groups of wires)

use crate::graphics::WireLayoutCommand;

/// A command used to control how a bus is layed out.
pub enum BusLayoutCommand {
	/// Moves the bus to an absolute location.
	MoveTo((f64, f64)),
}

/// Computes the wire layouts given the bus layout and the positions of the starting pins.
pub fn compute_wire_commands(
	bus_commands: Vec<BusLayoutCommand>, start_positions: Vec<(f64, f64)>
) -> Vec<Vec<WireLayoutCommand>> {
	let mut result = vec![vec![]; start_positions.len()];

	for command in bus_commands {
		match command {
			BusLayoutCommand::MoveTo((x, y)) => {
				for wire in &mut result {
					wire.push(WireLayoutCommand::MoveTo((x, y)));
				}
			},
		}
	}

	result
}
