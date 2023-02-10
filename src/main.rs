use atlas::*;

fn main() {
	let mut circuit = get_computer_circuit();

	loop {
		circuit.toggle_switch(0);
	}
}
