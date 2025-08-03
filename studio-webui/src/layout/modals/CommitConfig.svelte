<script>
	import Button from '@/components/Button.svelte';
	import DiffViewer from '@/components/DiffViewer.svelte';
	import Modal from '@/components/Modal.svelte';

	let {
		open = $bindable(),
		original = $bindable(),
		modified = $bindable(),
		oncommit,
		onreset,
	} = $props();
</script>

<Modal bind:open>
	<header>
		<h2>Commit Changes</h2>
		<Button onclick={() => (open = false)}>&times;</Button>
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

<style>
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
