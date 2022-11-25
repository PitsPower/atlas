import * as wasm from "atlas";

const spawnButtonsEl = document.getElementById("spawn-buttons");
const coordsEl = document.getElementById("coords");
const xCoordEl = document.getElementById("x-coord");
const yCoordEl = document.getElementById("y-coord");

const codeViewerEl = document.getElementById("code-viewer");
const codeEl = document.getElementById("code");

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const editor = new wasm.Editor(ctx);

// Add the spawn buttons
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
		editor.spawn_component(ct);
	};

	spawnButtonsEl.appendChild(button);
});

window.addEventListener("resize", () => {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	editor.update_size();
});

coordsEl.addEventListener("mousedown", (e) => {
	e.stopPropagation();
});
coordsEl.addEventListener("mouseup", (e) => {
	e.stopPropagation();
});
xCoordEl.addEventListener("input", (e) => {
	const x = e.target.value;
	editor.set_selected_x(x);
});
yCoordEl.addEventListener("input", (e) => {
	const y = e.target.value;
	editor.set_selected_y(y);
});

codeViewerEl.style.visibility = "hidden";

// const keys = "asdfghjkzxcvbnm,";

window.addEventListener("keydown", (e) => {
	// if (keys.includes(e.key)) {
	// 	const index = keys.indexOf(e.key);
	// 	editor.toggle_switch(index);
	// }

	switch (e.key.toLowerCase()) {
		case "w": {
			editor.toggle_wire_mode();
			break;
		}
		case "c": {
			if (!e.ctrlKey) {
				if (codeViewerEl.style.visibility === "visible") {
					codeViewerEl.style.visibility = "hidden";
				} else {
					const code = editor.generate_code();
					navigator.clipboard.writeText(code);
					
					codeEl.innerText = code;
					codeViewerEl.style.visibility = "visible";
				}
			}
			break;
		}

		case "delete": {
			editor.delete_selected();
			break;
		}

		case "h": {
			if (e.ctrlKey) {
				e.preventDefault();
				editor.wire_center_horizontal();
			} else {
				editor.wire_align_horizontal();
			}

			break;
		}
		case "v": {
			if (e.ctrlKey) {
				e.preventDefault();
				editor.wire_center_vertical();
			} else {
				editor.wire_align_vertical();
			}

			break;
		}

		case "enter": {
			editor.handle_confirm();
			break;
		}

		default: {
			console.log("Unhandled key press:", e.key);
			break;
		}
	}
});

window.addEventListener("mousedown", (e) => {
	editor.handle_mouse_down(e.clientX, e.clientY);
});
window.addEventListener("mouseup", (e) => {
	editor.handle_mouse_up(e.clientX, e.clientY);
});
window.addEventListener("mousemove", (e) => {
	editor.handle_mouse_move(e.clientX, e.clientY, e.ctrlKey, e.shiftKey, e.altKey);
});

window.addEventListener("wheel", (e) => {
	if (codeViewerEl.style.visibility === "hidden") {
		const zoom = 0.95 ** (-e.deltaY / 100);
		editor.zoom(zoom, e.clientX, e.clientY);
	}
});

function render() {
	requestAnimationFrame(render);

	ctx.clearRect(0, 0, canvas.width, canvas.height);
	editor.render();
}

render();
