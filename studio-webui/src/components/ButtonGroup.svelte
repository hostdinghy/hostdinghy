<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="button-group">
	{@render children()}
</div>

<style lang="scss">
	.button-group {
		display: flex;
	}

	.button-group :global {
		.btn:not(:last-child) {
			border-right: none;
		}
	}
</style>
