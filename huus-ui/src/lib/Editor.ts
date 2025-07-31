import * as monaco from 'monaco-editor';

// @ts-ignore
self.MonacoEnvironment = {
	getWorker: function (workerId, label) {
		const getWorkerModule = (moduleUrl, label) => {
			return new Worker(self.MonacoEnvironment.getWorkerUrl(moduleUrl), {
				name: label,
				type: 'module',
			});
		};

		switch (label) {
			case 'yaml':
				return getWorkerModule(
					'/monaco-editor/esm/vs/language/yaml/yaml.worker?worker',
					label,
				);
			default:
				return getWorkerModule(
					'/monaco-editor/esm/vs/editor/editor.worker?worker',
					label,
				);
		}
	},
};

// monaco.languages.typescript.typescriptDefaults.setEagerModelSync(true);
