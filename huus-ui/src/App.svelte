<script lang="ts">
	import { Writable } from 'chuchi/stores';
	import './app.scss';
	import Toasts from './layout/Toasts.svelte';
	import type { RoutePage } from './main';
	import MainLayout from './layout/MainLayout.svelte';

	const { page }: { page: Writable<RoutePage> } = $props();

	const Component = $derived($page.component);
</script>

<Toasts />

<div id="app">
	{#if $page.layout === 'main'}
		<MainLayout>
			<Component {...$page.props} />
		</MainLayout>
	{:else}
		<Component {...$page.props} />
	{/if}
</div>

<style lang="scss">
</style>
