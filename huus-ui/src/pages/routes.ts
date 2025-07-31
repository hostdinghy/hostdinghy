import type { Router } from 'chuchi';
import NotFound from './NotFound.svelte';
// import DefaultLayout from '../layout/EmptyLayout.svelte';
import { layoutGroup, withLayout } from '@/lib/layout';

export { NotFound };

export function register(router: Router) {
	router.register(
		'/',
		withLayout(
			() => import('../layout/MainLayout.svelte'),
			() => import('./Index.svelte'),
		),
	);
	router.register('/apps/create', () => import('./apps/Create.svelte'));

	layoutGroup(
		router,
		/^\/apps\/(?<id>[a-zA-Z0-9_-]+)$/,
		() => import('../layout/AppLayout.svelte'),
		r => {
			r.register('', () => import('./apps/detail/Index.svelte'));
			r.register('/logs', () => import('./apps/detail/Logs.svelte'));
			r.register(
				'/settings',
				() => import('./apps/detail/Settings.svelte'),
			);
		},
	);

	router.register('/signin', () => import('./SignIn.svelte'));
}
