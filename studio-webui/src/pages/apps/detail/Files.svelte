<script module lang="ts">
	import type { Crumb } from '@/components/Crumbs.svelte';

	export async function loadProps(
		{ app, path = '' }: { path: string } & AppLayoutProps,
		lp: LoadProps,
	) {
		const cwd = new FsDir(path, [
			{
				url: getRoot(lp.req.url.pathname),
				label: app.name,
			},
		]);

		return {
			cwd,
		};
	}

	function getRoot(path: string, separator = '/files/') {
		const idx = path.indexOf(separator);
		return idx === -1 ? path : path.slice(0, idx + separator.length - 1);
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
			  }
		)[];
		crumbs: Crumb[];
		path: string;

		constructor(path: string, rootSegments: Crumb[]) {
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

			this.crumbs = rootSegments;

			const segments = path.split('/').filter(Boolean);
			for (let i = 0; i < segments.length; i++) {
				const label = segments[i];
				this.crumbs.push({
					url: this.crumbs.at(-1)?.url + '/' + label,
					label,
				});
			}
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
	import type LoadProps from '@/lib/LoadProps';

	let { cwd }: AppLayoutProps<typeof loadProps> = $props();
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
