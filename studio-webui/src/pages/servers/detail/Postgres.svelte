<script module lang="ts">
	export async function loadProps({ server }: ServerLayoutProps) {
		return {
			props: await loadPostgresProps(server.id),
		};
	}
</script>

<script lang="ts">
	import type { ServerLayoutProps } from '@/layout/ServerLayout.svelte';
	import Postgres, { loadPostgresProps } from '../comps/Postgres.svelte';

	let { server, props }: ServerLayoutProps<typeof loadProps> = $props();
</script>

<Postgres {server} {...props} />
