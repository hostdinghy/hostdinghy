import { sveltePreprocess } from 'svelte-preprocess';

/** @type {import('@sveltejs/vite-plugin-svelte').SvelteConfig} */
const config = {
	preprocess: [sveltePreprocess()],
	compilerOptions: {
		runes: true,
	},
};

export default config;
