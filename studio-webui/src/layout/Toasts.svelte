<script module lang="ts">
	import { onDestroy } from 'svelte';

	export type Toast = {
		message: string;
		status: ToastStatus;
	};

	export type ToastStatus = 'success' | 'info' | 'error';

	export type ToastRef = {
		id: number;
		update: (obj: { message?: string; status?: ToastStatus }) => void;
		remove: () => void;
	};

	export function toast(obj: Toast): ToastRef {
		if (import.meta.env.SSR) throw new Error('Cannot use toast in SSR');

		// @ts-ignore
		return window.NEW_TOAST(obj);
	}

	export type ToastHandler = {
		/** Shows a success toast message */
		success: (message: string) => void;
		/** Shows an info toast message */
		info: (message: string) => void;
		/** Shows an error toast message */
		error: (message: string) => void;
		/** Updates the current toast message */
		update: (message: string) => void;
		/** Removes the current toast message */
		remove: () => void;
	};

	/** Creates a possible toast instance useful in svelte components */
	export function createToastHandler(): ToastHandler {
		let toastRef: ToastRef | null = null;

		const destroy = () => {
			toastRef?.remove();
			toastRef = null;
		};
		onDestroy(destroy);

		const newToast = (message: string, status: ToastStatus) => {
			toastRef?.remove();
			toastRef = toast({ message, status });
		};

		return {
			success: (message: string) => newToast(message, 'success'),
			info: (message: string) => newToast(message, 'info'),
			error: (message: string) => newToast(message, 'error'),
			update: (message: string) => toastRef?.update({ message }),
			remove: () => destroy(),
		};
	}
</script>

<script lang="ts">
	let toasts = $state<({ ref: ToastRef } & Toast)[]>([]);
	let counter = 0;

	// @ts-ignore
	window.NEW_TOAST = d => {
		const id = ++counter;

		const ref: ToastRef = {
			id,
			update: obj => {
				// probably because of reactivity, taking the idx
				const idx = toasts.findIndex(t => t.ref.id === id);
				if (idx === -1) return;

				if (typeof obj.message === 'string') {
					toasts[idx].message = obj.message;
				}

				if (typeof obj.status === 'string') {
					toasts[idx].status = obj.status;
				}
			},
			remove: () => {
				toasts = toasts.filter(t => t.ref.id !== id);
			},
		};

		toasts = [...toasts, { ref, ...d }];

		return ref;
	};
</script>

{#if toasts.length}
	<div class="toasts">
		{#each toasts as toast (toast.ref.id)}
			<div class="toast status-{toast.status}">
				<p>{toast.message}</p>

				<button class="close" onclick={() => toast.ref.remove()}>
					Close
				</button>
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
	.toasts {
		position: fixed;
		display: flex;
		bottom: 1rem;
		left: 50%;
		max-width: 38rem;
		width: 100%;
		padding: 0 var(--sx-body);
		z-index: 1100;
		flex-direction: column;
		gap: 1rem;
		transform: translateX(-50%);
	}

	.toast {
		display: flex;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;

		&.status-success {
			background-color: var(--blue);
			color: var(--white);
		}

		&.status-info {
			// todo find a color for this
			background-color: var(--blue);
			color: var(--white);
		}

		&.status-error {
			background-color: var(--red);
			color: var(--white);
		}
	}

	.close {
		cursor: pointer;
	}
</style>
