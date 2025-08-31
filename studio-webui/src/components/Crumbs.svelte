<script lang="ts">
	type Props = {
		breadcrumbs: { url: string; label: string }[];
	};
	let { breadcrumbs }: Props = $props();
</script>

<div class="crumbs">
	{#each breadcrumbs as breadcrumb (breadcrumb.url)}
		<a href={breadcrumb.url}>
			<span class="hover:underline">
				{breadcrumb.label}
			</span>
		</a>
	{/each}
</div>

<style lang="scss">
	.crumbs {
		display: flex;
		overflow-x: auto;
		flex: 1;
		a + a:before {
			content: '/';
			padding-left: 0.5rem;
			padding-right: 0.5rem;
			opacity: 0.4;
		}
	}
</style>
