<script module>
	import { all, App } from '@/api/apps';
	import Button from '@/components/Button.svelte';

	export async function loadProps() {
		return {
			apps: await all(),
		};
	}
</script>

<script lang="ts">
	import Table from '@/components/Table.svelte';
	let { apps }: { apps: App[] } = $props();
</script>

<svelte:head>
	<title>Huus</title>
</svelte:head>

<div class="layout wrap">
	<Table
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'protocol', value: 'Status' },
			{ key: 'port', value: 'Uptime' },
			{ key: 'rule', value: 'Actions' },
		]}
		rows={apps}
	>
		{#snippet toolbar()}
			<Button href="/apps/create">add</Button>
		{/snippet}
	</Table>
</div>

<style>
	.layout {
		margin-top: 5rem;
	}
</style>
