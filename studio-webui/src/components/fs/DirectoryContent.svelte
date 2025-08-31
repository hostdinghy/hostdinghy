<script lang="ts">
	import type { FsDir } from '@/pages/apps/detail/Files.svelte';

	type Props = {
		directory: FsDir;
	};
	let { directory }: Props = $props();

	function toRelativeTime(date: Date, locale: string = 'en'): string {
		const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
		const now = new Date();
		const diff = date.getTime() - now.getTime();

		const seconds = Math.round(diff / 1000);
		const minutes = Math.round(seconds / 60);
		const hours = Math.round(minutes / 60);
		const days = Math.round(hours / 24);
		const months = Math.round(days / 30);
		const years = Math.round(days / 365);

		if (Math.abs(years) >= 1) return rtf.format(years, 'year');
		if (Math.abs(months) >= 1) return rtf.format(months, 'month');
		if (Math.abs(days) >= 1) return rtf.format(days, 'day');
		if (Math.abs(hours) >= 1) return rtf.format(hours, 'hour');
		if (Math.abs(minutes) >= 1) return rtf.format(minutes, 'minute');
		return rtf.format(seconds, 'second');
	}

	function joinPath(...parts) {
		return '/' + parts.map(p => p.replace(/^\/+|\/+$/g, '')).join('/');
	}
</script>

{#snippet item(
	type: 'directory' | 'file',
	name: string,
	url: string,
	meta: string[],
)}
	<div class="row type-{type}">
		<div class="icon">
			{type === 'directory' ? '📁' : '📄'}
		</div>
		<div class="name">
			<a class="underline" href={url}>{name}</a>
		</div>
		{#each meta as item}
			<div>{item}</div>
		{/each}
	</div>
{/snippet}

<div class="directory-content">
	<header class="row">
		<div>Name</div>
		<div></div>
		<div>permissions</div>
		<div>modified</div>
	</header>
	<div class="content">
		{#if !directory.isRoot()}
			{@render item(
				'directory',
				'..',
				directory.getParentSegment()?.url ?? '#',
				['', ''],
			)}
		{/if}
		{#each directory.content as row, i (row.id ?? i)}
			{@render item(
				row.type,
				row.name,
				joinPath(directory.getAbsoluteUrl(), row.name),
				[(row.permissions, toRelativeTime(row.modified))],
			)}
		{/each}
	</div>
</div>

<style lang="scss">
	header {
		font-weight: 500;
		border-bottom: 1px solid var(--c-border);
		padding: 0.5rem;
	}
	.content {
		display: flex;
		flex-direction: column;
		gap: 1em;
		margin-top: 1rem;
	}
	.row {
		display: grid;
		grid-template-columns: 1em 1fr 10rem 10rem;
		gap: 1em;
		align-items: center;
		padding-left: 1rem;
	}
</style>
