<script module lang="ts">
	import {
		newPassword as pgNewPw,
		createDatabase,
		loadDatabases,
		restoreDatabase,
		dumpDatabase,
	} from '@/api/servers/postgres';

	export type Databases = string[];

	export async function loadPostgresProps(serverId: string) {
		return {
			dbs: await loadDatabases(serverId),
		};
	}
</script>

<script lang="ts">
	import Add from '@/assets/icons/Add.svelte';
	import Button from '@/components/Button.svelte';
	import Header from '@/components/Header.svelte';
	import Table from '@/components/table/Table.svelte';
	import Modal from '@/components/modal/Modal.svelte';
	import { errorToStr } from '@/api/lib';
	import Input from '@/form/Input.svelte';
	import FileUpload from '@/form/FileUpload.svelte';
	import type { Server } from '@/api/servers';
	import ButtonGroup from '@/components/ButtonGroup.svelte';
	import KeyAlert from '@/assets/icons/KeyAlert.svelte';
	import Upload from '@/assets/icons/Upload.svelte';
	import NewPasswordModal from '@/components/modal/NewPasswordModal.svelte';
	import Download from '@/assets/icons/Download.svelte';
	import ProgressBar from '@/form/ProgressBar.svelte';
	import { createToastHandler } from '@/layout/Toasts.svelte';
	import { browserDateTimePathSafe } from '@/lib/time';

	let { server, dbs = $bindable() }: { server: Server; dbs: Databases } =
		$props();

	let openCreateModal = $state(false);
	let newName = $state('');
	let newPassword = $state('');
	let error = $state('');
	let toast = createToastHandler();

	function resetErrors() {
		error = '';
		toast.remove();
	}

	function resetCreateModal() {
		openCreateModal = false;
		newName = '';
		error = '';
	}

	async function onCreateSubmit(e: Event) {
		e.preventDefault();
		resetErrors();

		try {
			const { name, password } = await createDatabase(server.id, newName);

			resetCreateModal();
			newPassword = password;
			dbs.push(name);
			dbs = dbs;
		} catch (e) {
			error = errorToStr(e);
		}
	}

	async function onGenNewPw(name: string) {
		if (!confirm('Are you sure you want to generate a new password?'))
			return;

		try {
			const { password } = await pgNewPw(server.id, name);

			newPassword = password;
		} catch (e) {
			toast.error(errorToStr(e));
		}
	}

	// contains the databasename if the modal is open
	let openRestoreModal: string | null = $state(null);
	let restoreFile: File | null = $state(null);
	let uploadProgress: number = $state(0);

	async function onRestoreSubmit(e: Event) {
		e.preventDefault();
		resetErrors();
		uploadProgress = 0;

		if (!restoreFile) {
			error = 'Please select a file to restore.';
			return;
		}

		if (
			!confirm(
				'Restoring will overwrite the existing database. Continue?',
			)
		)
			return;

		try {
			await restoreDatabase(
				server.id,
				openRestoreModal!,
				restoreFile,
				prog => (uploadProgress = prog),
			);
			resetRestoreModal();
		} catch (e) {
			error = errorToStr(e);
		}
	}

	function resetRestoreModal() {
		openRestoreModal = null;
		restoreFile = null;
		error = '';
		uploadProgress = 0;
	}

	async function onDumpDb(name: string) {
		let resp: Response;
		try {
			resp = await dumpDatabase(server.id, name);
		} catch (e) {
			toast.error(errorToStr(e));
			return;
		}

		const filename = `${name}-${browserDateTimePathSafe(new Date())}.sql`;

		toast.info(`Downloading "${filename}"`);

		try {
			// @ts-ignore
			if (typeof window.showSaveFilePicker === 'function') {
				const downloaded = await downloadStream(resp, filename);
				if (!downloaded) {
					toast.remove();
					return;
				}
			} else {
				await downloadBlob(resp, filename);
			}

			toast.success('Database dump downloaded.');
		} catch (e) {
			toast.error(errorToStr(e));
		}
	}

	async function downloadStream(
		resp: Response,
		filename: string,
	): Promise<boolean> {
		let fileHandle;
		try {
			// @ts-ignore
			fileHandle = await window.showSaveFilePicker({
				suggestedName: filename,
			});
		} catch (e) {
			if ((e as DOMException)?.name === 'AbortError') return false;
			throw e;
		}

		const writable = await fileHandle.createWritable();
		await resp.body!.pipeTo(writable);

		return true;
	}

	async function downloadBlob(resp: Response, filename: string) {
		const blob = await resp.blob();

		const el = document.createElement('a');
		const url = URL.createObjectURL(blob);
		document.body.appendChild(el);
		el.href = url;
		el.download = filename;
		el.click();
		el.remove();
		window.URL.revokeObjectURL(url);
	}
</script>

<div id="postgres">
	<Header>
		<h1>Postgres databases</h1>

		<Button
			title="add database"
			aria-label="add database"
			onclick={() => (openCreateModal = true)}
		>
			<Add />
		</Button>
	</Header>

	<Table
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'actions', value: '' },
		]}
		rows={dbs.map(name => ({ name, actions: null }))}
		bx={false}
	>
		{#snippet actions(row)}
			<td class="actions">
				<ButtonGroup style="text" align="right">
					<Button
						title="new password"
						aria-label="new password"
						onclick={() => onGenNewPw(row.name)}
					>
						<KeyAlert />
					</Button>
					<Button
						title="restore"
						aria-label="restore"
						onclick={() => (openRestoreModal = row.name)}
					>
						<Upload />
					</Button>
					<Button
						title="dump"
						aria-label="dump"
						onclick={() => onDumpDb(row.name)}
					>
						<Download />
					</Button>
				</ButtonGroup>
			</td>
		{/snippet}
	</Table>

	<Modal
		open={openCreateModal}
		title="Create database"
		size="small"
		onclose={resetCreateModal}
	>
		<form onsubmit={onCreateSubmit}>
			<Input
				id="name"
				name="name"
				type="text"
				label="name"
				placeholder="Enter name..."
				bind:value={newName}
				required
				bx={false}
			/>

			{#if error}
				<div class="error">{error}</div>
			{/if}

			<div class="btns">
				<Button>submit</Button>
			</div>
		</form>
	</Modal>

	<NewPasswordModal title="Database password" bind:password={newPassword} />

	<Modal
		open={!!openRestoreModal}
		title="Restore database"
		size="medium"
		onclose={resetRestoreModal}
	>
		<form onsubmit={onRestoreSubmit}>
			<FileUpload
				id="restore-file"
				name="restore-file"
				label="sql file"
				bx={false}
				bind:file={restoreFile}
			/>

			{#if error}
				<div class="error">{error}</div>
			{/if}

			<ProgressBar progress={uploadProgress} mt />

			<div class="btns">
				<Button>submit</Button>
			</div>
		</form>
	</Modal>
</div>

<style lang="scss">
	.actions {
		width: 20%;
	}

	.error {
		margin-top: 1rem;
		padding: 0 1rem;
		color: var(--red);
	}

	.btns {
		display: flex;
		margin-top: 1rem;
		padding: 1rem;
	}
</style>
