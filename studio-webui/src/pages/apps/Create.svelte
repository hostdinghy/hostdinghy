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
	import Button from '@/components/Button.svelte';

	const router = getRouter();

	let { servers }: ResolvedProps<typeof loadProps> = $props();

	let name = $state('');
	let id = $state('');
	// the id follows the name until it is changed by hand
	let idEdited = $state(false);
	let serverId = $state(servers.first()?.id ?? '');
	let error = $state('');
	let creating = $state(false);

	/** lowercase letters, digits and dashes, no dash at the ends */
	function idFromName(name: string): string {
		return (
			name
				.toLowerCase()
				// ä -> a, é -> e
				.normalize('NFKD')
				.replace(/[\u0300-\u036f]/g, '')
				.replace(/[^a-z0-9]+/g, '-')
				.slice(0, 42)
				.replace(/^-+|-+$/g, '')
		);
	}

	function onNameInput() {
		if (!idEdited) id = idFromName(name);
	}

	function onIdInput() {
		// an emptied id follows the name again
		idEdited = id !== '';
	}

	async function onsubmit(e: Event) {
		e.preventDefault();
		if (creating) return;

		error = '';
		creating = true;

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
		} finally {
			creating = false;
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
				oninput={onNameInput}
				required
			/>

			<Input
				id="id"
				name="id"
				type="text"
				label="id"
				placeholder="Enter a unique id..."
				bind:value={id}
				oninput={onIdInput}
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

			<p class="help">
				The id can't be changed later. 3 to 42 lowercase letters,
				digits, dashes or underscores, starting and ending with a letter
				or digit.
			</p>

			<Button mt disabled={creating}>
				{creating ? 'creating...' : 'submit'}
			</Button>
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

	.help {
		margin-top: 1rem;
		color: rgb(from var(--c-text) r g b / 70%);
		font-size: 0.875rem;
	}

	.error {
		margin-top: 1.5rem;
		color: var(--red);
	}
</style>
