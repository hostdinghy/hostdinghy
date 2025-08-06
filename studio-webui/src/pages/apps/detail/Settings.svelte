<script module lang="ts">
	import * as composeApi from '@/api/apps/compose';
	import type { loadProps as layoutLoadProps } from '@/layout/AppLayout.svelte';

	export async function loadProps({ id }: { id: string }) {
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
	import { toast, type ToastRef } from '@/layout/Toasts.svelte';
	import { errorToStr } from '@/api/lib';

	let { app, compose }: Props = $props();

	let editor: Editor;
	let commitConfigOpen = $state(false);
	let original = $state(compose);
	let modified = $state(compose);

	let toastRef: ToastRef | null = null;

	function onsave(newValue: string) {
		toastRef?.remove();

		modified = newValue;
		commitConfigOpen = true;
	}

	async function oncommit() {
		toastRef?.remove();

		try {
			original = await composeApi.set(app.id, modified);
			modified = original;
			editor.setValue(original);

			toastRef = toast({
				status: 'success',
				message: 'new Config saved',
			});
		} catch (e: any) {
			toastRef = toast({
				status: 'error',
				message: errorToStr(e),
			});
		}
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
		<div class="title">
			<h1>Docker Compose</h1>
			{#if modified !== original}
				<span class="unsaved">unsaved</span>
			{/if}
		</div>

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
	.title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	h1 {
		font-size: 1.125rem;
	}
	.unsaved {
		opacity: 0.5;
		font-size: 0.9rem;
	}
</style>
