<script module lang="ts">
	import { App, byId } from '@/api/apps';

	export const layout = 'app';

	export async function loadProps({ id }) {
		return {
			app: await byId(id),
		};
	}
</script>

<script lang="ts">
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
	import TabLayout from './TabLayout.svelte';
	import type { LayoutProps } from '@/lib/LoadProps';

	let { children, app }: LayoutProps<typeof loadProps> = $props();
</script>

<div class="main">
	<Header
		breadcrumbs={[
			{
				label: '🛶 HostDinghy',
				url: '/',
			},
			{
				label: app.name,
				url: `/apps/${app.id}`,
			},
		]}
	/>

	<div class="wrap">
		<TabLayout
			sidebar={[
				{
					label: 'Overview',
					url: `/apps/${app.id}/`,
				},
				{
					label: 'Settings',
					url: `/apps/${app.id}/settings`,
				},
				{ label: 'Logs', url: `/apps/${app.id}/logs` },
			]}
		>
			{@render children()}
		</TabLayout>
	</div>
	<Footer />
</div>

<style lang="scss">
	.wrap {
		display: flex;
		margin-top: 2rem;
	}
	.main {
		display: grid;
		grid-template-rows: auto 1fr auto;
		min-height: 100vh;
	}
</style>
