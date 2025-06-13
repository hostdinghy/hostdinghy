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
	<input {type} {id} bind:value {...rest} />
</Field>

<style lang="scss">
	input {
		padding: 0.5rem 1.5rem;
		background-color: transparent;
		border: none;
		min-height: 100%;
		color: var(--white);

		&::placeholder {
			color: var(--placeholder-blue);
		}
	}
</style>
