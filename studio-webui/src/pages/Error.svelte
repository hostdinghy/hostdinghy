<script>
	import Button from '@/components/Button.svelte';

	let { error } = $props();
</script>

<div class="cont">
	<div class="center">
		<h1>Something went wrong</h1>
		<p>{error.message}</p>

		{#if error.stack}
			<pre>{error.stack}</pre>
		{/if}
	</div>
</div>

<style lang="scss">
	.cont {
		display: flex;
		min-height: 100vh;
		width: 100%;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.center {
		max-width: 100%;
		padding: 2rem;
		background-color: var(--red);
	}

	h1 {
		font-size: 1.75rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
	}

	p {
		font-weight: 600;
	}

	pre {
		margin-top: 1rem;
	}
</style>
