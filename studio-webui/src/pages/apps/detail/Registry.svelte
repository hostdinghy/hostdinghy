<script module lang="ts">
	export async function loadProps({ app }: AppLayoutProps) {
		return {
			props: await loadRegistryProps(app.server.id),
		};
	}
</script>

<script lang="ts">
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import Registry, {
		loadRegistryProps,
	} from '../../servers/comps/Registry.svelte';

	let { app, props }: AppLayoutProps<typeof loadProps> = $props();
</script>

<Registry server={app.server} {...props} />
