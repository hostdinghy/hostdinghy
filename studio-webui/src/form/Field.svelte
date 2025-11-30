<script lang="ts">
	import type { Snippet } from 'svelte';

	const {
		id,
		label,
		children,
		bx = true,
	}: {
		id: string;
		label: string;
		children: Snippet;
		bx?: boolean;
	} = $props();
</script>

<div class="field" class:bx>
	<label for={id}>{label}</label>

	<div class="ctn">
		{@render children()}
	</div>
</div>

<style lang="scss">
	.field {
		display: flex;
		min-height: 2.6rem;
		border: 1px solid var(--c-border);

		&:not(:first-child) {
			border-top: none;
		}

		&:not(.bx) {
			border-left: none;
			border-right: none;
		}
	}

	label {
		display: flex;
		width: 10rem;
		padding: 0.1rem 0.8rem;
		align-items: center;
		color: var(--c-label);
		border-right: 1px solid var(--c-border);
	}

	.ctn {
		flex: 1;
	}
</style>
