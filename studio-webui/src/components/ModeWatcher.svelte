<script>
	import { systemPrefersMode } from '@/lib/theme/themeMode';
	import { onMount } from 'svelte';

	onMount(() => {
		systemPrefersMode.query();
	});
</script>
