<script lang="ts">
	import type { Snippet } from 'svelte';
	import type {
		HTMLButtonAttributes,
		HTMLAnchorAttributes,
	} from 'svelte/elements';

	let {
		children,
		iconColor = 'default',
		class: cls = '',
		...rest
	}: (HTMLButtonAttributes | HTMLAnchorAttributes) & {
		children?: Snippet;
		iconColor?: 'default' | 'red';
		class?: string;
	} = $props();
</script>

<svelte:element
	this={'href' in rest ? 'a' : 'button'}
	class="btn {cls} icon-{iconColor}"
	{...rest}
>
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

	.icon-red {
		color: var(--red);
	}
</style>
