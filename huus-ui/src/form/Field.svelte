<script lang="ts">
	import type { Snippet } from 'svelte';

	const {
		id,
		label,
		children,
	}: { id: string; label: string; children: Snippet } = $props();
</script>

<div class="field">
	<label for={id}>{label}</label>

	<div class="ctn">
		{@render children()}
	</div>
</div>

<style lang="scss">
	.field {
		display: flex;
		min-height: 2.6rem;
		border: 1px solid var(--border-blue);
	}

	.field :global(+ .field) {
		border-top: none;
	}

	label {
		display: flex;
		width: 10rem;
		padding: 0.1rem 0.8rem;
		align-items: center;
		color: var(--label-blue);
		border-right: 1px solid var(--border-blue);
	}
</style>
