import NotFound from './NotFound.svelte';
import Error from './Error.svelte';
import { layoutGroup, withLayout } from '@/lib/layout';
import type { Router } from '@/lib';

export { NotFound, Error };

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
			r.register(
				'/settings',
				() => import('./apps/detail/Settings.svelte'),
			);
			r.register(
				'/registry',
				() => import('./apps/detail/Registry.svelte'),
			);
			r.register('/logs', () => import('./apps/detail/Logs.svelte'));
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
