let disableCounter = 0;

export function disableScroll() {
	disableCounter += 1;
	update();
}

export function enableScroll() {
	disableCounter = Math.max(0, disableCounter - 1);
	update();
}

function update() {
	document.body.classList.toggle('disable-scroll', disableCounter > 0);
}
