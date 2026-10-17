<script module lang="ts">
	import { info, read, type FsInfo } from '@/api/apps/fs';
	import { decodeText, MAX_EDIT_SIZE } from '@/components/fs/FileView.svelte';

	export async function loadProps({
		app,
		path = '',
	}: { path?: string } & AppLayoutProps) {
		const p = decodePath(path);
		const fsInfo = await info(app.id, p);

		// text files open in the editor
		let text: string | null = null;
		if (
			fsInfo.entry.kind === 'file' &&
			fsInfo.entry.size <= MAX_EDIT_SIZE
		) {
			const resp = await read(app.id, p);
			text = decodeText(await resp.arrayBuffer());
		}

		return { path: p, fsInfo, text };
	}

	/** the path from the url, '' is the app directory */
	function decodePath(raw: string): string {
		return raw
			.split('/')
			.filter(Boolean)
			.map(s => {
				try {
					return decodeURIComponent(s);
				} catch {
					return s;
				}
			})
			.join('/');
	}
</script>

<script lang="ts">
	import * as fs from '@/api/apps/fs';
	import { FsEntry, humanSize, joinPath, parentPath } from '@/api/apps/fs';
	import { ApiError, errorToStr } from '@/api/lib';
	import ContentCopy from '@/assets/icons/ContentCopy.svelte';
	import CreateNewFolder from '@/assets/icons/CreateNewFolder.svelte';
	import Delete from '@/assets/icons/Delete.svelte';
	import Download from '@/assets/icons/Download.svelte';
	import DriveFileMove from '@/assets/icons/DriveFileMove.svelte';
	import DriveFolderUpload from '@/assets/icons/DriveFolderUpload.svelte';
	import FolderZip from '@/assets/icons/FolderZip.svelte';
	import Lock from '@/assets/icons/Lock.svelte';
	import MoreVert from '@/assets/icons/MoreVert.svelte';
	import NoteAdd from '@/assets/icons/NoteAdd.svelte';
	import Save from '@/assets/icons/Save.svelte';
	import Unarchive from '@/assets/icons/Unarchive.svelte';
	import Upload from '@/assets/icons/Upload.svelte';
	import UploadFile from '@/assets/icons/UploadFile.svelte';
	import ContextMenu, {
		type ContextMenuItem,
	} from '@/components/ContextMenu.svelte';
	import Button from '@/components/Button.svelte';
	import ButtonGroup from '@/components/ButtonGroup.svelte';
	import Crumbs, { type Crumb } from '@/components/Crumbs.svelte';
	import DirectoryContent from '@/components/fs/DirectoryContent.svelte';
	import FileView from '@/components/fs/FileView.svelte';
	import PathModal from '@/components/fs/PathModal.svelte';
	import PermissionsModal from '@/components/fs/PermissionsModal.svelte';
	import ProgressBar from '@/form/ProgressBar.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import { createToastHandler } from '@/layout/Toasts.svelte';
	import { dispositionFilename, saveResponse } from '@/lib/download';
	import { getRouter } from '@/main';

	let { app, path, fsInfo, text }: AppLayoutProps<typeof loadProps> =
		$props();

	const router = getRouter();
	const toast = createToastHandler();

	// a reload after a change replaces the loaded info until the next page
	let reloaded = $state<{ path: string; info: FsInfo } | null>(null);
	let cur = $derived(reloaded?.path === path ? reloaded.info : fsInfo);
	let isDir = $derived(cur.entry.isDir());

	function urlFor(p: string): string {
		const base = `/apps/${app.id}/files`;
		if (!p) return base;
		return base + '/' + p.split('/').map(encodeURIComponent).join('/');
	}

	let crumbs: Crumb[] = $derived.by(() => {
		const crumbs = [{ url: urlFor(''), label: '/' }];
		let p = '';
		for (const seg of path.split('/').filter(Boolean)) {
			p = joinPath(p, seg);
			crumbs.push({ url: urlFor(p), label: seg });
		}
		return crumbs;
	});

	async function reload() {
		try {
			reloaded = { path, info: await fs.info(app.id, path) };
		} catch (e) {
			toast.error(errorToStr(e));
		}
	}

	/** runs an action and reloads, the result shows in the listing */
	async function act(fn: () => Promise<void>) {
		await fn();
		await reload();
	}

	type Selected = { entry: FsEntry; path: string };

	/** the actions of the context menu of an entry */
	function actionsFor(s: Selected): ContextMenuItem[] {
		const items: ContextMenuItem[] = [];
		const changeable = s.path !== '' && s.path !== 'compose.yml';

		if (changeable)
			items.push({
				label: 'Rename or move',
				icon: DriveFileMove,
				onclick: () => askRename(s),
			});
		if (s.path !== '')
			items.push({
				label: 'Copy',
				icon: ContentCopy,
				onclick: () => askCopy(s),
			});
		if (s.entry.kind !== 'symlink') {
			items.push({
				label: 'Permissions',
				icon: Lock,
				onclick: () => (permissionsOf = s),
			});
			items.push({
				label: 'Zip',
				icon: FolderZip,
				onclick: () => askZip(s),
			});
		}
		if (s.entry.kind === 'file' && fs.isArchive(s.entry.name))
			items.push({
				label: 'Extract',
				icon: Unarchive,
				onclick: () => askExtract(s),
			});
		if (changeable)
			items.push({
				label: 'Delete',
				icon: Delete,
				danger: true,
				onclick: () => deleteEntry(s),
			});

		return items;
	}

	type PathAsk = {
		title: string;
		label: string;
		value: string;
		help?: string;
		submit: string;
		run: (value: string) => Promise<void>;
	};
	let ask = $state<PathAsk | null>(null);
	let permissionsOf = $state<{ entry: FsEntry; path: string } | null>(null);

	const PATH_HELP =
		'Relative to the app directory, missing folders are created.';

	function askNewFolder() {
		ask = {
			title: 'New folder',
			label: 'path',
			value: joinPath(path, ''),
			help: PATH_HELP,
			submit: 'create',
			run: async p => act(() => fs.mkdir(app.id, p)),
		};
	}

	function askNewFile() {
		ask = {
			title: 'New file',
			label: 'path',
			value: joinPath(path, ''),
			help: PATH_HELP,
			submit: 'create',
			run: async p => {
				const name = p.split('/').pop()!;
				await fs.write(app.id, p, new File([], name), false);
				ask = null;
				router.open(urlFor(p));
			},
		};
	}

	function askRename(s: { entry: FsEntry; path: string }) {
		ask = {
			title: `Rename or move ${s.entry.name}`,
			label: 'new path',
			value: s.path,
			help: PATH_HELP,
			submit: 'move',
			run: async to => {
				await fs.move(app.id, s.path, to);
				// the open file or directory moved
				if (s.path === path) router.open(urlFor(to));
				else await reload();
			},
		};
	}

	function askCopy(s: { entry: FsEntry; path: string }) {
		const dot = s.entry.isDir() ? -1 : s.path.lastIndexOf('.');
		const copyPath =
			dot > s.path.lastIndexOf('/') + 1
				? `${s.path.slice(0, dot)}-copy${s.path.slice(dot)}`
				: `${s.path}-copy`;
		ask = {
			title: `Copy ${s.entry.name}`,
			label: 'copy to',
			value: copyPath,
			help: PATH_HELP,
			submit: 'copy',
			run: async to => act(() => fs.copy(app.id, s.path, to)),
		};
	}

	function askZip(s: { entry: FsEntry; path: string }) {
		const name = s.entry.name || app.id;
		ask = {
			title: `Zip ${name}`,
			label: 'zip file',
			value: joinPath(
				s.path === path ? parentPath(path) : path,
				`${name}.zip`,
			),
			submit: 'zip',
			run: async to => act(() => fs.zip(app.id, s.path, to)),
		};
	}

	function askExtract(s: { entry: FsEntry; path: string }) {
		const base = s.entry.name.replace(/\.(zip|tar|tar\.gz|tgz)$/i, '');
		ask = {
			title: `Extract ${s.entry.name}`,
			label: 'into new folder',
			value: joinPath(parentPath(s.path), base),
			help: 'Symlinks in the archive are skipped.',
			submit: 'extract',
			run: async to => act(() => fs.extract(app.id, s.path, to)),
		};
	}

	async function onAskSubmit(value: string) {
		if (!ask) return;
		await ask.run(value);
		ask = null;
	}

	async function deleteEntry(s: { entry: FsEntry; path: string }) {
		const what = s.entry.isDir()
			? `the folder "${s.path}" and everything inside`
			: `"${s.path}"`;
		if (!confirm(`Delete ${what}?`)) return;

		try {
			await fs.remove(app.id, s.path);
			if (s.path === path) router.open(urlFor(parentPath(path)));
			else await reload();
		} catch (e) {
			toast.error(errorToStr(e));
		}
	}

	let download = $state<{
		name: string;
		received: number;
		total: number | null;
	} | null>(null);

	async function downloadPath(p: string, entry: FsEntry) {
		download = { name: entry.name || app.id, received: 0, total: null };
		try {
			const resp = await fs.download(app.id, p);
			const name = dispositionFilename(resp) ?? download.name;
			download.name = name;
			await saveResponse(resp, name, (received, total) => {
				if (!download) return;
				download.received = received;
				download.total = total;
			});
		} catch (e) {
			toast.error(errorToStr(e));
		} finally {
			download = null;
		}
	}

	async function onPermissions(req: {
		mode?: number;
		uid?: number;
		gid?: number;
		recursive: boolean;
	}) {
		const p = permissionsOf!.path;
		await act(() => fs.permissions(app.id, { path: p, ...req }));
		permissionsOf = null;
	}

	// uploads
	type UploadItem = { path: string; file: File };
	let upload = $state<{
		total: number;
		done: number;
		current: string;
		progress: number;
	} | null>(null);
	let fileInput: HTMLInputElement;
	let folderInput: HTMLInputElement;
	let dragging = $state(0);

	async function uploadAll(items: UploadItem[]) {
		if (!items.length || upload) return;
		toast.remove();
		upload = { total: items.length, done: 0, current: '', progress: 0 };
		let replaceAll = false;

		for (const item of items) {
			upload.current = item.path;
			upload.progress = 0;
			const send = (overwrite: boolean) =>
				fs.write(app.id, item.path, item.file, overwrite, p => {
					if (upload) upload.progress = p;
				});

			try {
				try {
					await send(replaceAll);
				} catch (e) {
					if (!(e instanceof ApiError) || e.type !== 'ALREADY_EXISTS')
						throw e;
					if (
						!replaceAll &&
						!confirm(
							`"${item.path}" already exists, replace it` +
								(items.length > 1
									? ' and all other existing files?'
									: '?'),
						)
					)
						continue;
					replaceAll = true;
					await send(true);
				}
			} catch (e) {
				toast.error(
					`Upload of "${item.path}" failed: ${errorToStr(e)}`,
				);
			}
			upload.done++;
		}

		upload = null;
		await reload();
	}

	function onFilesPicked(input: HTMLInputElement) {
		const items = Array.from(input.files ?? []).map(file => ({
			// a folder pick has the relative path inside the folder
			path: joinPath(path, file.webkitRelativePath || file.name),
			file,
		}));
		input.value = '';
		uploadAll(items);
	}

	/** files of a dropped folder */
	async function readEntry(
		entry: any,
		prefix: string,
	): Promise<UploadItem[]> {
		if (entry.isFile) {
			const file: File = await new Promise((res, rej) =>
				entry.file(res, rej),
			);
			return [{ path: joinPath(prefix, entry.name), file }];
		}

		const reader = entry.createReader();
		const items: UploadItem[] = [];
		// readEntries returns at most 100 entries per call
		while (true) {
			const batch: any[] = await new Promise((res, rej) =>
				reader.readEntries(res, rej),
			);
			if (!batch.length) break;
			for (const child of batch)
				items.push(
					...(await readEntry(child, joinPath(prefix, entry.name))),
				);
		}
		return items;
	}

	async function ondrop(e: DragEvent) {
		e.preventDefault();
		dragging = 0;
		if (!isDir || !e.dataTransfer) return;

		const entries = Array.from(e.dataTransfer.items)
			.map(i => i.webkitGetAsEntry?.())
			.filter(Boolean);
		try {
			const items = (
				await Promise.all(entries.map(en => readEntry(en, path)))
			).flat();
			await uploadAll(items);
		} catch (e) {
			toast.error(errorToStr(e));
		}
	}

	// editing a text file
	let fileView: FileView | undefined = $state();

	/** ctrl+s keeps the file open, the save button goes back */
	async function saveText(value: string): Promise<boolean> {
		try {
			await fs.write(
				app.id,
				path,
				new File([value], cur.entry.name),
				true,
			);
			toast.success(`Saved "${cur.entry.name}"`);
			await reload();
			return true;
		} catch (e) {
			toast.error(errorToStr(e));
			return false;
		}
	}

	async function saveAndBack() {
		const value = fileView?.getValue();
		if (value === undefined) return;
		if (await saveText(value)) router.open(urlFor(parentPath(path)));
	}

	const isCompose = $derived(path === 'compose.yml');
	const currentActions = $derived(actionsFor({ entry: cur.entry, path }));
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="files"
	class:dragging={dragging > 0}
	{ondrop}
	ondragover={e => e.preventDefault()}
	ondragenter={() => isDir && dragging++}
	ondragleave={() => (dragging = Math.max(0, dragging - 1))}
>
	<!-- the table has its own top border -->
	<header class:bb={!isDir}>
		<Crumbs breadcrumbs={crumbs} root />

		<ButtonGroup>
			{#if isDir}
				<Button
					title="new folder"
					aria-label="new folder"
					onclick={askNewFolder}
				>
					<CreateNewFolder />
				</Button>
				<Button
					title="new file"
					aria-label="new file"
					onclick={askNewFile}
				>
					<NoteAdd />
				</Button>
				<ContextMenu
					title="upload"
					items={[
						{
							label: 'Upload files',
							icon: UploadFile,
							onclick: () => fileInput.click(),
						},
						{
							label: 'Upload a folder',
							icon: DriveFolderUpload,
							onclick: () => folderInput.click(),
						},
					]}
				>
					<Upload />
				</ContextMenu>
			{:else if text !== null && !isCompose}
				<Button
					title="save and go back (ctrl+s saves and stays)"
					aria-label="save"
					onclick={saveAndBack}
				>
					<Save />
				</Button>
			{/if}
			<Button
				title={isDir ? 'download as .tar.gz' : 'download'}
				aria-label="download"
				onclick={() => downloadPath(path, cur.entry)}
			>
				<Download />
			</Button>
			{#if currentActions.length}
				<ContextMenu title="actions" items={currentActions}>
					<MoreVert />
				</ContextMenu>
			{/if}
		</ButtonGroup>
	</header>

	<input
		bind:this={fileInput}
		type="file"
		class="sr-only"
		multiple
		onchange={() => onFilesPicked(fileInput)}
	/>
	<input
		bind:this={folderInput}
		type="file"
		class="sr-only"
		webkitdirectory
		onchange={() => onFilesPicked(folderInput)}
	/>

	{#if upload}
		<div class="upload">
			<p>
				Uploading {upload.done + 1} of {upload.total}: {upload.current}
			</p>
			<ProgressBar
				progress={((upload.done + upload.progress / 100) /
					upload.total) *
					100}
			/>
		</div>
	{/if}

	{#if download}
		<div class="upload">
			<!-- a directory is packed while it is sent, so its size is unknown -->
			<p>
				Downloading {download.name}: {humanSize(download.received)}
				{#if download.total}of {humanSize(download.total)}{/if}
			</p>
			{#if download.total}
				<ProgressBar
					progress={(download.received / download.total) * 100}
				/>
			{/if}
		</div>
	{/if}

	{#if isDir}
		<DirectoryContent
			entries={cur.children ?? []}
			{path}
			{urlFor}
			actionsFor={e =>
				actionsFor({ entry: e, path: joinPath(path, e.name) })}
			ondownload={e => downloadPath(joinPath(path, e.name), e)}
		/>
		<p class="hint">Drop files or folders here to upload them.</p>
	{:else}
		<div class="file-info">
			{humanSize(cur.entry.size)} · {cur.entry.modeStr()} · {cur.entry
				.uid}:{cur.entry.gid}
			{#if isCompose}
				· read only here, edit it in the
				<a class="underline" href="/apps/{app.id}/settings">settings</a>
			{/if}
		</div>
		{#key path}
			<FileView
				bind:this={fileView}
				name={cur.entry.name}
				{text}
				readonly={isCompose}
				onsave={saveText}
			/>
		{/key}
	{/if}
</div>

<PathModal
	open={!!ask}
	title={ask?.title ?? ''}
	label={ask?.label ?? ''}
	value={ask?.value ?? ''}
	help={ask?.help}
	submit={ask?.submit ?? ''}
	onsubmit={onAskSubmit}
	onclose={() => (ask = null)}
/>

<PermissionsModal
	entry={permissionsOf?.entry ?? null}
	onsubmit={onPermissions}
	onclose={() => (permissionsOf = null)}
/>

<style lang="scss">
	.files {
		min-height: 100%;

		&.dragging {
			background-color: rgb(from var(--c-accent) r g b / 8%);
		}
	}

	header {
		display: flex;
		gap: 1rem;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;

		&.bb {
			border-bottom: 1px solid var(--c-border);
		}
	}

	.upload {
		padding: 1rem;
		border-bottom: 1px solid var(--c-border);

		p {
			margin-bottom: 0.5rem;
		}
	}

	.hint {
		padding: 1rem;
		color: rgb(from var(--c-text) r g b / 50%);
		font-size: 0.875rem;
	}

	.file-info {
		padding: 0.5rem 1rem;
		border-bottom: 1px solid var(--c-border);
		color: rgb(from var(--c-text) r g b / 70%);
		font-size: 0.875rem;
	}
</style>
