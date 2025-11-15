<script lang="ts">
	import { byId, Service } from '@/api/apps';
	import { composeCommand, type ComposeCommand } from '@/api/apps/compose';
	import PlayArrow from '@/assets/icons/PlayArrow.svelte';
	import Refresh from '@/assets/icons/Refresh.svelte';
	import Stop from '@/assets/icons/Stop.svelte';
	import Sync from '@/assets/icons/Sync.svelte';
	import Button from '@/components/Button.svelte';
	import ButtonGroup from '@/components/ButtonGroup.svelte';
	import Status from '@/components/Status.svelte';
	import Table from '@/components/Table.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import { onMount } from 'svelte';

	let { app }: AppLayoutProps = $props();

	let loading = false;
	async function reloadApp() {
		if (loading) return;
		loading = true;

		try {
			app = await byId(app.id);
		} catch (e) {
			console.error('Failed to reload app:', e);
		}

		loading = false;
	}

	async function onComposeCommand(
		e: Event,
		service: string | null,
		command: ComposeCommand,
	) {
		e.preventDefault();

		try {
			await composeCommand(app.id, service, command);
		} catch (e) {
			alert('Failed to execute command: ' + e);
		}
	}

	onMount(() => {
		const int = setInterval(reloadApp, 1000);
		return () => clearInterval(int);
	});
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<header class:border={!app.services.length}>
	<h1>
		<span class="name">{app.name}</span>
		<span class="id">[{app.id}]</span>
		<span class="server">running on {app.server?.name}</span>
	</h1>

	<ButtonGroup>
		<Button
			title="restart"
			aria-label="restart"
			onclick={e => onComposeCommand(e, null, 'restart')}
		>
			<Refresh />
		</Button>

		<Button
			title="pull"
			aria-label="pull"
			onclick={e => onComposeCommand(e, null, 'up')}
		>
			<Sync />
		</Button>
	</ButtonGroup>
</header>

{#if app.services.length > 0}
	<!-- the type shenanigangs for domains is not that nice -->
	<Table
		headers={[
			{ value: 'Name', key: 'name' },
			{ value: 'State', key: 'state' },
			{ value: 'State Human Readable', key: 'stateHr' },
			{ value: 'Domains', key: 'domains' },
			{ value: '', key: 'actions' },
		]}
		rows={app.services as (Service & { domains: any; actions: any })[]}
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
				{#each row.routes as route (route.rule)}
					{#each route.domains as domain (domain)}
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

		{#snippet actions(service)}
			<td class="actions">
				<ButtonGroup>
					{#if service.canStart()}
						<Button
							title="start"
							aria-label="start"
							onclick={e =>
								onComposeCommand(e, service.name, 'start')}
						>
							<PlayArrow />
						</Button>
					{/if}
					{#if service.canRestart()}
						<Button
							title="restart"
							aria-label="restart"
							onclick={e =>
								onComposeCommand(e, service.name, 'restart')}
						>
							<Refresh />
						</Button>
					{/if}
					{#if service.canStop()}
						<Button
							title="stop"
							aria-label="stop"
							onclick={e =>
								onComposeCommand(e, service.name, 'stop')}
						>
							<Stop />
						</Button>
					{/if}
					<Button
						title="pull"
						aria-label="pull"
						onclick={e => onComposeCommand(e, service.name, 'up')}
					>
						<Sync />
					</Button>
				</ButtonGroup>
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

		.id,
		.server {
			color: var(--c-label);
		}

		.server {
			font-size: 0.9rem;
		}
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

	// i dont really like the design here :/
	.actions :global {
		.button-group {
			width: fit-content;
			margin-left: auto;
			border: 1px solid var(--c-border);

			.btn {
				border: none;
				color: var(--white);

				&:not(:last-child) {
					border-right: 1px solid var(--c-border);
				}
			}
		}
	}
</style>
