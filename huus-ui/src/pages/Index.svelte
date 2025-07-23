<script module>
	import { all, App } from '@/api/apps';

	export async function loadProps() {
		return {
			apps: await all(),
		};
	}
</script>

<script lang="ts">
	let { apps }: { apps: App[] } = $props();
</script>

<svelte:head>
	<title>Huus</title>
</svelte:head>

<div class="wrap">
	<ul>
		{#each apps as app}
			<li>
				<a href="/apps/{app.id}">
					{app.name}
				</a>
			</li>
		{/each}
	</ul>
</div>
