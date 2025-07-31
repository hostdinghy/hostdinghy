<script lang="ts">
	import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { onDestroy, onMount } from 'svelte';

	let monacoEl: HTMLDivElement;
	let editor: monaco.editor.IStandaloneCodeEditor;

	const configFromServer = `services:
  frontend:
    image: traefik:2.6
    command: --providers.docker --entrypoints.web.address=:80 --providers.docker.exposedbydefault=false
    ports:
      # The HTTP port
      - "80:80"
    volumes:
      # So that Traefik can listen to the Docker events
      - /var/run/docker.sock:/var/run/docker.sock
    depends_on:
      - backend
  backend:
    build: backend
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.go.rule=Path(\`/\`)"
      - "traefik.http.services.go.loadbalancer.server.port=80"
`;

	onMount(() => {
		monaco.languages.register({ id: 'yaml' });
		editor = monaco.editor.create(monacoEl, {
			value: configFromServer,
			language: 'yaml',
			theme: 'vs',
		});
	});

	onDestroy(() => {
		editor.dispose();
	});
</script>

<div class="editor" bind:this={monacoEl}></div>

<style>
	.editor {
		width: 100%;
		height: 100%;
	}
</style>
