<script lang="ts">
	import { onDestroy, type Snippet } from 'svelte';
	import { on } from 'svelte/events';

	let {
		open = $bindable(),
		/** if the modal should fill the screen */
		fillScreen = false,
		children,
		class: cls,
		onclose = () => (open = false),
		...rest
	}: {
		open: boolean;
		fillScreen?: boolean;
		children: Snippet;
		class?: string;
		onclose?: () => void;
		[key: string]: any;
	} = $props();

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
			<div class="modal {cls}" class:fill-screen={fillScreen} {...rest}>
				{@render children()}
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div onclick={onclose} class="background"></div>
	</div>
{/if}

<style lang="scss">
	.modal-layer {
		position: fixed;
		inset: 0;
		// see app.scss (z-index map)
		z-index: 1000;
		overflow-y: auto;
	}

	.wrap {
		display: flex;
		min-height: 100%;
		align-items: center;
		justify-content: center;
	}

	.modal {
		border: 1px solid var(--c-border);
		background: var(--c-bg);
		z-index: 1;
		overflow: hidden;

		&.fill-screen {
			width: 100%;
			min-height: 80vh;
		}
	}

	.background {
		position: absolute;
		inset: 0;
		background: var(--c-bg);
		opacity: 0.8;
		z-index: 0;
	}
</style>
