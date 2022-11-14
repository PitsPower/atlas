import * as wasm from "atlas";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const circuit = new wasm.Circuit();

const renderer = new wasm.Renderer(ctx);
renderer.update_sim_modes(circuit);

Object.values(wasm.ComponentType).forEach((ct) => {
	if (typeof ct === "string") {
		return;
	}

	const img = document.createElement("img");
	img.src = `./img/component_icons/${wasm.get_ct_slug(ct)}.png`;

	const text = document.createTextNode(wasm.get_ct_name(ct));

	const button = document.createElement("button");
	button.appendChild(img);
	button.appendChild(text);

	button.onclick = () => {
		const index = circuit.spawn_component(ct, renderer.get_viewport_x(), renderer.get_viewport_y());
		updateSelection([index]);
	};

	document.getElementById("spawn-buttons").appendChild(button);
});

window.addEventListener("resize", () => {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	renderer.update_size();
	renderer.update_sim_modes(circuit);
});

let isPanning = false;
let hasMoved = false;
let prevCursor = null;

let currentChipStack = [];
let prevInCircuitCursor = null;

let isDrawingWire = false;
let firstExternalPin = null;

let selectedChipStack = [];

document.getElementById("coords").style.visibility = "hidden";

function updateSelection(chipStack) {
	selectedChipStack = chipStack;

	document.getElementById("coords").style.visibility = chipStack.length === 0 ? "hidden" : "visible";
	document.getElementById("x-coord").value = circuit.get_x_from_chip_stack(chipStack);
	document.getElementById("y-coord").value = circuit.get_y_from_chip_stack(chipStack);
}

document.getElementById("coords").addEventListener("mousedown", (e) => {
	e.stopPropagation();
});
document.getElementById("coords").addEventListener("mouseup", (e) => {
	e.stopPropagation();
});
document.getElementById("x-coord").addEventListener("input", (e) => {
	const x = e.target.value;
	circuit.set_x_from_chip_stack(selectedChipStack, x);
});
document.getElementById("y-coord").addEventListener("input", (e) => {
	const y = e.target.value;
	circuit.set_y_from_chip_stack(selectedChipStack, y);
});

const keys = "asdfghjkzxcvbnm,";

window.addEventListener("keypress", (e) => {
	if (keys.includes(e.key)) {
		const index = keys.indexOf(e.key);
		circuit.toggle_switch(index);
	}

	switch (e.key) {
		case "p": {
			renderer.switch_viewport_mode();
			break;
		}

		case "w": {
			isDrawingWire = !isDrawingWire;
			renderer.switch_pin_mode();

			if (isDrawingWire) {
				document.getElementById("status-text").innerHTML = "Wire Mode";
			} else {
				document.getElementById("status-text").innerHTML = "";
			}

			break;
		}
	}
});

window.addEventListener("mousedown", (e) => {
	if (isDrawingWire) {
		const pin = renderer.get_clicked_pin(circuit, e.clientX, e.clientY);

		if (pin) {
			firstExternalPin = pin;
		}

		isPanning = !pin;
	} else {
		currentChipStack = Array.from(renderer.get_chip_stack_from_pos(circuit, e.clientX, e.clientY));
		isPanning = currentChipStack.length === 0;
	}

	prevCursor = {
		x: e.clientX,
		y: e.clientY,
	};
	
	const cursor = renderer.get_cursor_from_pos(circuit, currentChipStack, e.clientX, e.clientY);
	prevInCircuitCursor = {
		x: cursor.get_x(),
		y: cursor.get_y(),
	};
});

window.addEventListener("mouseup", (e) => {
	isPanning = false;

	if (!hasMoved) {
		if (isDrawingWire) {
			const pin = renderer.get_clicked_pin(circuit, e.clientX, e.clientY);

			if (!pin) {
				return;
			}

			if (!firstExternalPin) {
				firstExternalPin = pin;
				return;
			}

			circuit.connect_external(
				firstExternalPin.component_idx, firstExternalPin.pin_idx,
				pin.component_idx, pin.pin_idx,
			);

			firstExternalPin = null;
		} else {
			const chipStack = Array.from(renderer.get_chip_stack_from_pos(circuit, e.clientX, e.clientY));

			if (
				chipStack.length > 0 &&
				chipStack.length === selectedChipStack.length &&
				selectedChipStack.every((v, i) => chipStack[i] === v)
			) {
				circuit.toggle_switch_from_chip_stack(chipStack);
			}

			updateSelection(chipStack);
		}
	}

	hasMoved = false;

	currentChipStack = [];
});

window.addEventListener("mousemove", (e) => {
	if (!isPanning) {
		if (currentChipStack.length === 0) {
			return;
		}

		const cursor = renderer.get_cursor_from_pos(circuit, currentChipStack, e.clientX, e.clientY);
		const inCircuitCursor = {
			x: cursor.get_x(),
			y: cursor.get_y(),
		};

		circuit.translate_component_from_chip_stack(
			currentChipStack,
			e.ctrlKey ? 0 : inCircuitCursor.x - prevInCircuitCursor.x,
			e.shiftKey ? 0 : inCircuitCursor.y - prevInCircuitCursor.y,	
		);
		
		document.getElementById("x-coord").value = circuit.get_x_from_chip_stack(currentChipStack);
		document.getElementById("y-coord").value = circuit.get_y_from_chip_stack(currentChipStack);

		prevInCircuitCursor = inCircuitCursor;
		hasMoved = true;
		
		return;
	}

	hasMoved = true;

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

function render() {
	requestAnimationFrame(render);

	ctx.clearRect(0, 0, canvas.width, canvas.height);
	renderer.render(circuit);
}

render();
