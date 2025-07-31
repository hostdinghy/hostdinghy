<script module>
	import { all } from '@/api/servers';

	export async function loadProps() {
		return {
			servers: await all(),
		};
	}
</script>

<script>
	import { create } from '@/api/apps';
	import Input from '@/form/Input.svelte';
	import Select from '@/form/Select.svelte';
	import { getRouter } from '@/main';

	let name = $state();
	let id = $state();
	let server = $state();
	let error = $state();
	let { servers } = $props();

	const router = getRouter();

	async function onsubmit(e) {
		e.preventDefault();

		const app = await create({
			id,
			name,
			serverId: server.id,
		});
		router.open(`/apps/${app.id}`);
	}
</script>

<svelte:head>
	<title>Huus</title>
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
				bind:value={server}
				label="server"
				options={servers.map(s => ({ ...s, value: s.name }))}
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
		text-transform: uppercase;
	}

	.error {
		margin-top: 1.5rem;
		color: var(--red);
	}
</style>
