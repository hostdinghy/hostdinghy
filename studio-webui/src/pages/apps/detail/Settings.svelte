<script module lang="ts">
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import {
		COMMAND_TITLES,
		composeCommand,
		createCompose,
		loadCompose,
	} from '@/api/apps/compose';

	export async function loadProps({ app }: AppLayoutProps) {
		return {
			compose: await loadCompose(app.id),
		};
	}
</script>

<script lang="ts">
	import Button from '@/components/Button.svelte';
	import Editor from '@/components/Editor.svelte';
	import CommitConfigModal from '@/layout/modals/CommitConfig.svelte';
	import { createToastHandler } from '@/layout/Toasts.svelte';
	import { errorToStr } from '@/api/lib';
	import Header from '@/components/Header.svelte';
	import CommandOutput, {
		type CommandRun,
	} from '@/components/CommandOutput.svelte';

	let { app, compose }: AppLayoutProps<typeof loadProps> = $props();

	let toast = createToastHandler();

	let editor: Editor;
	let commitConfigOpen = $state(false);
	let original = $state(compose);
	let modified = $state(compose);

	function onsave(newValue: string) {
		toast.remove();

		modified = newValue;
		commitConfigOpen = true;
	}

	let run = $state<CommandRun | null>(null);

	async function oncommit() {
		if (run?.status === 'running') return;
		toast.remove();

		try {
			original = await createCompose(app.id, modified);
			modified = original;
			editor.setValue(original);
			commitConfigOpen = false;
		} catch (e: any) {
			toast.error(errorToStr(e));
			return;
		}

		// apply the new config
		const title = `Config saved, ${COMMAND_TITLES.up.toLowerCase()} ${app.name}`;
		const r: CommandRun = $state({
			title,
			lines: [],
			status: 'running',
			error: '',
		});
		run = r;

		try {
			await composeCommand(app.id, null, 'up', line =>
				r.lines.push(line),
			);
			r.status = 'done';
			toast.success('New config saved and applied');
		} catch (e) {
			r.status = 'error';
			r.error = errorToStr(e);
			toast.error('New config saved, but applying it failed');
		}
	}

	function onreset() {
		commitConfigOpen = false;
		modified = original;
		editor.setValue(original);
	}
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div class="settings">
	<Header bb>
		<div class="title">
			<h1>Docker Compose</h1>
			{#if modified !== original}
				<span class="unsaved c-label">unsaved</span>
			{/if}
		</div>

		<Button
			disabled={run?.status === 'running'}
			onclick={() => editor.save()}
		>
			save
		</Button>
	</Header>

	{#if run}
		<CommandOutput {run} onclose={() => (run = null)} />
	{/if}

	<Editor value={compose} {onsave} bind:this={editor} />
</div>

<CommitConfigModal
	open={commitConfigOpen}
	{original}
	{modified}
	{oncommit}
	{onreset}
/>

<style lang="scss">
	.settings {
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	.title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
</style>
