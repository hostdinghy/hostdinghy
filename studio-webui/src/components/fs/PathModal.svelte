<script lang="ts">
	import Button from '@/components/Button.svelte';
	import Modal from '@/components/modal/Modal.svelte';
	import Input from '@/form/Input.svelte';

	/** asks for a path or name, e.g. for new folder, rename, copy */
	let {
		open,
		title,
		label,
		value: initial,
		help = '',
		submit,
		onsubmit,
		onclose,
	}: {
		open: boolean;
		title: string;
		label: string;
		value: string;
		help?: string;
		submit: string;
		/** throws to show an error */
		onsubmit: (value: string) => Promise<void>;
		onclose: () => void;
	} = $props();

	let value = $state('');
	let error = $state('');
	let busy = $state(false);

	// reset when opened
	$effect(() => {
		if (open) {
			value = initial;
			error = '';
		}
	});

	async function onformsubmit(e: Event) {
		e.preventDefault();
		if (busy) return;

		busy = true;
		error = '';
		try {
			await onsubmit(value.trim());
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}
</script>

<Modal {open} {title} size="medium" {onclose}>
	<form onsubmit={onformsubmit}>
		<Input
			id="fs-path"
			name="fs-path"
			type="text"
			{label}
			bind:value
			bx={false}
			required
			autofocus
		/>

		{#if help}
			<p class="help">{help}</p>
		{/if}

		{#if error}
			<p class="error">{error}</p>
		{/if}

		<div class="btns">
			<Button disabled={busy}>{busy ? 'working...' : submit}</Button>
		</div>
	</form>
</Modal>

<style lang="scss">
	.help {
		padding: 1rem 1rem 0;
		color: rgb(from var(--c-text) r g b / 70%);
		font-size: 0.875rem;
	}

	.error {
		padding: 1rem 1rem 0;
		color: var(--red);
	}

	.btns {
		display: flex;
		padding: 1rem;
	}
</style>
