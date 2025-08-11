<script lang="ts">
	import Field from './Field.svelte';

	let {
		id,
		name,
		type,
		label,
		value = $bindable(),
		...rest
	}: {
		id: string;
		name: string;
		type: 'text' | 'email' | 'password';
		label: string;
		value: string;
		[rest: string]: any;
	} = $props();
</script>

<Field {id} {label}>
	<input {type} {name} {id} bind:value {...rest} />
</Field>

<style lang="scss">
	input {
		width: 100%;
		min-height: 100%;
		padding: 0.5rem 1.5rem;
		background-color: transparent;
		border: none;
		color: var(--c-text);

		&::placeholder {
			color: var(--c-placeholder);
		}
	}
</style>
