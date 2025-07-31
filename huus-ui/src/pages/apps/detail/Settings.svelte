<script lang="ts">
	import type { loadProps } from '@/layout/AppLayout.svelte';
	import type { ResolvedProps } from '@/lib/LoadProps';

	let { app }: ResolvedProps<typeof loadProps> = $props();
</script>

<svelte:head>
	<title>Huus</title>
</svelte:head>

settings for {app.name}

<style></style>
