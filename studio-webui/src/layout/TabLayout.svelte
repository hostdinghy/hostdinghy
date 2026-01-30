<script module lang="ts">
	export type Props = {
		sidebar: ({ url: string; label: string } | { url: ''; label: '' })[];
		children: Snippet;
	};
</script>

<script lang="ts">
	import { getRouter } from '@/main';
	import type { Snippet } from 'svelte';

	let { sidebar, children }: Props = $props();

	const page = getRouter().currentRequest;
	const pathname = $derived($page.url.pathname);

	if (
		import.meta.env.DEV &&
		new Set(sidebar.map(item => item.url)).size !== sidebar.length
	)
		throw new Error('each sidebar item needs to have a unique url');
</script>

<div class="tabs">
	<nav>
		<ul>
			{#each sidebar as item (item.url)}
				{#if !item.label}
					<li class="line" aria-hidden="true"></li>
				{:else}
					<li>
						<a
							class:active={item.url.replace(/\/$/, '') ===
								pathname.replace(/\/$/, '')}
							href={item.url}
						>
							{item.label}
						</a>
					</li>
				{/if}
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
		border: 1px solid var(--c-border);
	}

	nav {
		flex-basis: 17rem;
		border-right: 1px solid var(--c-border);
		padding: 2rem 1rem 0 1rem;
	}

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
			border-color: hsl(from var(--c-border) h 70% l);
			color: var(--c-accent);
			background: hsl(from var(--c-accent) h s l / 5%);
		}
	}

	.line {
		margin: 1rem 0;
		height: 1px;
		background-color: var(--c-border);
	}

	.content {
		display: flex;
		flex-direction: column;
		flex: 1;
	}
</style>
