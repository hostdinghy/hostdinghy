<script module lang="ts">
	export type IconColor = 'default' | 'red';
	// check with button group as well
	export type Style = 'accent' | 'text';
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import type {
		HTMLButtonAttributes,
		HTMLAnchorAttributes,
	} from 'svelte/elements';

	let {
		iconColor = 'default',
		mt = false,
		style = 'accent',
		class: cls = '',
		children,
		...rest
	}: (HTMLButtonAttributes | HTMLAnchorAttributes) & {
		iconColor?: IconColor;
		/** margin top */
		mt?: boolean;
		style?: Style;
		class?: string;
		children?: Snippet;
	} = $props();
</script>

<svelte:element
	this={'href' in rest ? 'a' : 'button'}
	class="btn {cls} icon-{iconColor} style-{style}"
	class:mt
	{...rest}
>
	{#if children}
		{@render children()}
	{/if}
</svelte:element>

<style lang="scss">
	.btn {
		display: flex;
		padding: 0.3rem 1rem;
		gap: 0.5rem;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--c-accent);
		color: var(--c-accent);
		cursor: pointer;
		transition: opacity 0.2s ease;
		text-decoration: none;

		&:hover {
			opacity: 0.7;
		}

		// has only an icon
		&:global(:has(svg:first-child:last-child)) {
			padding: 0.3rem;
		}
	}

	.icon-red {
		color: var(--red);
	}

	.style-text {
		border-color: var(--c-border);
		color: var(--c-text);
	}

	.mt {
		margin-top: 1.5rem;
	}
</style>
