<script lang="ts">
	type State =
		| 'empty'
		| 'created'
		| 'unhealthy'
		| 'running'
		| 'paused'
		| 'restarting'
		| 'exited'
		| 'removing'
		| 'dead'
		| 'unknown';

	let { value, size = 'md' }: { value: State; size: 'sm' | 'md' | 'lg' } =
		$props();
</script>

<div class="status {size}" data-status={value} title={value}></div>

<style>
	.status {
		border-radius: 50%;
		border: 0.1rem solid transparent;
		flex-shrink: 0;
	}
	.sm {
		width: 0.25rem;
		height: 0.25rem;
	}
	.md {
		width: 0.5rem;
		height: 0.5rem;
	}
	.lg {
		width: 0.75rem;
		height: 0.75rem;
	}

	[data-status='running'] {
		background-color: var(--green);
		border-color: var(--green);
	}

	[data-status='created'] {
		background-color: var(--blue);
		border-color: var(--blue);
	}

	[data-status='unhealthy'] {
		background-color: var(--red);
		border-color: var(--red);
	}

	[data-status='running'] {
		background-color: var(--green);
		border-color: var(--green);
	}

	[data-status='paused'] {
		background-color: var(--yellow);
		border-color: var(--yellow);
	}

	[data-status='restarting'] {
		background-color: var(--orange);
		border-color: var(--orange);
	}

	[data-status='removing'] {
		background-color: var(--purple);
		border-color: var(--purple);
	}

	[data-status='empty'],
	[data-status='exited'] {
		background-color: transparent;
		&.sm {
			background-color: var(--c-border);
		}
		border-color: var(--c-border);
	}
	[data-status='unknown'],
	[data-status='dead'] {
		background-color: transparent;
		&.sm {
			background-color: var(--purple);
		}
		border-color: var(--purple);
	}
</style>
