<script>
	import { derivedMode } from '@/lib/theme/themeMode';
	import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { get } from 'svelte/store';

	let { original, modified } = $props();

	const originalModel = monaco.editor.createModel('', 'yaml');
	const modifiedModel = monaco.editor.createModel('', 'yaml');

	$effect(() => {
		originalModel.setValue(original);
		modifiedModel.setValue(modified);
	});

	const themes = {
		light: 'dinghy-light',
		dark: 'dinghy-dark',
	};
</script>

<div
	class="editor"
	{@attach el => {
		const diffEditor = monaco.editor.createDiffEditor(el, {
			// You can optionally disable the resizing
			enableSplitViewResizing: false,
			renderSideBySide: true,
			theme: themes[get(derivedMode)],
			readOnly: true,
		});
		diffEditor.setModel({
			original: originalModel,
			modified: modifiedModel,
		});

		return () => {
			diffEditor.dispose();
		};
	}}
></div>

<style>
	.editor {
		min-height: 200px;
		width: 100%;
	}
</style>
