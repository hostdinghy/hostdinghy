<script module lang="ts">
	import { loadServers } from '@/api/servers';

	export async function loadProps() {
		return {
			servers: await loadServers(),
		};
	}
</script>

<script lang="ts">
	import { create } from '@/api/apps';
	import Input from '@/form/Input.svelte';
	import Select from '@/form/Select.svelte';
	import { getRouter } from '@/main';
	import type { ResolvedProps } from '@/lib/LoadProps';
	import { errorToStr } from '@/api/lib';

	const router = getRouter();

	let { servers }: ResolvedProps<typeof loadProps> = $props();

	let name = $state('');
	let id = $state('');
	let serverId = $state(servers.first()?.id ?? '');
	let error = $state('');

	async function onsubmit(e: Event) {
		e.preventDefault();

		error = '';

		try {
			const app = await create({
				id,
				name,
				serverId,
			});
			router.open(`/apps/${app.id}`);
		} catch (e) {
			console.error('Error creating app:', e);
			error = errorToStr(e);
		}
	}
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div id="createApp">
	<div class="center">
		<h1>Create App</h1>
		<form {onsubmit}>
			<Input
				id="name"
				name="name"
				type="text"
				label="name"
				placeholder="Enter App Name..."
				bind:value={name}
				required
			/>

			<Input
				id="id"
				name="id"
				type="text"
				label="id"
				placeholder="Enter a unique id..."
				bind:value={id}
				required
			/>

			<Select
				id="server-id"
				name="server-id"
				label="server"
				bind:value={serverId}
				options={servers.all().map(s => ({ value: s.name, key: s.id }))}
			/>

			{#if error}
				<div class="error">{error}</div>
			{/if}

			<button type="submit" class="btn mt">submit</button>
		</form>
	</div>
</div>

<style lang="scss">
	#createApp {
		display: flex;
		width: 100%;
		justify-content: center;
		align-items: center;
	}

	h1 {
		font-size: 1.25rem;
		margin-bottom: 2rem;
	}

	.error {
		margin-top: 1.5rem;
		color: var(--red);
	}
</style>
