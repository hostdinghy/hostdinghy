<script module lang="ts">
	import { all } from '@/api/servers';

	export async function loadProps() {
		return {
			servers: await all(),
		};
	}

	export type Props = ResolvedProps<typeof loadProps>;
</script>

<script lang="ts">
	import { create } from '@/api/apps';
	import Input from '@/form/Input.svelte';
	import Select from '@/form/Select.svelte';
	import { getRouter } from '@/main';
	import type { ResolvedProps } from '@/lib/LoadProps';

	let name = $state('');
	let id = $state('');
	let serverId = $state('');
	let error = $state();
	let { servers }: Props = $props();

	const router = getRouter();

	async function onsubmit(e) {
		e.preventDefault();

		const app = await create({
			id,
			name,
			serverId,
		});
		router.open(`/apps/${app.id}`);
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
				options={servers.map(s => ({ value: s.name, key: s.id }))}
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
		min-height: 100vh;
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
