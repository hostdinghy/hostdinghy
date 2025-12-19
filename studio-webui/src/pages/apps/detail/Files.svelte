<script module lang="ts">
	export async function loadProps(
		{ app, path = '' }: { path: string; app: AppLayoutProps },
		route,
	) {
		function getRoot(path, separator = '/files/') {
			const idx = path.indexOf(separator);
			return idx === -1
				? path
				: path.slice(0, idx + separator.length - 1);
		}

		const cwd = new FsDir(path, [
			{
				url: getRoot(route.req.url.pathname),
				label: app.name,
			},
		]);
		return {
			cwd,
		};
	}

	export class FsDir {
		content: (
			| {
					name: string;
					type: string;
					size: number;
					mime: string;
					permissions: string;
					owner: number;
					group: number;
					modified: Date;
			  }
			| {
					name: string;
					type: string;
					permissions: string;
					owner: number;
					group: number;
					modified: Date;
					size?: undefined;
					mime?: undefined;
			  }
		)[];
		crumbs: any;

		constructor(path, rootSegment) {
			this.content = [
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
			];

			this.path = path;

			this.crumbs = path
				.split('/')
				.filter(Boolean)
				.reduce(
					(acc, label) => [
						...acc,
						{
							url: acc.at(-1)?.url + '/' + label,
							label,
						},
					],
					rootSegment,
				);
		}

		getAbsoluteUrl() {
			return this.crumbs[this.crumbs.length - 1].url;
		}

		getParentSegment() {
			if (this.crumbs.length < 2) {
				return null;
			}
			return this.crumbs[this.crumbs.length - 2];
		}

		isRoot() {
			return this.crumbs.length === 1;
		}
	}
</script>

<script lang="ts">
	import Crumbs from '@/components/Crumbs.svelte';
	import DirectoryContent from '@/components/fs/DirectoryContent.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';

	let { app, cwd }: AppLayoutProps = $props();
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

{#key cwd}
	<header>
		<Crumbs breadcrumbs={cwd.crumbs} />
	</header>

	<DirectoryContent directory={cwd} />
{/key}

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
