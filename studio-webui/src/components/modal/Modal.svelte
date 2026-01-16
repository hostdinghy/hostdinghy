<script module lang="ts">
	export type ModalSize = undefined | 'fill-screen' | 'small' | 'medium';
</script>

<script lang="ts">
	import { type Snippet } from 'svelte';
	import Button from '../Button.svelte';
	import Close from '@/assets/icons/Close.svelte';
	import { disableScroll, enableScroll } from '@/lib/body';

	let {
		// not bindable, because almost always you wan't to do some
		// cleanup when closing the modal
		// so this forces you to think about that
		open,
		title,
		children,
		onclose,
		size = undefined,
		headerBb = false,
		class: cls,
		...rest
	}: {
		open: boolean;
		title?: string;
		children: Snippet;
		onclose: () => void;
		fillScreen?: boolean;
		size?: ModalSize;
		/** header border bottom */
		headerBb?: boolean;
		class?: string;
		[key: string]: any;
	} = $props();

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}

	function oncloseclick(e: MouseEvent) {
		// i dont really like that we have to use two onclick handlers
		// but using pointer-events: none is not much nicer
		if (e.target === e.currentTarget) onclose();
	}

	$effect(() => {
		if (open) disableScroll();
		else enableScroll();
	});
</script>

<svelte:window on:keydown={onkeydown} />

{#if open}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-layer" onclick={oncloseclick}>
		<div class="wrap" onclick={oncloseclick}>
			<div class="modal {cls} size-{size ?? ''}" {...rest}>
				{#if title}
					<header class:bb={headerBb}>
						<h2>{title}</h2>

						<Button
							onclick={onclose}
							title="close modal"
							aria-label="close modal"
						>
							<Close />
						</Button>
					</header>
				{/if}

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
		background-color: rgb(from var(--c-bg) r g b / 80%);
	}

	.wrap {
		display: flex;
		min-height: 100%;
		align-items: center;
		justify-content: center;
	}

	.modal {
		position: relative;
		border: 1px solid var(--c-border);
		background: var(--c-bg);
		overflow: hidden;

		&.size-fill-screen {
			width: 100%;
			min-height: 80vh;
		}

		&.size-small {
			width: 100%;
			max-width: 28rem;
		}

		&.size-medium {
			width: 100%;
			max-width: 38rem;
		}
	}

	header {
		display: flex;
		padding: 1rem;
		justify-content: space-between;
		align-items: center;

		&.bb {
			border-bottom: 1px solid var(--c-border);
		}

		h2 {
			font-size: 1.125rem;
		}
	}
</style>
