<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		children,
		bb = false,
	}: {
		children: Snippet;
		/** border bottom */
		bb?: boolean;
	} = $props();
</script>

<header class:bb>
	{@render children()}
</header>

<style lang="scss">
	header {
		padding: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	header.bb {
		border-bottom: 1px solid var(--c-border);
	}

	header :global {
		h1 {
			font-size: 1.125rem;
		}
	}
</style>
