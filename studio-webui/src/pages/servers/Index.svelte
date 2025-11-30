<script module lang="ts">
	import { loadServers } from '@/api/servers';
	import Button from '@/components/Button.svelte';
	import Table from '@/components/Table.svelte';
	import type LoadProps from '@/lib/LoadProps';
	import type { ResolvedProps } from '@/lib/LoadProps';

	export async function loadProps(_props: any, _lp: LoadProps) {
		return {
			servers: await loadServers(),
		};
	}
</script>

<script lang="ts">
	let { servers }: ResolvedProps<typeof loadProps> = $props();
</script>

<div id="servers">
	<Table
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'domain', value: 'Domain' },
		]}
		rows={servers.all()}
	>
		{#snippet toolbar()}
			<h1>Servers</h1>
			<Button href="/servers/create">add</Button>
		{/snippet}
	</Table>
</div>

<style lang="scss">
	// todo this is not a good solution
	#servers :global(.pre-header) {
		border-top: none;
	}

	h1 {
		padding: 1rem;
		flex: 1;
		font-size: 1.125rem;
	}
</style>
