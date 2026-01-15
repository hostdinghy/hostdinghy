<script lang="ts">
	import Field from './Field.svelte';

	let {
		id,
		name,
		label,
		value = $bindable(),
		options,
		bx,
		...rest
	}: {
		id: string;
		name: string;
		label: string;
		value: string;
		options: { value: string; key: string }[];
		/** border x */
		bx?: boolean;
		[rest: string]: any;
	} = $props();
</script>

<Field {id} {label} {bx}>
	<select {id} {name} bind:value {...rest}>
		{#each options as option (option.key)}
			<option value={option.key}>
				{option.value}
			</option>
		{/each}
	</select>
</Field>

<style lang="scss">
	select {
		width: 100%;
		min-height: 100%;
		padding: 0.5rem 1.5rem;
		background-color: transparent;
		border: none;
		color: var(--c-text);
		background-color: var(--c-bg);

		&::placeholder {
			color: var(--c-placeholder);
		}
	}
</style>
