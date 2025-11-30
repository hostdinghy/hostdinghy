<script>
	import Header from '@/components/Header.svelte';
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

<div class="appearance">
	<Header>
		<h1>Appearance</h1>
	</Header>

	<form onsubmit={e => e.preventDefault()}>
		<Select
			label="Theme Mode"
			id="theme-mode"
			name="theme-mode"
			bind:value={themeMode}
			options={Object.entries(themeModes).map(([key, value]) => ({
				key,
				value,
			}))}
			bx={false}
		></Select>
	</form>
</div>
