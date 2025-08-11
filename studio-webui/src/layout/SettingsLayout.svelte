<script lang="ts">
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
	import TabLayout from './TabLayout.svelte';
	import type { LayoutProps } from '@/lib/LoadProps';

	let { children }: LayoutProps<() => void> = $props();
</script>

<div class="main">
	<Header
		breadcrumbs={[
			{
				label: '🛶 HostDinghy',
				url: '/',
			},
			{
				label: 'Settings',
				url: `/settings/account/`,
			},
		]}
	/>

	<div class="wrap">
		<TabLayout
			sidebar={[
				{
					label: 'Account',
					url: `/settings/account/`,
				},
				{
					label: 'Appearance',
					url: `/settings/appearance/`,
				},
				{
					label: 'Servers',
					url: '/settings/servers',
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
