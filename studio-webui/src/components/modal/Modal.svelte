<script lang="ts">
	import { type Snippet } from 'svelte';

	let {
		open,
		// not bindable, because almost always you wan't to do some
		// cleanup when closing the modal
		// so this forces you to think about that
		children,
		onclose,
		/** if the modal should fill the screen */
		fillScreen = false,
		class: cls,
		...rest
	}: {
		open: boolean;
		children: Snippet;
		onclose: () => void;
		fillScreen?: boolean;
		class?: string;
		[key: string]: any;
	} = $props();

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}
</script>

<svelte:window on:keydown={onkeydown} />

{#if open}
	<div class="modal-layer">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div onclick={onclose} class="background"></div>

		<div class="wrap">
			<div class="modal {cls}" class:fill-screen={fillScreen} {...rest}>
				{@render children()}
			</div>
		</div>
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
	}
</style>
