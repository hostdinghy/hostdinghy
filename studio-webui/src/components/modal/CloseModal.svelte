<script lang="ts">
	import Close from '@/assets/icons/Close.svelte';
	import Button from '../Button.svelte';

	let { onclick }: { onclick: () => void } = $props();
</script>

<Button {onclick} title="close modal" aria-label="close modal">
	<Close />
</Button>
