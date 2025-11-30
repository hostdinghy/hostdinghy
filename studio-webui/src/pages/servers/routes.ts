import type { Router } from '@/lib';
import { layoutGroup, withLayout } from '@/lib/layout';

const MainLayout = () => import('@/layout/MainLayout.svelte');
const SettingsLayout = () => import('@/layout/SettingsLayout.svelte');
const ServerLayout = () => import('@/layout/ServerLayout.svelte');

export function registerServers(router: Router) {
	router.register(
		'/servers',
		withLayout(SettingsLayout, () => import('./Index.svelte')),
	);
	router.register(
		'/servers/create',
		withLayout(MainLayout, () => import('./Create.svelte')),
	);
	layoutGroup(
		router,
		/^\/servers\/(?<id>[a-zA-Z0-9_-]+)$/,
		ServerLayout,
		r => {
			r.register('', () => import('./detail/Index.svelte'));
			r.register('/registry', () => import('./detail/Registry.svelte'));
		},
	);
}
