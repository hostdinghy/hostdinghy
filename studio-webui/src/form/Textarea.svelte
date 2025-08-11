<script lang="ts">
	import Field from './Field.svelte';

	let {
		id,
		name,
		label,
		value = $bindable(),
		...rest
	}: {
		id: string;
		name: string;
		label: string;
		value: string;
		[rest: string]: any;
	} = $props();
</script>

<Field {id} {label}>
	<textarea {id} {name} bind:value {...rest}></textarea>
</Field>

<style lang="scss">
	textarea {
		width: 100%;
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
