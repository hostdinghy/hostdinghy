<script module lang="ts">
	// check with Button as well
	export type Style = 'accent' | 'text';
	export type Align = 'left' | 'right';
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		children,
		style = 'accent',
		align = 'left',
	}: {
		children: Snippet;
		style?: Style;
		align?: Align;
	} = $props();
</script>

<div class="button-group style-{style} align-{align}">
	{@render children()}
</div>

<style lang="scss">
	.button-group {
		display: flex;
	}

	// only the direct buttons and the buttons of context menus, the open
	// menu next to a context menu button must not change its borders
	.button-group > :global {
		.btn,
		.btn svg,
		.context-menu {
			flex-shrink: 0;
		}

		.btn:not(:last-child),
		.context-menu:not(:last-child) > .btn {
			border-right: none;
		}
	}

	.style-text {
		border: 1px solid var(--c-border);
	}

	.style-text > :global {
		.btn,
		.context-menu > .btn {
			border: none;
			color: var(--c-text);
		}

		.btn:not(:last-child),
		.context-menu:not(:last-child) > .btn {
			border-right: 1px solid var(--c-border);
		}
	}

	.align-right {
		width: fit-content;
		margin-left: auto;
	}
</style>
