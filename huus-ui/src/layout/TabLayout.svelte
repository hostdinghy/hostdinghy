<script lang="ts">
	import { getRouter } from '@/main';
	import type { Snippet } from 'svelte';

	interface Props {
		sidebar: { url: string; label: string }[];
		children: Snippet;
	}
	let { sidebar, children }: Props = $props();

	const page = getRouter().currentRequest;
	const pathname = $derived($page.url.pathname);
</script>

<div class="tabs">
	<nav>
		<ul>
			{#each sidebar as item}
				<li>
					<a
						class:active={item.url.replace(/\/$/, '') ===
							pathname.replace(/\/$/, '')}
						href={item.url}
					>
						{item.label}
					</a>
				</li>
			{/each}
		</ul>
	</nav>

	<div class="content">
		{@render children()}
	</div>
</div>

<style lang="scss">
	.tabs {
		display: flex;
		flex: 1;
		gap: 2rem;
		border: 1px solid var(--c-border);
	}
	nav {
		flex-basis: 17rem;
		border-right: 1px solid var(--c-border);
		padding: 2rem 1rem 0 1rem;
		ul {
			display: flex;
			flex-direction: column;
			gap: 0.25rem;
		}
		a {
			cursor: pointer;
			text-align: left;
			padding: 0.5rem 1rem;
			display: block;
			border: 1px solid transparent;
			&.active {
				border-color: var(--c-border);
				color: var(--c-accent);
				background: hsl(from var(--c-accent) h s l / 5%);
			}
		}
	}
	.content {
		padding-top: 2rem;
	}
</style>
