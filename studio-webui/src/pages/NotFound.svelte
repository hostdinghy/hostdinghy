<script>
	import Button from '@/components/Button.svelte';
</script>

<div class="notfound wrap">
	<h1>Not Found</h1>

	<Button href="/">Go to Home</Button>
</div>

<style lang="scss">
	.notfound {
		padding-top: 4rem;
	}

	h1 {
		font-size: 1.125rem;
		margin-bottom: 1rem;
	}

	.notfound :global .btn {
		width: fit-content;
	}
</style>
