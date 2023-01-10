//! Various utility functions.

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

/// Returns the coordinates of pins given the centre of the group of pins,
/// the number of pins, and the desired spacing between them. 
pub fn get_pin_coords(center: f64, pin_amount: usize, spacing: f64) -> Vec<f64> {
	let first_pin_coord = center - (pin_amount - 1) as f64 * 0.5 * spacing;
	(0..pin_amount).map(|i| first_pin_coord + i as f64 * spacing).collect()
}
