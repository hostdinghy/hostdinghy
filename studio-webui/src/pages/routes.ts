import type { Router } from 'chuchi';
import NotFound from './NotFound.svelte';
// import DefaultLayout from '../layout/EmptyLayout.svelte';
import { layoutGroup, withLayout } from '@/lib/layout';

export { NotFound };

const MainLayout = () => import('../layout/MainLayout.svelte');

export function register(router: Router) {
	router.register(
		'/',
		withLayout(MainLayout, () => import('./Index.svelte')),
	);
	router.register(
		'/apps/create',
		withLayout(MainLayout, () => import('./apps/Create.svelte')),
	);

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

	layoutGroup(
		router,
		'/settings',
		() => import('../layout/SettingsLayout.svelte'),
		r => {
			r.register('/account', () => import('./settings/Account.svelte'));
			r.register(
				'/appearance',
				() => import('./settings/Appearance.svelte'),
			);
			r.register(
				'/servers',
				() => import('./settings/servers/Index.svelte'),
			);
		},
	);
	router.register(
		'/settings/servers/create',
		withLayout(
			MainLayout,
			() => import('./settings/servers/Create.svelte'),
		),
	);

	router.register('/signin', () => import('./SignIn.svelte'));
}
