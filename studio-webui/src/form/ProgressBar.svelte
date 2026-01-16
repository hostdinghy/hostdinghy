<script lang="ts">
	let {
		progress = 0,
		mt = false,
	}: {
		/** 0-100 */
		progress: number;
		/** margin top */
		mt?: boolean;
	} = $props();
</script>

{#if progress > 0}
	<div class="progress" class:mt style:--prog={progress} aria-hidden="true">
		<!-- <span class="label">{Math.round(progress)}%</span> -->
	</div>
{/if}

<style lang="scss">
	.progress {
		position: relative;
		/* padding: 0.1rem 0; */
		height: 0.75rem;
		text-align: center;

		&::before {
			content: '';
			position: absolute;
			display: block;
			inset: 0;
			background-color: var(--c-accent);
			width: 100%;
			height: 100%;
			transform: scaleX(calc(var(--prog) / 100));
			transform-origin: left;
			transition: transform 0.1s ease;
		}
	}

	/* .label {
		position: relative;
		color: white;
		mix-blend-mode: difference;
		opacity: 0.5;
		font-size: 0.9rem;
	} */

	.mt {
		margin-top: 1.5rem;
	}
</style>
