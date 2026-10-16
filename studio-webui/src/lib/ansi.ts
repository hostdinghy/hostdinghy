export type AnsiSegment = {
	text: string;
	/** inline css, empty if unstyled */
	style: string;
};

// the 16 standard colors, readable on light and dark backgrounds
const COLORS = [
	'#7f7f7f',
	'#d6453d',
	'#3f9e3f',
	'#b8860b',
	'#3b78d8',
	'#b04fc0',
	'#2a9fae',
	'#a0a0a0',
	'#8c8c8c',
	'#ef5f57',
	'#4fb84f',
	'#d4a017',
	'#5a92f0',
	'#c86fd6',
	'#3cbccc',
	'#c0c0c0',
];

type State = {
	fg: string | null;
	bg: string | null;
	bold: boolean;
	dim: boolean;
	italic: boolean;
	underline: boolean;
};

function color256(n: number): string | null {
	if (n < 16) return COLORS[n];
	if (n < 232) {
		n -= 16;
		const c = (v: number) => (v === 0 ? 0 : 55 + v * 40);
		return `rgb(${c(Math.floor(n / 36))}, ${c(Math.floor(n / 6) % 6)}, ${c(n % 6)})`;
	}
	if (n < 256) {
		const v = 8 + (n - 232) * 10;
		return `rgb(${v}, ${v}, ${v})`;
	}
	return null;
}

function applySgr(state: State, params: number[]) {
	for (let i = 0; i < params.length; i++) {
		const p = params[i];
		if (p === 0) {
			Object.assign(state, {
				fg: null,
				bg: null,
				bold: false,
				dim: false,
				italic: false,
				underline: false,
			});
		} else if (p === 1) state.bold = true;
		else if (p === 2) state.dim = true;
		else if (p === 3) state.italic = true;
		else if (p === 4) state.underline = true;
		else if (p === 22) state.bold = state.dim = false;
		else if (p === 23) state.italic = false;
		else if (p === 24) state.underline = false;
		else if (p >= 30 && p <= 37) state.fg = COLORS[p - 30];
		else if (p === 39) state.fg = null;
		else if (p >= 40 && p <= 47) state.bg = COLORS[p - 40];
		else if (p === 49) state.bg = null;
		else if (p >= 90 && p <= 97) state.fg = COLORS[p - 90 + 8];
		else if (p >= 100 && p <= 107) state.bg = COLORS[p - 100 + 8];
		else if (p === 38 || p === 48) {
			// 38;5;n or 38;2;r;g;b
			let color: string | null = null;
			if (params[i + 1] === 5) {
				color = color256(params[i + 2]);
				i += 2;
			} else if (params[i + 1] === 2) {
				const [r, g, b] = params.slice(i + 2, i + 5);
				color = `rgb(${r}, ${g}, ${b})`;
				i += 4;
			}
			if (p === 38) state.fg = color;
			else state.bg = color;
		}
	}
}

function toCss(state: State): string {
	let css = '';
	if (state.fg) css += `color: ${state.fg};`;
	if (state.bg) css += `background-color: ${state.bg};`;
	if (state.bold) css += 'font-weight: bold;';
	if (state.dim) css += 'opacity: 0.7;';
	if (state.italic) css += 'font-style: italic;';
	if (state.underline) css += 'text-decoration: underline;';
	return css;
}

// SGR sequences, other CSI sequences, OSC sequences and lone escapes
const ESCAPES =
	// eslint-disable-next-line no-control-regex
	/\x1b\[([0-9;]*)m|\x1b\[[0-9;?]*[A-Za-z]|\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)|\x1b/g;

/**
 * Splits a line with ansi escape codes into styled segments. Colors and
 * text styles are kept, other escape codes (cursor movement...) dropped.
 */
export function parseAnsi(line: string): AnsiSegment[] {
	const segments: AnsiSegment[] = [];
	const state: State = {
		fg: null,
		bg: null,
		bold: false,
		dim: false,
		italic: false,
		underline: false,
	};
	let last = 0;

	const push = (text: string) => {
		if (text) segments.push({ text, style: toCss(state) });
	};

	for (const m of line.matchAll(ESCAPES)) {
		push(line.slice(last, m.index));
		last = m.index! + m[0].length;

		if (m[1] !== undefined) {
			const params = m[1] === '' ? [0] : m[1].split(';').map(Number);
			applySgr(state, params);
		}
	}
	push(line.slice(last));

	return segments;
}
