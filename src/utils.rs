//! Various utility functions.

use crate::core::PinState;

/// Used for better error messages.
pub fn set_panic_hook() {
	// When the `console_error_panic_hook` feature is enabled, we can call the
	// `set_panic_hook` function at least once during initialization, and then
	// we will get better error messages if our code ever panics.
	//
	// For more details see
	// https://github.com/rustwasm/console_error_panic_hook#readme
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

/// Converts a list of pin states into a number by interpreting the states as a binary value.
/// [`PinState::Disconnected`] and [`PinState::Off`] are treated as 0 and [`PinState::On`] is treated as 1.
/// The first state in the list is treated as the most significant bit.
pub fn states_to_num(states: &Vec<PinState>) -> u32 {
	let mut result = 0;

	for state in states {
		result *= 2;
		if *state == PinState::On {
			result += 1;
		}
	}

	result
}

/// Convert a number into a list of pin states where each pin state is a binary bit in the number.
/// The first state in the list is treated as the most significant bit.
pub fn num_to_states(num: u32, amount: usize) -> Vec<PinState> {
	let mut states = vec![];
	let mut current = num;

	while current != 0 {
		states.insert(0, if current % 2 == 1 { PinState::On } else { PinState::Off });
		current /= 2;
	}

	let mut result = vec![PinState::Off; amount - states.len()];
	result.append(&mut states);
	result
}

/// Returns the coordinates of pins given the centre of the group of pins,
/// the number of pins, and the desired spacing between them. 
pub fn get_pin_coords(center: f64, pin_amount: usize, spacing: f64) -> Vec<f64> {
	let first_pin_coord = center - (pin_amount - 1) as f64 * 0.5 * spacing;
	(0..pin_amount).map(|i| first_pin_coord + i as f64 * spacing).collect()
}
