import type { Router } from 'chuchi';
import NotFound from './NotFound.svelte';

export { NotFound };

export function register(router: Router) {
	router.register('/', () => import('./Index.svelte'));
	router.register('/apps/create', () => import('./apps/Create.svelte'));
	router.register(
		/^\/apps\/(?<id>[a-zA-Z0-9_-]+)$/,
		() => import('./apps/Detail.svelte'),
	);
	router.register('/signin', () => import('./SignIn.svelte'));
}
