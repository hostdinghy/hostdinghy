<script lang="ts">
	import Crumbs from '@/components/Crumbs.svelte';
	import DirectoryContent from '@/components/fs/DirectoryContent.svelte';
	import Status from '@/components/Status.svelte';
	import Table from '@/components/Table.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';

	let { app }: AppLayoutProps = $props();

	const cwd = {
		path: `${app.name}/craft/web/assets`
			.split('/')
			.map(s => ({ url: '#' + s, label: s })),
		children: [
			{
				name: 'file1.txt',
				type: 'file',
				size: 1234,
				mime: 'text/plain',
				permissions: 'rw-r--r--',
				owner: 1000,
				group: 1000,
				modified: new Date('2025-08-30T09:00:00Z'),
			},
			{
				name: 'subdir',
				type: 'directory',
				permissions: 'rwxr-xr-x',
				owner: 1000,
				group: 1000,
				modified: new Date('2025-08-29T18:00:00Z'),
			},
		],
	};
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<header>
	<Crumbs breadcrumbs={cwd.path} />
</header>

<DirectoryContent directory={cwd} />

<style lang="scss">
	header {
		padding: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;

		border-bottom: 1px solid var(--c-border);
	}
	h1 {
		font-size: 1.125rem;

		.id,
		.server {
			color: var(--c-label);
		}

		.server {
			font-size: 0.9rem;
		}
	}

	.msg {
		padding: 1rem;
		color: rgb(from var(--c-text) r g b / 70%);
		.underline {
			color: var(--c-text);
		}
	}

	.status {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}
</style>
