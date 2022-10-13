import * as wasm from "atlas";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const renderer = new wasm.Renderer(ctx);

window.addEventListener("resize", () => {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	renderer.update_size();
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

	prevCursor = {
		x: e.clientX,
		y: e.clientY,
	};
});

window.addEventListener("wheel", (e) => {
	const zoom = 0.95 ** (-e.deltaY / 100);
	renderer.zoom(zoom, e.clientX, e.clientY);
});

let isZooming = false;

window.addEventListener("keypress", (e) => {
	if (e.key === 'v') {
		renderer.switch_viewport_mode();
	}
	if (e.key === 'z') {
		isZooming = !isZooming;
	}
});

const circuit = wasm.example1(10);
// const circuit = wasm.example2();

function render() {
	requestAnimationFrame(render);

	ctx.clearRect(0, 0, canvas.width, canvas.height);
	renderer.render(circuit);

	if (isZooming) {
		renderer.zoom(0.95, canvas.width / 2, canvas.height / 2);
	}
}

render();
