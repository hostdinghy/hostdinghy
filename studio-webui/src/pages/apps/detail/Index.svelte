<script lang="ts">
	import type { Service } from '@/api/apps';
	import Status from '@/components/Status.svelte';
	import Table from '@/components/Table.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import type { ResolvedProps } from '@/lib/LoadProps';

	let { app }: AppLayoutProps<() => void> = $props();
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<header class:border={!app.services.length}>
	<h1>
		{app.name}
	</h1>
</header>

{#if app.services.length > 0}
	<!-- the type shenanigangs for domains is not that nice -->
	<Table
		headers={[
			{ value: 'Name', key: 'name' },
			{ value: 'State', key: 'state' },
			{ value: 'State Human Readable', key: 'stateHr' },
			{ value: 'Domains', key: 'domains' },
		]}
		rows={app.services as ((typeof app.services)[number] & {
			domains: any;
		})[]}
	>
		{#snippet state(row)}
			<td>
				<div class="status">
					<Status value={row.state} />
					{row.state.toLowerCase()}
				</div>
			</td>
		{/snippet}

		{#snippet domains(row)}
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

<style lang="scss">
	header {
		padding: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;

		&.border {
			border-bottom: 1px solid var(--c-border);
		}
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
