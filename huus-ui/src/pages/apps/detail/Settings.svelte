<script lang="ts">
	import Editor from '@/components/Editor.svelte';
	import type { loadProps } from '@/layout/AppLayout.svelte';
	import type { ResolvedProps } from '@/lib/LoadProps';

	let { app }: ResolvedProps<typeof loadProps> = $props();
</script>

<svelte:head>
	<title>Huus</title>
</svelte:head>

<div class="settings">
	<Editor />
</div>

<style>
	.settings {
		flex: 1;
	}
</style>
