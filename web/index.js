import * as wasm from "atlas";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

// const circuit = wasm.example1(10);
// const circuit = wasm.example2();
// const circuit = wasm.transistor_example();
// const circuit = wasm.transistor_example2();
// const circuit = wasm.bidirectional_example();
// const circuit = wasm.not_gate_example();
// const circuit = wasm.nor_gate_example();
// const circuit = wasm.or_gate_example();
// const circuit = wasm.nand_gate_example();
// const circuit = wasm.and_gate_example();
// const circuit = wasm.xor_gate_example();
// const circuit = wasm.nor_latch_example();
// const circuit = wasm.test_example();
// const circuit = wasm.bus_example();
const circuit = wasm.latch_example();

const renderer = new wasm.Renderer(ctx);
renderer.update_sim_modes(circuit);

window.addEventListener("resize", () => {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	renderer.update_size();
	renderer.update_sim_modes(circuit);
});

let isPanning = false;
let prevCursor = null;

window.addEventListener("mousedown", (e) => {
	isPanning = true;

	prevCursor = {
		x: e.clientX,
		y: e.clientY,
	};
});

window.addEventListener("mouseup", () => {
	isPanning = false;
});

window.addEventListener("mousemove", (e) => {
	if (!isPanning) {
		return;
	}

	const xDiff = e.clientX - prevCursor.x;
	const yDiff = e.clientY - prevCursor.y;

	renderer.pan(xDiff, yDiff);
	renderer.update_sim_modes(circuit);

	prevCursor = {
		x: e.clientX,
		y: e.clientY,
	};
});

window.addEventListener("wheel", (e) => {
	const zoom = 0.95 ** (-e.deltaY / 100);
	renderer.zoom(zoom, e.clientX, e.clientY);
	renderer.update_sim_modes(circuit);
});

const keys = 'asdfghjkzxcvbnm,';

window.addEventListener("keypress", (e) => {
	if (keys.includes(e.key)) {
		const index = keys.indexOf(e.key);
		circuit.toggle_switch(index);
	}

	switch (e.key) {
		case 'p': {
			renderer.switch_viewport_mode();
			break;
		}
	}
});

function render() {
	requestAnimationFrame(render);

	ctx.clearRect(0, 0, canvas.width, canvas.height);
	renderer.render(circuit);
}

render();
