<script lang="ts">
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';

	let { app }: AppLayoutProps = $props();
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

Logs for {app.name}

<style></style>
