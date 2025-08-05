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
	<title>HostDinghy</title>
</svelte:head>

<div class="layout wrap">
	<Table
		search={true}
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'status', value: 'Status' },
			{ key: 'port', value: 'Uptime' },
			{ key: 'rule', value: 'Actions' },
		]}
		rows={apps}
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
					{#each row.servicesStates as state}
						<Status value={state.toLowerCase()} />
					{:else}
						no services
					{/each}
					{#if new Set(row.servicesStates).size === 1}
						{row.servicesStates[0].toLowerCase()}
					{/if}
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
