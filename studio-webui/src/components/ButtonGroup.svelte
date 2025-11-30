<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		children,
		style = 'accent',
		align = 'left',
	}: {
		children: Snippet;
		style?: 'accent' | 'text';
		align?: 'left' | 'right';
	} = $props();
</script>

<div class="button-group style-{style} align-{align}">
	{@render children()}
</div>

<style lang="scss">
	.button-group {
		display: flex;
	}

	.button-group :global {
		.btn,
		.btn svg {
			flex-shrink: 0;
		}

		.btn:not(:last-child) {
			border-right: none;
		}
	}

	.style-text :global {
		border: 1px solid var(--c-border);

		.btn {
			border: none;
			color: var(--c-text);

			&:not(:last-child) {
				border-right: 1px solid var(--c-border);
			}
		}
	}

	.align-right {
		width: fit-content;
		margin-left: auto;
	}
</style>
