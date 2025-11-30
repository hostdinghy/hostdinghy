<script lang="ts">
	import DescriptionList from '@/components/DescriptionList.svelte';
	import Header from '@/components/Header.svelte';
	import type { ServerLayoutProps } from '@/layout/ServerLayout.svelte';

	let { server }: ServerLayoutProps = $props();
</script>

<div class="server">
	<Header bb>
		<h1>{server.name}</h1>
	</Header>

	<div class="detail">
		<DescriptionList
			list={{
				Domain: server.domain,
				Version: server.version ?? '-',
			}}
		/>
	</div>
</div>

<style lang="scss">
	.detail {
		padding: 1rem;
	}
</style>
