import * as monaco from 'monaco-editor';
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import CssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import HtmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import TsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
import * as editorDark from './theme/editor-dark.json';
import * as editorLight from './theme/editor-light.json';

// the languages with a language service need their worker, yaml and the
// other basic languages only use the editor worker
self.MonacoEnvironment = {
	getWorker(_workerId: string, label: string): Worker {
		switch (label) {
			case 'json':
				return new JsonWorker();
			case 'css':
			case 'scss':
			case 'less':
				return new CssWorker();
			case 'html':
			case 'handlebars':
			case 'razor':
				return new HtmlWorker();
			case 'typescript':
			case 'javascript':
				return new TsWorker();
			default:
				return new EditorWorker();
		}
	},
};

monaco.editor.defineTheme(
	'dinghy-light',
	editorLight as monaco.editor.IStandaloneThemeData,
);
monaco.editor.defineTheme(
	'dinghy-dark',
	editorDark as monaco.editor.IStandaloneThemeData,
);
