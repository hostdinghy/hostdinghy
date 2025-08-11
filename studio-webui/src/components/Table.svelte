<script lang="ts" generics="Row extends Record<string, any>">
	import type { Snippet } from 'svelte';

	type MainProps = {
		headers: { key: string; value: string }[];
		rows: Row[];
		toolbar?: Snippet;
		search?: boolean;
	};

	type RowSnippets = {
		// we need to map and cant use [key: string]
		// because else this type would need to be all typeof
		// of MainProps
		[Property in keyof Row]?: Snippet<[Row]>;
	};

	let {
		headers,
		rows,
		toolbar = undefined,
		search = false,
		...customCells
	}: MainProps & RowSnippets = $props();
</script>

<div class="data-table">
	{#if search || toolbar}
		<div class="pre-header">
			{#if search}
				<input class="search" placeholder="search" type="text" />
			{/if}
			{#if toolbar}
				{@render toolbar()}
			{/if}
		</div>
	{/if}
	<table>
		<thead>
			<tr>
				{#each headers as header}
					<th>{header.value}</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each rows as row, i}
				<tr>
					{#each headers as header}
						<!-- this fixes type inference -->
						{@const cc = customCells[header.key]}
						{#if cc}
							{@render cc(row)}
						{:else}
							<td>{row[header.key] ?? '-'}</td>
						{/if}
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style lang="scss">
	.data-table {
		--border: 1px solid var(--c-border);
	}
	table {
		width: 100%;
		thead {
			border: var(--border);
			th {
				text-align: left;
				padding: 1rem;
			}
		}
		tbody {
			:global {
				td {
					border: var(--border);
					padding: 1rem;
				}
			}
		}
	}

	input::placeholder {
		color: var(--c-accent);
	}

	.pre-header {
		display: flex;
		min-height: 3.5rem;
		border: var(--border);
		border-bottom-style: none;
		justify-content: end;
	}

	.search {
		flex: 1;
		padding: 1rem;
	}
</style>
