import { Writable } from 'chuchi/stores';
import { derived } from 'svelte/store';

const THEME_MODE_KEY = 'dinghy-theme-mode';

type Mode = 'system' | 'light' | 'dark';

export const userPrefersMode = createUserPrefersMode();
export const systemPrefersMode = createSystemPrefersMode();
export const derivedMode = createDerivedMode();

function createUserPrefersMode() {
	let mode = (localStorage.getItem(THEME_MODE_KEY) as Mode) ?? 'system';

	const store = new Writable<Mode>(mode);

	store.subscribe(mode => {
		localStorage.setItem(THEME_MODE_KEY, mode);
	});

	return store;
}

function createSystemPrefersMode() {
	const mediaQueryState = window.matchMedia('(prefers-color-scheme: light)');

	const store = new Writable(mediaQueryState.matches ? 'light' : 'dark');

	const handler = (e: MediaQueryListEvent) => {
		store.set(e.matches ? 'light' : 'dark');
	};

	mediaQueryState.addEventListener('change', handler);
	// todo: remove Eventlistener on store destroy, missing store api

	function query() {
		store.set(mediaQueryState.matches ? 'light' : 'dark');
	}

	return {
		store,
		query,
	};
}

function createDerivedMode() {
	const derivedMode = derived(
		[userPrefersMode, systemPrefersMode.store],
		([$userPrefersMode, $systemPrefersMode]) => {
			return $userPrefersMode === 'system'
				? $systemPrefersMode
				: $userPrefersMode;
		},
	);

	derivedMode.subscribe(mode => {
		document.documentElement.classList.remove('light');
		document.documentElement.classList.remove('dark');
		document.documentElement.classList.add(mode);
		document.documentElement.style.colorScheme = mode;
	});

	return derivedMode;
}
