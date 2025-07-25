<script module>
	import { all, App } from '@/api/apps';
	import Button from '@/components/Button.svelte';
	import Status from '@/components/Status.svelte';

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
			{ key: 'status', value: 'Status' },
			{ key: 'port', value: 'Uptime' },
			{ key: 'rule', value: 'Actions' },
		]}
		rows={apps.map(a => ({ ...a, status: 'running' }))}
	>
		{#snippet toolbar()}
			<Button href="/apps/create">add</Button>
		{/snippet}

		{#snippet name(row)}
			<td>
				<a class="underline" href="/apps/{row.id}">{row.name}</a>
			</td>
		{/snippet}

		{#snippet status(row)}
			<td>
				<div class="status">
					<Status value={row.status} />
					{row.status}
				</div>
			</td>
		{/snippet}
	</Table>
</div>

<style>
	.layout {
		margin-top: 5rem;
	}
	.status {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.demo {
		margin-top: 2rem;
	}
</style>
