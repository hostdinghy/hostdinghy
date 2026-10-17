<script lang="ts">
	import type { FsEntry } from '@/api/apps/fs';
	import Button from '@/components/Button.svelte';
	import Modal from '@/components/modal/Modal.svelte';
	import Input from '@/form/Input.svelte';
	import Select from '@/form/Select.svelte';

	let {
		entry,
		onsubmit,
		onclose,
	}: {
		/** the modal is open if set */
		entry: FsEntry | null;
		/** throws to show an error */
		onsubmit: (req: {
			mode?: number;
			uid?: number;
			gid?: number;
			recursive: boolean;
		}) => Promise<void>;
		onclose: () => void;
	} = $props();

	let mode = $state('');
	let uid = $state('');
	let gid = $state('');
	let recursive = $state('no');
	let error = $state('');
	let busy = $state(false);

	$effect(() => {
		if (entry) {
			mode = entry.modeOctal();
			uid = String(entry.uid);
			gid = String(entry.gid);
			recursive = 'no';
			error = '';
		}
	});

	function parseNum(s: string, radix: number, what: string): number {
		const n = parseInt(s.trim(), radix);
		if (isNaN(n) || n < 0 || !/^[0-9]+$/.test(s.trim()))
			throw new Error(`${what} must be a number`);
		return n;
	}

	async function onformsubmit(e: Event) {
		e.preventDefault();
		if (busy || !entry) return;

		error = '';
		let req;
		try {
			const m = parseNum(mode, 8, 'Mode');
			if (m > 0o7777) throw new Error('Mode must be at most 7777');
			const u = parseNum(uid, 10, 'User id');
			const g = parseNum(gid, 10, 'Group id');
			req = {
				// only send what changed
				mode: m !== entry.mode ? m : undefined,
				uid: u !== entry.uid ? u : undefined,
				gid: g !== entry.gid ? g : undefined,
				recursive: recursive === 'yes',
			};
			// recursive applies the values to everything inside
			if (req.recursive) {
				req.mode = m;
				req.uid = u;
				req.gid = g;
			}
		} catch (e: any) {
			error = e.message;
			return;
		}

		busy = true;
		try {
			await onsubmit(req);
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}
</script>

<Modal
	open={!!entry}
	title="Permissions of {entry?.name || 'app directory'}"
	size="medium"
	{onclose}
>
	<form onsubmit={onformsubmit}>
		<Input
			id="fs-mode"
			name="fs-mode"
			type="text"
			label="mode (octal)"
			bind:value={mode}
			bx={false}
		/>
		<Input
			id="fs-uid"
			name="fs-uid"
			type="text"
			label="user id"
			bind:value={uid}
			bx={false}
		/>
		<Input
			id="fs-gid"
			name="fs-gid"
			type="text"
			label="group id"
			bind:value={gid}
			bx={false}
		/>
		{#if entry?.isDir()}
			<Select
				id="fs-recursive"
				name="fs-recursive"
				label="everything inside"
				bind:value={recursive}
				options={[
					{ key: 'no', value: 'no, only this directory' },
					{ key: 'yes', value: 'yes, same mode and owner' },
				]}
				bx={false}
			/>
		{/if}

		<p class="help">
			e.g. 644 for files and 755 for directories. Ids are the ones inside
			the containers, many images run as 1000 or 33 (www-data).
		</p>

		{#if error}
			<p class="error">{error}</p>
		{/if}

		<div class="btns">
			<Button disabled={busy}>{busy ? 'saving...' : 'save'}</Button>
		</div>
	</form>
</Modal>

<style lang="scss">
	.help {
		padding: 1rem 1rem 0;
		color: rgb(from var(--c-text) r g b / 70%);
		font-size: 0.875rem;
	}

	.error {
		padding: 1rem 1rem 0;
		color: var(--red);
	}

	.btns {
		display: flex;
		padding: 1rem;
	}
</style>
