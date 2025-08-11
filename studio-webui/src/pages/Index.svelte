<script module lang="ts">
	import { all, AppSummary } from '@/api/apps';

	// Maybe another type approach?
	// signature of loadProps is enforced but return type needs
	// to be defined, also no that nice
	type Props = {
		apps: AppSummary[];
	};

	export const loadProps: LoadPropsFn = async (): Promise<Props> => {
		return {
			apps: await all(),
		};
	};
</script>

<script lang="ts">
	import Table from '@/components/Table.svelte';
	import Button from '@/components/Button.svelte';
	import Status from '@/components/Status.svelte';
	import type { LoadPropsFn } from '@/lib/LoadProps';

	let { apps }: Props = $props();
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div class="layout wrap">
	<Table
		search
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'serverName', value: 'Server' },
			{ key: 'status', value: 'Status' },
			{ key: 'rule', value: 'Actions' },
		]}
		rows={apps.map(a => ({
			...a,
			serverName: a.server?.name,
			status: null,
		}))}
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
						<Status value={state} />
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
</style>
