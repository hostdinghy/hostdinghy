<script lang="ts">
	import { Writable } from 'chuchi/stores';
	import './app.scss';
	import './lib/Editor'; // todo: we should probably only load this if a page uses the editor
	import Toasts from './layout/Toasts.svelte';
	import type { RoutePage } from './main';

	const { page }: { page: Writable<RoutePage> } = $props();

	const LayoutComponent = $derived($page.layout);
	const Component = $derived($page.component);
</script>

<Toasts />

<div id="app">
	{#if LayoutComponent}
		<LayoutComponent {...$page.props}>
			<Component {...$page.props} />
		</LayoutComponent>
	{:else}
		<Component {...$page.props} />
	{/if}
</div>

<style lang="scss">
</style>
