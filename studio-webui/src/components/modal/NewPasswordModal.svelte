<script lang="ts">
	import Modal from './Modal.svelte';

	let { title, password = $bindable() }: { title: string; password: string } =
		$props();

	function onclose() {
		password = '';
	}
</script>

<Modal open={!!password} {title} size="small" headerBb {onclose}>
	<div class="new-password">
		<p>
			<strong>Your new password:</strong>
			<span class="pw">{password}</span>
		</p>

		<p>!This will only be display once!</p>
	</div>
</Modal>

<style lang="scss">
	.new-password {
		padding: 1rem;

		strong {
			display: block;
			margin-bottom: 0.3rem;
			color: var(--c-label);
		}

		.pw {
			word-break: break-all;
		}

		p:not(:last-child) {
			margin-bottom: 1.5rem;
		}
	}
</style>
