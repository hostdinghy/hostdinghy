<script lang="ts">
	import DescriptionList from '@/components/DescriptionList.svelte';
	import type { ServerLayoutProps } from '@/layout/ServerLayout.svelte';

	let { server }: ServerLayoutProps = $props();
</script>

<div class="detail">
	<DescriptionList
		list={{
			Name: server.name,
			Domain: server.domain,
			Version: server.version ?? '-',
		}}
	/>
</div>

<style lang="scss">
	.detail {
		padding: 1rem;
	}
</style>
