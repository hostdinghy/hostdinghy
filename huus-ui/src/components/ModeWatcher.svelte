<script>
	import { systemPrefersMode } from '@/lib/theme/themeMode.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		systemPrefersMode.query();
	});
</script>
