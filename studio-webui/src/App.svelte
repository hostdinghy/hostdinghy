<script lang="ts">
	import { Writable } from 'chuchi/stores';
	import './app.scss';
	import './lib/Editor'; // todo: we should probably only load this if a page uses the editor
	import PageLoader from './layout/PageLoader.svelte';
	import Toasts from './layout/Toasts.svelte';
	import type { RoutePage } from './main';
	import { onMount } from 'svelte';
	import { themeModeMount } from './lib/theme/themeMode';

	// page is a private implementation detail of App (and client.ts) so it should not be exposed
	// globally
	const {
		page,
		loading,
	}: { page: Writable<RoutePage>; loading: Writable<number> } = $props();

	const LayoutComponent = $derived($page.layout);
	const Component = $derived($page.component);
	const pageProps = $derived($page.props);

	onMount(() => {
		themeModeMount();
	});
</script>

<PageLoader {loading} />
<Toasts />

<div id="app">
	{#if LayoutComponent}
		<LayoutComponent {...pageProps}>
			<Component {...pageProps} />
		</LayoutComponent>
	{:else}
		<Component {...pageProps} />
	{/if}
</div>

<style lang="scss">
</style>
