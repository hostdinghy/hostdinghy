import NotFound from './NotFound.svelte';
import Error from './Error.svelte';
import { withLayout } from '@/lib/layout';
import type { Router } from '@/lib';
import { registerApps } from './apps/routes';
import { registerSettings } from './settings/routes';
import { registerServers } from './servers/routes';

export { NotFound, Error };

const MainLayout = () => import('../layout/MainLayout.svelte');

export function register(router: Router) {
	router.register(
		'/',
		withLayout(MainLayout, () => import('./Index.svelte')),
	);
	router.register('/signin', () => import('./SignIn.svelte'));

	registerApps(router);
	registerSettings(router);
	registerServers(router);
}
