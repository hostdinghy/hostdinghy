<script lang="ts">
	import type { Writable } from 'chuchi/stores';

	let { loading }: { loading: Writable<number> } = $props();

	let prog = $state(0);
	let visible = $state(false);

	let prevLoading = 0;
	let hideTim: any = null;
	$effect(() => {
		const ld = $loading;

		// new prog
		if (prevLoading < ld) {
			prog = 0.8;
			visible = true;
		} else if (prevLoading !== 0 && ld == 0) {
			prog = 1;
			visible = false;
			if (hideTim) clearTimeout(hideTim);
			hideTim = setTimeout(() => (prog = 0), 500);
		}

		prevLoading = ld;
	});
</script>

<div class="pageloader" class:visible style:--prog={prog}></div>

<style lang="scss">
	.pageloader {
		position: fixed;
		top: 0;
		left: 0;
		height: 3px;
		width: 100%;
		transform: translateY(calc(-100% - 1px));
		transition: transform 0.5s ease;

		&::before {
			content: '';
			display: block;
			width: 100%;
			height: 100%;
			transform: scaleX(var(--prog, 0));
			transform-origin: left;
			background-color: var(--c-accent);
			transition: transform 0.5s ease;
		}

		&.visible {
			transform: translateY(0%);
		}
	}
</style>
