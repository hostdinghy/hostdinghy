<script module lang="ts">
	import { byId } from '@/api/apps';

	export async function loadProps({ id }: { id: string }) {
		return {
			app: await byId(id),
		};
	}

	export type AppLayoutProps<T extends (...args: any) => any = () => void> =
		ResolvedProps<typeof loadProps> & ResolvedProps<T>;
</script>

<script lang="ts">
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
	import TabLayout from './TabLayout.svelte';
	import type { LayoutProps, ResolvedProps } from '@/lib/LoadProps';

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
					url: `/apps/${app.id}`,
				},
				{
					label: 'Settings',
					url: `/apps/${app.id}/settings`,
				},
				{
					label: 'Registry',
					url: `/apps/${app.id}/registry`,
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
	.main {
		display: grid;
		grid-template-rows: auto 1fr auto;
		min-height: 100vh;
	}

	.wrap {
		display: flex;
		margin-top: 2rem;
	}
</style>
