<script module>
	import { App, byId } from '@/api/apps';

	export async function loadProps({ id }) {
		return {
			app: await byId(id),
		};
	}
</script>

<script lang="ts">
	let { id, app }: { id: string; app: App } = $props();
</script>

<svelte:head>
	<title>Huus</title>
</svelte:head>

<h1>{app.name}</h1>
