<script module lang="ts">
	import * as composeApi from '@/api/apps/compose';
	import type { loadProps as layoutLoadProps } from '@/layout/AppLayout.svelte';

	export async function loadProps({ id }) {
		return {
			compose: await composeApi.get(id),
		};
	}

	type Props = ResolvedProps<typeof loadProps> &
		ResolvedProps<typeof layoutLoadProps>;
</script>

<script lang="ts">
	import Button from '@/components/Button.svelte';
	import Editor from '@/components/Editor.svelte';
	import CommitConfigModal from '@/layout/modals/CommitConfig.svelte';
	import type { ResolvedProps } from '@/lib/LoadProps';
	import { toast } from '@/layout/Toasts.svelte';

	let { app, compose }: Props = $props();

	let editor;
	let commitConfigOpen = $state(false);
	let original = $state(compose);
	let modified = $state(compose);

	function onsave(newValue) {
		modified = newValue;
		commitConfigOpen = true;
	}

	async function oncommit() {
		original = modified;
		const res = await composeApi.set(app.id, {
			createDatabase: false,
			compose: modified,
		});

		toast({
			status: 'success',
			message: 'new Config saved',
		});
	}

	function onreset() {
		modified = original;
		editor.setValue(original);
	}
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div class="settings">
	<header>
		<h1>Docker Compose</h1>
		<Button onclick={() => editor.save()}>save</Button>
	</header>
	<Editor value={compose} {onsave} bind:this={editor} />
</div>

<CommitConfigModal
	bind:open={commitConfigOpen}
	bind:original
	bind:modified
	{oncommit}
	{onreset}
/>

<style lang="scss">
	.settings {
		flex: 1;
		display: flex;
		flex-direction: column;
	}
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
</style>
