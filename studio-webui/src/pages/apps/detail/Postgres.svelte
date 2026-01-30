<script module lang="ts">
	export async function loadProps({ app }: AppLayoutProps) {
		return {
			props: await loadPostgresProps(app.server.id),
		};
	}
</script>

<script lang="ts">
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import Postgres, {
		loadPostgresProps,
	} from '../../servers/comps/Postgres.svelte';

	let { app, props }: AppLayoutProps<typeof loadProps> = $props();
</script>

<Postgres server={app.server} {...props} />
