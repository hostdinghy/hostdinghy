<script lang="ts">
	import Button from '@/components/Button.svelte';
	import Editor from '@/components/Editor.svelte';
	import type { loadProps } from '@/layout/AppLayout.svelte';
	import CommitConfigModal from '@/layout/modals/CommitConfig.svelte';
	import type { ResolvedProps } from '@/lib/LoadProps';

	let { app }: ResolvedProps<typeof loadProps> = $props();

	const value = `services:
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

	let editor;
	let commitConfigOpen = $state(false);
	let original = $state(value);
	let modified = $state(value);

	function onsave(newValue) {
		modified = newValue;
		commitConfigOpen = true;
	}

	function oncommit() {
		original = modified;
		console.warn('todo: send config to backend');
	}

	function onreset() {
		modified = original;
		editor.setValue(original);
	}
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div class="settings">
	<header>
		<h1>Docker Compose</h1>
		<Button onclick={() => editor.save()}>save</Button>
	</header>
	<Editor {value} {onsave} bind:this={editor} />
</div>

<CommitConfigModal
	bind:open={commitConfigOpen}
	bind:original
	bind:modified
	{oncommit}
	{onreset}
/>

<style lang="scss">
	.settings {
		flex: 1;
		display: flex;
		flex-direction: column;
	}
	header {
		padding: 1rem;
		border-bottom: 1px solid var(--c-border);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	h1 {
		font-size: 1.125rem;
	}
</style>
