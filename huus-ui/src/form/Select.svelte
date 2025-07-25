<script lang="ts">
	import Field from './Field.svelte';

	let {
		id,
		name,
		type,
		label,
		value = $bindable(),
		options,
		...rest
	}: {
		id: string;
		name: string;
		label: string;
		value: string;
		options: { value: string; [key: string]: any }[];
		[rest: string]: any;
	} = $props();
</script>

<Field {id} {label}>
	<select {id} {name} bind:value {...rest}>
		{#each options as option}
			<option value={option}>
				{option.value}
			</option>
		{/each}
	</select>
</Field>

<style lang="scss">
	select {
		padding: 0.5rem 1.5rem;
		background-color: transparent;
		border: none;
		min-height: 100%;
		color: var(--c-text);

		&::placeholder {
			color: var(--c-placeholder);
		}
	}
</style>
