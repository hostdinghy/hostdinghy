<script module lang="ts">
	import type { Component } from 'svelte';

	export type ContextMenuItem = {
		label: string;
		icon?: Component;
		onclick: () => void;
		/** e.g. delete */
		danger?: boolean;
	};
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import Button, { type Style } from './Button.svelte';

	/** A button which opens a list of actions below it */
	let {
		items,
		children,
		title,
		style = 'accent',
		align = 'right',
	}: {
		items: ContextMenuItem[];
		/** the content of the button, e.g. an icon */
		children: Snippet;
		title: string;
		style?: Style;
		/** which edge of the button the menu lines up with */
		align?: 'left' | 'right';
	} = $props();

	let open = $state(false);
	let el: HTMLDivElement;

	function onwindowclick(e: MouseEvent) {
		if (open && !el.contains(e.target as Node)) open = false;
	}

	function onkeydown(e: KeyboardEvent) {
		if (open && e.key === 'Escape') open = false;
	}

	function onitem(item: ContextMenuItem) {
		open = false;
		item.onclick();
	}
</script>

<svelte:window onclick={onwindowclick} {onkeydown} />

<div class="context-menu" bind:this={el}>
	<Button
		{style}
		{title}
		aria-label={title}
		aria-haspopup="menu"
		aria-expanded={open}
		onclick={() => (open = !open)}
	>
		{@render children()}
	</Button>

	{#if open}
		<div class="menu align-{align}" role="menu">
			{#each items as item (item.label)}
				<button
					class="item"
					class:danger={item.danger}
					role="menuitem"
					onclick={() => onitem(item)}
				>
					{#if item.icon}
						<item.icon />
					{/if}
					{item.label}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.context-menu {
		position: relative;
		display: flex;
	}

	.menu {
		position: absolute;
		top: calc(100% + 0.25rem);
		z-index: 20;
		display: flex;
		flex-direction: column;
		width: max-content;
		min-width: 12rem;
		max-width: 20rem;
		padding: 0.25rem 0;
		border: 1px solid var(--c-border);
		background-color: var(--c-bg);
		// black in light and dark mode
		box-shadow: 0 0.25rem 1rem rgb(0 0 0 / 15%);
	}

	.align-right {
		right: 0;
	}

	.align-left {
		left: 0;
	}

	.item {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		padding: 0.5rem 1rem;
		text-align: left;
		cursor: pointer;

		&:hover {
			background-color: rgb(from var(--c-accent) r g b / 8%);
		}

		:global(svg) {
			width: 1.25rem;
			height: 1.25rem;
			flex-shrink: 0;
		}
	}

	.danger {
		color: var(--red);
	}
</style>
