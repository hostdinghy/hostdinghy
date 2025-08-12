<script>
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';

	let { children } = $props();
</script>

<div class="main">
	<Header />

	{@render children()}

	<Footer />
</div>

<style lang="scss">
	.main {
		display: grid;
		grid-template-rows: auto 1fr auto;
		min-height: 100vh;
	}
</style>
