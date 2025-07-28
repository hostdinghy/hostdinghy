<script module>
	import { App, byId } from '@/api/apps';
	import Tabs from '@/components/Tabs.svelte';

	// export const layout = 'app';

	export async function loadProps({ id }) {
		return {
			app: await byId(id),
		};
	}
</script>

<script lang="ts">
	let { id, app }: { id: string; app: App } = $props();
</script>

<svelte:head>
	<title>Huus</title>
</svelte:head>

<div class="wrap">
	<Tabs
		headers={{ overview: 'Overview', settings: 'Settings', logs: 'Logs' }}
	>
		{#snippet overview()}
			<h1>{app.name}</h1>
		{/snippet}
		{#snippet settings()}
			settings
		{/snippet}
		{#snippet logs()}
			logs
		{/snippet}
	</Tabs>
</div>

<style>
	.wrap {
		display: flex;
		margin-top: 2rem;
	}
</style>
