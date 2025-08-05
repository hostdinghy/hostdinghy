<script>
	import Button from './Button.svelte';

	let { headers, rows, toolbar, search = false, ...customCells } = $props();
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
						{#if customCells[header.key]}
							{@render customCells[header.key](row)}
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
		border: var(--border);
		border-bottom-style: none;
	}

	.search {
		flex: 1;
		padding: 1rem;
	}
</style>
