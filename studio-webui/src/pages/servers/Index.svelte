<script module lang="ts">
	import { loadServers } from '@/api/servers';
	import Add from '@/assets/icons/Add.svelte';
	import Button from '@/components/Button.svelte';
	import Table from '@/components/table/Table.svelte';
	import type LoadProps from '@/lib/LoadProps';
	import type { ResolvedProps } from '@/lib/LoadProps';

	export async function loadProps(_props: any, _lp: LoadProps) {
		return {
			servers: await loadServers(),
		};
	}
</script>

<script lang="ts">
	import Header from '@/components/Header.svelte';

	let { servers }: ResolvedProps<typeof loadProps> = $props();
</script>

<div id="servers">
	<Header>
		<h1>Servers</h1>

		<Button
			href="/servers/create"
			title="add server"
			aria-label="add server"
		>
			<Add />
		</Button>
	</Header>

	<Table
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'domain', value: 'Domain' },
		]}
		rows={servers.all()}
		bx={false}
	>
		{#snippet name(row)}
			<td>
				<a class="underline" href="/servers/{row.id}">{row.name}</a>
			</td>
		{/snippet}
	</Table>
</div>
