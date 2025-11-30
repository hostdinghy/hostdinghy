<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		bt = true,
		bx = true,
		children,
	}: {
		/** border top */
		bt?: boolean;
		/** border x */
		bx?: boolean;
		children: Snippet;
	} = $props();
</script>

<div class="toolbar" class:bt class:bx>
	{@render children()}
</div>

<style lang="scss">
	.toolbar {
		display: flex;
		min-height: 3.5rem;
		padding: 1rem;
		gap: 1rem;
		border: 1px solid var(--c-border);
		border-bottom: none;
		justify-content: end;
		align-items: center;
	}

	.toolbar:not(.bt) {
		border-top: none;
	}

	.toolbar:not(.bx) {
		border-left: none;
		border-right: none;
	}

	.toolbar :global {
		h2 {
			flex: 1;
			font-size: 1.125rem;
		}
	}
</style>
