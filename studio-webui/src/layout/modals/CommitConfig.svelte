<script>
	import Button from '@/components/Button.svelte';
	import DiffViewer from '@/components/DiffViewer.svelte';
	import Modal from '@/components/modal/Modal.svelte';

	let { open, original, modified, oncommit, onreset } = $props();
</script>

<Modal
	{open}
	title="Commit changes"
	class="commit-config"
	size="fill-screen"
	onclose={onreset}
>
	<DiffViewer {original} {modified} />

	<footer>
		<Button onclick={onreset}>Reset</Button>
		<Button onclick={oncommit}>Commit</Button>
	</footer>
</Modal>

<style lang="scss">
	:global {
		// not really a fan of global fully global styles in a
		// component...
		.commit-config.modal {
			display: grid;
			grid-template-rows: auto 1fr auto;

			header {
				border-bottom: 1px solid var(--c-border);
			}
		}
	}

	footer {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		justify-content: space-between;
		border-top: 1px solid var(--c-border);
	}
</style>
