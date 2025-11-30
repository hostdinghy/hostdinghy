<script module lang="ts">
	export async function loadProps({ server }: ServerLayoutProps) {
		return {
			props: await loadRegistryProps(server.id),
		};
	}
</script>

<script lang="ts">
	import type { ServerLayoutProps } from '@/layout/ServerLayout.svelte';
	import Registry, { loadRegistryProps } from '../comps/Registry.svelte';

	let { server, props }: ServerLayoutProps<typeof loadProps> = $props();
</script>

<Registry {server} {...props} />
