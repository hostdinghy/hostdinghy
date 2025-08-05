<script lang="ts">
	import type { Service } from '@/api/apps';
	import Status from '@/components/Status.svelte';
	import Table from '@/components/Table.svelte';
	import type { loadProps } from '@/layout/AppLayout.svelte';
	import type { ResolvedProps } from '@/lib/LoadProps';

	let { app }: ResolvedProps<typeof loadProps> = $props();
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<header>
	<h1>
		{app.name}
	</h1>
</header>

{#if app.services.length > 0}
	<Table
		headers={[
			{ value: 'Name', key: 'name' },
			{ value: 'State', key: 'state' },
			{ value: 'State Human Readable', key: 'stateHr' },
			{ value: 'Domains', key: 'domains' },
		]}
		rows={app.services}
	>
		{#snippet state(row)}
			<td>
				<div class="status">
					<Status value={row.state.toLowerCase()} />
					{row.state.toLowerCase()}
				</div>
			</td>
		{/snippet}
		{#snippet domains(row: Service)}
			<td>
				{#each row.routes as route}
					{#each route.domains as domain}
						<a
							target="_blank"
							class="underline"
							href="https://{domain}"
						>
							{domain}
						</a>
					{/each}
				{/each}
			</td>
		{/snippet}
	</Table>
{:else}
	<p class="msg">
		No services. Add services to your compose file in the <a
			class="underline"
			href="/apps/{app.id}/settings"
		>
			settings
		</a>
		tab.
	</p>
{/if}

<style>
	header {
		padding: 1rem;
		border-bottom: 1px solid var(--c-border);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	h1 {
		font-size: 1.125rem;
	}
	.msg {
		padding: 1rem;
		color: rgb(from var(--c-text) r g b / 70%);
		.underline {
			color: var(--c-text);
		}
	}

	.status {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}
</style>
