<script module lang="ts">
	import {
		newPassword as pgNewPw,
		createDatabase,
		loadDatabases,
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
	import CloseModal from '@/components/modal/CloseModal.svelte';
	import Input from '@/form/Input.svelte';
	import type { Server } from '@/api/servers';
	import ButtonGroup from '@/components/ButtonGroup.svelte';
	import Refresh from '@/assets/icons/Refresh.svelte';

	let { server, dbs = $bindable() }: { server: Server; dbs: Databases } =
		$props();

	let openCreateModal = $state(false);
	let newName = $state('');
	let newPassword = $state('');
	let error = $state('');

	function resetCreateModal() {
		openCreateModal = false;
		newName = '';
		error = '';
	}

	async function onCreateSubmit(e: Event) {
		e.preventDefault();
		error = '';

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
			alert(errorToStr(e));
		}
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
						<Refresh />
					</Button>
				</ButtonGroup>
			</td>
		{/snippet}
	</Table>

	<Modal open={openCreateModal} onclose={resetCreateModal}>
		<header>
			<h2 class="h2-mod">Create database</h2>
			<CloseModal onclick={resetCreateModal} />
		</header>

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

	<Modal open={!!newPassword} onclose={() => (newPassword = '')}>
		<header>
			<h2 class="h2-mod">Database password</h2>
			<CloseModal onclick={() => (newPassword = '')} />
		</header>

		<div class="new-password">
			<p>
				<strong>Your new password:</strong>
				<span class="pw">{newPassword}</span>
			</p>

			<p>!This will only be display once!</p>
		</div>
	</Modal>
</div>

<style lang="scss">
	.actions {
		width: 10%;
	}

	// modal
	#postgres :global .modal {
		width: 100%;
		max-width: 28rem;
	}

	header {
		display: flex;
		padding: 1rem;
		justify-content: space-between;
		align-items: center;
	}

	.new-password {
		padding: 1rem;

		strong {
			display: block;
			margin-bottom: 0.3rem;
			color: var(--c-label);
		}

		.pw {
			word-break: break-all;
		}

		p:not(:last-child) {
			margin-bottom: 1.5rem;
		}
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
