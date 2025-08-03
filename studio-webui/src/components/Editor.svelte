<script lang="ts">
	import { derivedMode } from '@/lib/theme/themeMode.svelte';
	import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { onDestroy, onMount } from 'svelte';
	import { get } from 'svelte/store';

	let monacoEl: HTMLDivElement;
	let editor: monaco.editor.IStandaloneCodeEditor;

	let { value, onsave } = $props();

	export function save() {
		onsave(editor.getValue());
	}

	export function setValue(v) {
		editor.setValue(v);
	}

	const themes = {
		light: 'dinghy-light',
		dark: 'dinghy-dark',
	};

	onMount(() => {
		monaco.languages.register({ id: 'yaml' });
		editor = monaco.editor.create(monacoEl, {
			value,
			language: 'yaml',
			theme: themes[get(derivedMode)],
		});

		editor.addAction({
			id: 'save-config',
			label: 'Save Config',
			keybindings: [
				monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, // Bind to Ctrl+S
			],
			run: function (editor) {
				// Get the content of the editor
				const configContent = editor.getValue();

				// Call the POST API to save the config
				// saveConfig(configContent);
				onsave(configContent);
			},
		});
	});

	onDestroy(() => {
		editor.dispose();
	});
</script>

<div class="editor" bind:this={monacoEl}></div>

<style>
	.editor {
		flex: 1;
	}
</style>
