<script>
	import { onDestroy } from 'svelte';
	import { on } from 'svelte/events';

	let { open = $bindable(), children } = $props();

	const removeHandler = on(window, 'keydown', ({ key }) => {
		if (key === 'Escape') {
			open = false;
		}
	});

	onDestroy(() => {
		removeHandler();
	});
</script>

{#if open}
	<div class="modal-layer">
		<div class="wrap">
			<div class="modal">
				{@render children()}
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div onclick={() => (open = false)} class="background"></div>
	</div>
{/if}

<style lang="scss">
	.modal-layer {
		z-index: 1000;
		position: fixed;
		inset: 0;
		display: flex;
	}

	.wrap {
		z-index: 1;
		display: flex;
		flex: 1;
		height: 100%;
		align-items: center;
		justify-content: center;
	}
	.modal {
		flex: 1;
		border: 1px solid var(--c-border);
		background: var(--c-bg);
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		max-height: 80vh;
	}

	.background {
		position: absolute;
		z-index: 0;
		inset: 0;
		background: var(--c-bg);
		opacity: 0.8;
	}
</style>
