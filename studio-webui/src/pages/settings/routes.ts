import type { Router } from '@/lib';
import { layoutGroup } from '@/lib/layout';

const SettingsLayout = () => import('@/layout/SettingsLayout.svelte');

export function registerSettings(router: Router) {
	layoutGroup(router, '/settings', SettingsLayout, r => {
		r.register('/account', () => import('./Account.svelte'));
		r.register('/appearance', () => import('./Appearance.svelte'));
	});
}
