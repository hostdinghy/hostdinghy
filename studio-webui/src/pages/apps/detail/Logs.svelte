<script module lang="ts">
	import { logs as loadLogs } from '@/api/apps';

	export async function loadProps({ app }: AppLayoutProps) {
		return {
			logs: await loadLogs(app.id),
		};
	}
</script>

<script lang="ts">
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';

	let { app, logs }: AppLayoutProps<typeof loadProps> = $props();
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div class="logs">
	{#if logs}
		<pre>{logs}</pre>
	{:else}
		<p class="msg">No logs available</p>
	{/if}
</div>

<style lang="scss">
	.logs {
		padding: 1rem;
	}

	pre {
		white-space: pre-wrap;
		word-break: break-word;
	}

	.msg {
		color: rgb(from var(--c-text) r g b / 70%);

		.underline {
			color: var(--c-text);
		}
	}
</style>
