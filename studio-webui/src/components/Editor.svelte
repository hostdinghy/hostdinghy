<script lang="ts">
	import { derivedMode } from '@/lib/theme/themeMode';
	import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { get } from 'svelte/store';

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
</script>

<div
	class="editor"
	{@attach monacoEl => {
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
				const configContent = editor.getValue();
				onsave(configContent);
			},
		});

		return () => {
			editor.dispose();
		};
	}}
></div>

<style>
	.editor {
		flex: 1;
	}
</style>
