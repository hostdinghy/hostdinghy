<script>
	import Button from '@/components/Button.svelte';
	import DiffViewer from '@/components/DiffViewer.svelte';
	import Modal from '@/components/modal/Modal.svelte';
	import CloseModal from '@/components/modal/CloseModal.svelte';

	let {
		open = $bindable(),
		original = $bindable(),
		modified = $bindable(),
		oncommit,
		onreset,
	} = $props();
</script>

<Modal bind:open class="commit-config" fillScreen>
	<header>
		<h2>Commit Changes</h2>
		<CloseModal onclick={() => (open = false)} />
	</header>

	<DiffViewer bind:original bind:modified />

	<footer>
		<Button
			onclick={() => {
				open = false;
				onreset();
			}}
		>
			Reset
		</Button>
		<Button
			onclick={() => {
				open = false;
				oncommit();
			}}
		>
			Commit
		</Button>
	</footer>
</Modal>

<style lang="scss">
	:global {
		// not really a fan of global fully global styles in a
		// component...
		.commit-config.modal {
			display: grid;
			grid-template-rows: auto 1fr auto;
		}
	}

	header {
		padding: 1rem;
		border-bottom: 1px solid var(--c-border);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	h2 {
		font-size: 1.25rem;
	}

	footer {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		justify-content: space-between;
		border-top: 1px solid var(--c-border);
	}
</style>
