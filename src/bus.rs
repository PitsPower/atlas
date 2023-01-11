//! Provides data structures and algorithms for laying out buses (groups of wires)

use crate::graphics::WireLayoutCommand;

/// A command used to control how a bus is layed out.
#[derive(Clone, Copy)]
pub enum BusLayoutCommand {
	/// Moves the bus up or down to be aligned with the end pins.
	AlignHorizontal,
	/// Moves the bus left or right to be aligned with the end pins.
	AlignVertical,
	/// Moves the bus in-between the start and end pins horizontally.
	CenterHorizontal,
	/// Moves the bus in-between the start and end pins vertically.
	CenterVertical,
	/// Moves the bus horizontally.
	MoveHorizontal(f64),
	/// Moves the bus vertically.
	MoveVertical(f64),
	/// Moves the bus horizontally and vertically at the same time.
	Move((f64, f64)),
	/// Moves the x coordinate to an absolute location.
	MoveXTo(f64),
	/// Moves the y coordinate to an absolute location.
	MoveYTo(f64),
	/// Moves the bus to an absolute location.
	MoveTo((f64, f64)),
}

/// Computes the wire layouts given the bus layout and the positions of the pins.
pub fn compute_wire_commands(
	bus_commands: &[BusLayoutCommand], start_positions: &[(f64, f64)], end_positions: &[(f64, f64)],
) -> Vec<Vec<WireLayoutCommand>> {
	// The starting position of the bus is the average of
	// the middle two pin starting positions

	let size = start_positions.len();
	let mid = size / 2;

	let mut bus_pos = (0.0, 0.0);

	if size % 2 == 0 {
		let p1 = start_positions[mid - 1];
		let p2 = start_positions[mid];
		bus_pos.0 = (p1.0 + p2.0) * 0.5;
		bus_pos.1 = (p1.1 + p2.1) * 0.5;
	} else {
		bus_pos = start_positions[mid];
	}

	let end_size = end_positions.len();
	let end_mid = end_size / 2;

	let mut end_bus_pos = (0.0, 0.0);

	if end_size % 2 == 0 {
		let p1 = end_positions[end_mid - 1];
		let p2 = end_positions[end_mid];
		end_bus_pos.0 = (p1.0 + p2.0) * 0.5;
		end_bus_pos.1 = (p1.1 + p2.1) * 0.5;
	} else {
		end_bus_pos = end_positions[end_mid];
	}

	// The offsets of each wire from the bus line
	let mut offsets = vec![0.0; size];

	let mut prev_parallel = (0.0, 0.0);
	let mut prev_perpendicular = (0.0, 0.0);

	let mut result: Vec<Vec<WireLayoutCommand>> = vec![vec![]; size];

	for (cidx, command) in bus_commands.iter().enumerate() {
		let mut new_bus_pos = bus_pos;

		match command {
			BusLayoutCommand::AlignHorizontal => {
				new_bus_pos.1 = end_bus_pos.1;
			},
			BusLayoutCommand::AlignVertical => {
				new_bus_pos.0 = end_bus_pos.0;
			},
			BusLayoutCommand::CenterHorizontal => {
				new_bus_pos.0 = (new_bus_pos.0 + end_bus_pos.0) * 0.5;
			},
			BusLayoutCommand::CenterVertical => {
				new_bus_pos.1 = (new_bus_pos.1 + end_bus_pos.1) * 0.5;
			},
			BusLayoutCommand::MoveHorizontal(x) => {
				new_bus_pos.0 += x;
			},
			BusLayoutCommand::MoveVertical(y) => {
				new_bus_pos.1 += y;
			},
			BusLayoutCommand::Move((x, y)) => {
				new_bus_pos.0 += x;
				new_bus_pos.1 += y;
			},
			BusLayoutCommand::MoveXTo(x) => {
				new_bus_pos.0 = *x;
			},
			BusLayoutCommand::MoveYTo(y) => {
				new_bus_pos.1 = *y;
			},
			BusLayoutCommand::MoveTo((x, y)) => {
				new_bus_pos = (*x, *y);
			},
		}

		let (x, y) = new_bus_pos;
		let diff = (new_bus_pos.0 - bus_pos.0, new_bus_pos.1 - bus_pos.1);

		// Fixes NaN bug
		if diff != (0.0, 0.0) {
			let parallel_len = (diff.0 * diff.0 + diff.1 * diff.1).sqrt();
			let parallel = (diff.0 / parallel_len, diff.1 / parallel_len);
			let perpendicular = (-parallel.1, parallel.0);
	
			if cidx == 0 {
				// Calculate offsets on the first bus line segment
				for i in 0..size {
					let pin_offset = (start_positions[i].0 - bus_pos.0, start_positions[i].1 - bus_pos.1);
					offsets[i] = pin_offset.0 * perpendicular.0 + pin_offset.1 * perpendicular.1;
				}
			} else {
				// For the rest, move the wires so that the offset is maintained
				for i in 0..size {
					let current_pos = (
						bus_pos.0 + prev_perpendicular.0 * offsets[i],
						bus_pos.1 + prev_perpendicular.1 * offsets[i],
					);
	
					let mut parallel_offset = offsets[i] + (x - current_pos.0) * perpendicular.0 + (y - current_pos.1) * perpendicular.1;
					
					// Fixes NaN bug
					if parallel_offset != 0.0 {
						parallel_offset /= perpendicular.0 * prev_parallel.0 + perpendicular.1 * prev_parallel.1;
					}
					
					result[i].push(WireLayoutCommand::DontRenderPrevious);
					result[i].push(WireLayoutCommand::Move((
						prev_parallel.0 * parallel_offset,
						prev_parallel.1 * parallel_offset,
					)));
				}
			}
	
			for (widx, wire) in result.iter_mut().enumerate() {
				wire.push(WireLayoutCommand::MoveTo((
					x + perpendicular.0 * offsets[widx],
					y + perpendicular.1 * offsets[widx],
				)));
			}
	
			bus_pos = new_bus_pos;
			prev_parallel = parallel;
			prev_perpendicular = perpendicular;
		}

		match command {
			BusLayoutCommand::AlignHorizontal => {
				for wire in &mut result {
					wire.push(WireLayoutCommand::DontRenderPreviousVertical);
					wire.push(WireLayoutCommand::AlignHorizontal);
				}
			},
			BusLayoutCommand::AlignVertical => {
				for wire in &mut result {
					wire.push(WireLayoutCommand::DontRenderPreviousHorizontal);
					wire.push(WireLayoutCommand::AlignVertical);
				}
			},
			_ => {},
		}
	}

	result
}
