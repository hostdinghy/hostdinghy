<script lang="ts">
	import { humanSize, joinPath, type FsEntry } from '@/api/apps/fs';
	import Download from '@/assets/icons/Download.svelte';
	import File from '@/assets/icons/File.svelte';
	import Folder from '@/assets/icons/Folder.svelte';
	import Link from '@/assets/icons/Link.svelte';
	import MoreVert from '@/assets/icons/MoreVert.svelte';
	import Button from '@/components/Button.svelte';
	import ButtonGroup from '@/components/ButtonGroup.svelte';
	import ContextMenu, {
		type ContextMenuItem,
	} from '@/components/ContextMenu.svelte';
	import Table from '@/components/table/Table.svelte';

	let {
		entries,
		path,
		urlFor,
		actionsFor,
		ondownload,
	}: {
		entries: FsEntry[];
		/** of the directory */
		path: string;
		urlFor: (path: string) => string;
		actionsFor: (entry: FsEntry) => ContextMenuItem[];
		ondownload: (entry: FsEntry) => void;
	} = $props();

	type Row = {
		id: string;
		/** null for the link to the parent */
		entry: FsEntry | null;
		name: null;
		size: null;
		mode: null;
		owner: null;
		modified: null;
		actions: null;
	};

	let rows: Row[] = $derived([
		...(path ? [{ id: '..', entry: null }] : []),
		...entries.map(entry => ({ id: entry.name, entry })),
	] as Row[]);

	const rtf = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });
	const absFormat = new Intl.DateTimeFormat(undefined, {
		dateStyle: 'medium',
		timeStyle: 'medium',
	});

	function relativeTime(date: Date): string {
		const steps: [number, Intl.RelativeTimeFormatUnit][] = [
			[60, 'second'],
			[60, 'minute'],
			[24, 'hour'],
			[30, 'day'],
			[12, 'month'],
			[Infinity, 'year'],
		];
		let value = Math.round((date.getTime() - Date.now()) / 1000);
		for (const [size, unit] of steps) {
			if (Math.abs(value) < size) return rtf.format(value, unit);
			value = Math.round(value / size);
		}
		return '';
	}
</script>

<Table
	headers={[
		{ key: 'name', value: 'Name' },
		{ key: 'size', value: 'Size' },
		{ key: 'mode', value: 'Permissions' },
		{ key: 'owner', value: 'Owner' },
		{ key: 'modified', value: 'Modified' },
		{ key: 'actions', value: '' },
	]}
	{rows}
	bx={false}
>
	{#snippet name(row)}
		<td class="name">
			{#if !row.entry}
				<a
					class="entry"
					href={urlFor(path.split('/').slice(0, -1).join('/'))}
				>
					<Folder />
					<span class="underline">..</span>
				</a>
			{:else}
				{@const e = row.entry}
				<a
					class="entry"
					class:other={e.kind === 'other'}
					href={e.kind === 'other'
						? undefined
						: urlFor(joinPath(path, e.name))}
				>
					{#if e.kind === 'directory'}
						<Folder />
					{:else if e.kind === 'symlink'}
						<Link />
					{:else}
						<File />
					{/if}
					<span class="underline">{e.name}</span>
					{#if e.linkTarget}
						<span class="link-target">→ {e.linkTarget}</span>
					{/if}
				</a>
			{/if}
		</td>
	{/snippet}

	{#snippet size(row)}
		<td class="size">
			{row.entry?.kind === 'file' ? humanSize(row.entry.size) : ''}
		</td>
	{/snippet}

	{#snippet mode(row)}
		<td class="mode" title={row.entry?.modeOctal()}>
			{row.entry?.modeStr() ?? ''}
		</td>
	{/snippet}

	{#snippet owner(row)}
		<td>{row.entry ? `${row.entry.uid}:${row.entry.gid}` : ''}</td>
	{/snippet}

	{#snippet modified(row)}
		<td title={row.entry ? absFormat.format(row.entry.modified) : ''}>
			{row.entry ? relativeTime(row.entry.modified) : ''}
		</td>
	{/snippet}

	{#snippet actions(row)}
		<td>
			{#if row.entry}
				{@const e = row.entry}
				<ButtonGroup style="text" align="right">
					{#if e.kind !== 'other'}
						<Button
							title="download"
							aria-label="download"
							onclick={() => ondownload(e)}
						>
							<Download />
						</Button>
					{/if}
					<ContextMenu
						style="text"
						title="actions"
						items={actionsFor(e)}
					>
						<MoreVert />
					</ContextMenu>
				</ButtonGroup>
			{/if}
		</td>
	{/snippet}
</Table>

{#if !entries.length}
	<p class="empty">This directory is empty.</p>
{/if}

<style lang="scss">
	.entry {
		display: flex;
		gap: 0.75rem;
		align-items: center;

		:global(svg) {
			flex-shrink: 0;
			color: rgb(from var(--c-text) r g b / 60%);
		}
	}

	.other .underline {
		text-decoration: none;
	}

	.link-target {
		color: rgb(from var(--c-text) r g b / 60%);
	}

	.size {
		white-space: nowrap;
	}

	.mode {
		font-variant-numeric: tabular-nums;
	}

	.empty {
		padding: 1rem;
		color: rgb(from var(--c-text) r g b / 70%);
	}
</style>
