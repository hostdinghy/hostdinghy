<script module lang="ts">
	import { loadServer } from '@/api/servers';

	export async function loadProps({ id }: { id: string }) {
		return {
			server: await loadServer(id),
		};
	}

	export type ServerLayoutProps<
		T extends (...args: any) => any = () => void,
	> = ResolvedProps<typeof loadProps> & ResolvedProps<T>;
</script>

<script lang="ts">
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
	import TabLayout from './TabLayout.svelte';
	import type { LayoutProps, ResolvedProps } from '@/lib/LoadProps';

	let { children, server }: LayoutProps<typeof loadProps> = $props();
</script>

<div class="main">
	<Header
		breadcrumbs={[
			{
				label: '🛶 HostDinghy',
				url: '/',
			},
			{
				label: server.name,
				url: `/servers/${server.id}`,
			},
		]}
	/>

	<div class="wrap">
		<TabLayout
			sidebar={[
				{
					label: 'Overview',
					url: `/servers/${server.id}`,
				},
				{
					label: 'Registry',
					url: `/servers/${server.id}/registry`,
				},
				{
					label: 'Postgres',
					url: `/servers/${server.id}/postgres`,
				},
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
