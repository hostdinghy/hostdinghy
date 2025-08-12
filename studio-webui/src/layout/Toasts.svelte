<script module lang="ts">
	export type Toast = {
		message: string;
		status: ToastStatus;
	};

	export type ToastStatus = 'success' | 'error';

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
</script>

<script lang="ts">
	let toasts = $state<({ ref: ToastRef } & Toast)[]>([]);
	let counter = 0;

	function init() {
		if (import.meta.env.SSR) return;

		// @ts-ignore
		window.NEW_TOAST = d => {
			const id = (counter = counter + 1);
			const ref: ToastRef = {
				id,
				update: obj => {
					const toast = toasts.findIndex(t => t.ref.id === id);
					if (toast === -1) return;

					if (typeof obj.message === 'string') {
						toasts[toast].message = obj.message;
					}

					if (typeof obj.status === 'string') {
						toasts[toast].status = obj.status;
					}
				},
				remove: () => {
					toasts = toasts.filter(t => t.ref.id !== id);
				},
			};

			toasts = [...toasts, { ref, ...d }];

			return ref;
		};
	}
	init();
</script>

{#if toasts.length}
	<div class="toasts">
		<!-- todo maybe can i use toast.ref? as key -->
		{#each toasts as toast, i (i)}
			<div class="toast status-{toast.status}">
				<p>
					{toast.message}
				</p>

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
		z-index: 1000;
		max-width: 38rem;
		width: 100%;
		flex-direction: column;
		gap: 1rem;
		transform: translateX(-50%);
		padding: 0 var(--sx-body);
	}

	.toast {
		display: flex;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;

		&.status-success {
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
