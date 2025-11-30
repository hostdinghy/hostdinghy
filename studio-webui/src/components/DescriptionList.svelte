<script lang="ts">
	let {
		list,
	}: {
		list: Record<string, string>;
	} = $props();
</script>

<dl>
	{#each Object.entries(list) as [key, value] (key)}
		<dt>{key}</dt>
		<dd>{value}</dd>
	{/each}
</dl>

<style lang="scss">
	dl {
		display: grid;
		grid-template-columns: 10rem 1fr;
		gap: 0.5rem 1rem;
	}

	dt {
		color: var(--c-label);
	}
</style>
