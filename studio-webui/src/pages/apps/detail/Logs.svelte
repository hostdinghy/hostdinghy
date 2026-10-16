<script lang="ts">
	import { logs as streamLogs, type LogLine } from '@/api/apps';
	import { errorToStr } from '@/api/lib';
	import Button from '@/components/Button.svelte';
	import Header from '@/components/Header.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import { parseAnsi } from '@/lib/ansi';

	let { app }: AppLayoutProps = $props();

	// keep the page fast for chatty apps
	const MAX_LINES = 5000;

	let tail: number | null = $state(500);
	let follow = $state(true);
	let service = $state('');

	let lines: LogLine[] = $state.raw([]);
	let status: 'loading' | 'following' | 'ended' | 'error' = $state('loading');
	let error = $state('');

	let outputEl: HTMLDivElement;
	let pinned = true;

	let services = $derived(app.services.map(s => s.name).sort());
	let shown = $derived(
		service ? lines.filter(l => l.service === service) : lines,
	);

	// restarts whenever tail or follow change
	$effect(() => {
		const ctrl = new AbortController();
		load(tail, follow, ctrl.signal);
		return () => ctrl.abort();
	});

	async function load(
		tail: number | null,
		follow: boolean,
		signal: AbortSignal,
	) {
		lines = [];
		status = 'loading';
		error = '';
		pinned = true;

		// collect lines and render them a few times per second
		let pending: LogLine[] = [];
		let timeout: ReturnType<typeof setTimeout> | null = null;
		const flush = () => {
			timeout = null;
			if (signal.aborted) return;
			lines = merge(lines, pending);
			pending = [];
		};

		try {
			await streamLogs(
				app.id,
				tail,
				follow,
				line => {
					pending.push(line);
					if (follow && status === 'loading') status = 'following';
					timeout ??= setTimeout(flush, 100);
				},
				signal,
			);
			flush();
			status = 'ended';
		} catch (e: any) {
			if (signal.aborted) return;
			flush();
			status = 'error';
			error = errorToStr(e);
		}
	}

	/** adds new lines in time order, they mostly belong at the end */
	function merge(lines: LogLine[], add: LogLine[]): LogLine[] {
		const all = lines.concat(add);
		all.sort((a, b) => (a.time < b.time ? -1 : a.time > b.time ? 1 : 0));
		return all.length > MAX_LINES ? all.slice(-MAX_LINES) : all;
	}

	function onscroll() {
		const el = outputEl;
		pinned = el.scrollHeight - el.scrollTop - el.clientHeight < 30;
	}

	// keep following the newest line unless the user scrolled up
	$effect(() => scrollToEnd(shown));

	function scrollToEnd(_lines: LogLine[]) {
		if (pinned) outputEl.scrollTop = outputEl.scrollHeight;
	}

	const timeFormat = new Intl.DateTimeFormat(undefined, {
		month: 'short',
		day: '2-digit',
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit',
		hourCycle: 'h23',
	});

	function fmtTime(time: string): string {
		return timeFormat.format(new Date(time));
	}
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div class="logs">
	<Header bb>
		<h1>
			Logs
			<span class="status c-label f-small">
				{#if status === 'loading'}
					loading...
				{:else if status === 'following'}
					following
				{:else if status === 'ended'}
					{shown.length} lines
				{/if}
			</span>
		</h1>

		<div class="controls">
			<select bind:value={service} aria-label="service">
				<option value="">all services</option>
				{#each services as name (name)}
					<option value={name}>{name}</option>
				{/each}
			</select>

			<select bind:value={tail} aria-label="lines">
				<option value={100}>last 100</option>
				<option value={500}>last 500</option>
				<option value={2000}>last 2000</option>
				<option value={null}>all</option>
			</select>

			<Button
				style={follow ? 'accent' : 'text'}
				title={follow ? 'stop following' : 'follow new lines'}
				aria-pressed={follow}
				onclick={() => (follow = !follow)}
			>
				follow
			</Button>
		</div>
	</Header>

	{#if error}
		<p class="error">{error}</p>
	{/if}

	<div class="output" bind:this={outputEl} {onscroll}>
		{#each shown as l, i (i)}
			<div class="line">
				<span class="time" title={l.time}>{fmtTime(l.time)}</span>
				{#if services.length > 1}
					<span class="service">{l.container}</span>
				{/if}
				<span class="text">
					{#each parseAnsi(l.line) as seg, j (j)}
						{#if seg.style}
							<span style={seg.style}>{seg.text}</span>
						{:else}
							{seg.text}
						{/if}
					{/each}
				</span>
			</div>
		{:else}
			{#if status !== 'loading'}
				<p class="msg">No logs available</p>
			{/if}
		{/each}
	</div>
</div>

<style lang="scss">
	.status {
		margin-left: 0.5rem;
	}

	.controls {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	select {
		padding: 0.2rem 0.4rem;
		border: 1px solid var(--c-border);
		background-color: transparent;
		color: var(--c-text);
		font: inherit;
	}

	.output {
		height: calc(100vh - 16rem);
		min-height: 20rem;
		overflow: auto;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.line {
		display: flex;
		gap: 1rem;
		line-height: 1.5;
	}

	.time,
	.service {
		flex-shrink: 0;
		color: rgb(from var(--c-text) r g b / 50%);
	}

	.service {
		min-width: 6rem;
	}

	.text {
		white-space: pre-wrap;
		word-break: break-word;
	}

	.msg {
		color: rgb(from var(--c-text) r g b / 70%);
	}

	.error {
		padding: 0.5rem 1rem;
		color: var(--red);
	}
</style>
