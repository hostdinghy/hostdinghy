<script lang="ts">
	let {
		headers,
		default: def = null,
		class: extraClasses = '',
		...content
	} = $props();

	let tabId = $state(def ?? Object.keys(headers)[0]);
</script>

<div class="tabs {extraClasses}">
	<nav>
		<ul>
			{#each Object.entries(headers) as [id, label] (id)}
				<li>
					<button
						class:active={id === tabId}
						onclick={() => (tabId = id)}
					>
						{label}
					</button>
				</li>
			{/each}
		</ul>
	</nav>

	<div class="content">
		{#if tabId in content}
			{@render content[tabId]()}
		{/if}
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
		button {
			cursor: pointer;
			text-align: left;
			padding: 0.5rem 1rem;
			width: 100%;
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
