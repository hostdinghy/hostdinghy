import type { Router } from 'chuchi';
import NotFound from './NotFound.svelte';

export { NotFound };

export function register(router: Router) {
	router.register('/', () => import('./Index.svelte'));
	router.register('/signin', () => import('./SignIn.svelte'));
}
