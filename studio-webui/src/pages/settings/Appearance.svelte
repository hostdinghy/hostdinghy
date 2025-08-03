<script>
	import Input from '@/form/Input.svelte';
	import Select from '@/form/Select.svelte';
	import { userPrefersMode } from '@/lib/theme/themeMode';

	const themeModes = {
		system: 'System',
		dark: 'Dark',
		light: 'Light',
	};

	// let themeMode = $state({
	// 	key: userPrefersMode.get(),
	// 	value: themeModes[userPrefersMode.get()],
	// });

	let themeMode = $state(userPrefersMode.get());

	$effect(() => {
		if (themeMode) {
			userPrefersMode.set(themeMode);
		}
	});
</script>

<Select
	label="Theme Mode"
	id="theme-mode"
	name="theme-mode"
	bind:value={themeMode}
	options={Object.entries(themeModes).map(([key, value]) => ({
		key,
		value,
	}))}
></Select>
