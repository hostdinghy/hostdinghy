<script lang="ts">
	import type {
		HTMLButtonAttributes,
		HTMLAnchorAttributes,
	} from 'svelte/elements';

	let { children, ...rest }: HTMLButtonAttributes | HTMLAnchorAttributes =
		$props();
</script>

<svelte:element this={'href' in rest ? 'a' : 'button'} class="btn" {...rest}>
	{#if children}
		{@render children()}
	{/if}
</svelte:element>

<style lang="scss">
	.btn {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		justify-content: center;
	}
</style>
