import type { Router } from '@/lib';
import { layoutGroup, withLayout } from '@/lib/layout';

const MainLayout = () => import('@/layout/MainLayout.svelte');
const AppLayout = () => import('@/layout/AppLayout.svelte');

export function registerApps(router: Router) {
	router.register(
		'/apps/create',
		withLayout(MainLayout, () => import('./Create.svelte')),
	);

	layoutGroup(router, /^\/apps\/(?<id>[a-zA-Z0-9_-]+)$/, AppLayout, r => {
		r.register('', () => import('./detail/Index.svelte'));
		r.register('/settings', () => import('./detail/Settings.svelte'));
		r.register('/registry', () => import('./detail/Registry.svelte'));
		r.register('/logs', () => import('./detail/Logs.svelte'));
		r.register(
			/\/files(?:\/(?<path>.*))?$/,
			() => import('./detail/Files.svelte'),
		);
	});
}
