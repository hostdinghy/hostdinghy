<script>
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
</script>

<div class="main">
	<Header />

	<slot />

	<Footer />
</div>

<style lang="scss">
	.main {
		display: grid;
		grid-template-rows: auto 1fr auto;
		min-height: 100vh;
	}
</style>
