<script lang="ts" generics="Row extends Record<string, any>">
	import type { Snippet } from 'svelte';

	type MainProps = {
		headers: { key: keyof Row; value: string }[];
		rows: Row[];
		/** border x */
		bx?: boolean;
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
		bx = true,
		...customCells
	}: MainProps & RowSnippets = $props();
</script>

<table class:bx>
	<thead>
		<tr>
			{#each headers as header (header.key)}
				<th>{header.value}</th>
			{/each}
		</tr>
	</thead>
	<tbody>
		{#each rows as row, i (row.id ?? i)}
			<tr>
				{#each headers as header (header.key)}
					<!-- this fixes type inference -->
					{@const cc = customCells[header.key as string]}
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

<style lang="scss">
	table {
		width: 100%;
		--border: 1px solid var(--c-border);

		thead {
			border: var(--border);

			th {
				text-align: left;
				padding: 1rem;
			}
		}

		tbody :global {
			td {
				border: var(--border);
				padding: 1rem;
			}

			// reduce padding for button groups
			td:has(.button-group) {
				padding: 0.5rem 1rem;
			}
		}
	}

	table:not(.bx) {
		thead {
			border-left: none;
			border-right: none;
		}

		tbody :global {
			td:first-child {
				border-left: none;
			}

			td:last-child {
				border-right: none;
			}
		}
	}
</style>
