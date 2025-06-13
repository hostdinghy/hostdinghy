import { resolve } from 'path';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { sveltePreprocess } from 'svelte-preprocess';

// https://vitejs.dev/config/
export default defineConfig(({ isSsrBuild }) => {
	return {
		publicDir: isSsrBuild ? false : undefined,
		ssr: {
			noExternal: isSsrBuild ? true : ['chuchi'],
		},
		plugins: [
			svelte({
				compilerOptions: {
					runes: true,
				},
				preprocess: [sveltePreprocess()],
			}),
		],
		resolve: {
			alias: [{ find: '@', replacement: resolve(__dirname, 'src') }],
		},
	};
});
