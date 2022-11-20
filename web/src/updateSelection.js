const coordsEl = document.getElementById("coords");
const xCoordEl = document.getElementById("x-coord");
const yCoordEl = document.getElementById("y-coord");

coordsEl.style.visibility = "hidden";

export function updateSelection(hasSelection, x, y) {
	coordsEl.style.visibility = hasSelection ? "visible" : "hidden";

	if (hasSelection) {
		xCoordEl.value = x;
		yCoordEl.value = y;
	}
}