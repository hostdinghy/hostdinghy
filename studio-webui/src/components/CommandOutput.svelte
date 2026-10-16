<script module lang="ts">
	export type CommandRun = {
		title: string;
		lines: string[];
		status: 'running' | 'done' | 'error';
		error: string;
	};
</script>

<script lang="ts">
	import Close from '@/assets/icons/Close.svelte';
	import Button from './Button.svelte';

	let {
		run,
		onclose,
	}: {
		run: CommandRun;
		onclose: () => void;
	} = $props();

	let outputEl: HTMLPreElement;

	// keep the newest line visible
	$effect(() => {
		run.lines.length;
		outputEl.scrollTop = outputEl.scrollHeight;
	});
</script>

<div class="command-output">
	<div class="head">
		<p class="title">
			{run.title}
			<span class="status status-{run.status}">
				{#if run.status === 'running'}
					running
					<span class="dots"></span>
				{:else if run.status === 'done'}
					done
				{:else}
					failed
				{/if}
			</span>
		</p>

		<Button
			style="text"
			title="close"
			aria-label="close"
			disabled={run.status === 'running'}
			onclick={onclose}
		>
			<Close />
		</Button>
	</div>

	<pre bind:this={outputEl}>{run.lines.join('\n')}</pre>

	{#if run.error}
		<p class="error">{run.error}</p>
	{/if}
</div>

<style lang="scss">
	.command-output {
		border-bottom: 1px solid var(--c-border);
	}

	.head {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 1rem;
	}

	.status {
		margin-left: 0.5rem;
		color: rgb(from var(--c-text) r g b / 70%);
	}

	.status-done {
		color: var(--c-accent);
	}

	.status-error {
		color: var(--red);
	}

	.dots::after {
		content: '';
		animation: dots 1.2s steps(4) infinite;
	}

	@keyframes dots {
		0% {
			content: '';
		}
		25% {
			content: '.';
		}
		50% {
			content: '..';
		}
		75% {
			content: '...';
		}
	}

	pre {
		max-height: 15rem;
		overflow: auto;
		margin: 0;
		padding: 0.5rem 1rem;
		white-space: pre-wrap;
		word-break: break-word;
		font-size: 0.875rem;
		background-color: rgb(from var(--c-text) r g b / 4%);
	}

	pre:empty {
		display: none;
	}

	.error {
		padding: 0.5rem 1rem;
		color: var(--red);
	}
</style>
