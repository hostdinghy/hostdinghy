<script module lang="ts">
	import {
		createRegistryUser,
		deleteRegistryUser,
		loadRegistryUsers,
	} from '@/api/servers/registry';
	import Button from '@/components/Button.svelte';
	import Table from '@/components/table/Table.svelte';

	export type Users = string[];

	export async function loadRegistryProps(serverId: string) {
		return {
			users: await loadRegistryUsers(serverId),
		};
	}
</script>

<script lang="ts">
	import Modal from '@/components/Modal.svelte';
	import Input from '@/form/Input.svelte';
	import { errorToStr } from '@/api/lib';
	import { toast } from '@/layout/Toasts.svelte';
	import type { Server } from '@/api/servers';
	import Delete from '@/assets/icons/Delete.svelte';
	import ButtonGroup from '@/components/ButtonGroup.svelte';
	import DescriptionList from '@/components/DescriptionList.svelte';
	import TableToolbar from '@/components/table/TableToolbar.svelte';
	import Add from '@/assets/icons/Add.svelte';
	import Header from '@/components/Header.svelte';

	let { server, users = $bindable() }: { server: Server; users: Users } =
		$props();

	let openCreateModal = $state(false);
	let newUsername = $state('');
	let newPassword = $state('');
	let error = $state('');

	function resetCreateModal() {
		openCreateModal = false;
		newUsername = '';
		newPassword = '';
		error = '';
	}

	async function onCreateSubmit(e: Event) {
		e.preventDefault();
		error = '';

		try {
			const { username, password } = await createRegistryUser(
				server.id,
				newUsername,
			);

			newPassword = password;
			users.push(username);
			users = users;
		} catch (e) {
			error = errorToStr(e);
		}
	}

	async function onDelete(username: string) {
		if (!confirm(`Are you sure you want to delete user "${username}"?`))
			return;

		try {
			await deleteRegistryUser(server.id, username);
			users = users.filter(u => u !== username);
		} catch (e) {
			toast({
				status: 'error',
				message: errorToStr(e),
			});
		}
	}
</script>

<!-- todo this looks shit -->

<div id="registry">
	<Header bb>
		<h1>Registry</h1>
	</Header>

	<div class="detail">
		<DescriptionList
			list={{
				Domain: server.registryDomain ?? '-',
			}}
		/>
	</div>

	<TableToolbar bt={false} bx={false}>
		<h2>Users</h2>

		<Button
			title="add user"
			aria-label="add user"
			onclick={() => (openCreateModal = true)}
		>
			<Add />
		</Button>
	</TableToolbar>

	<Table
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'actions', value: '' },
		]}
		rows={users.map(u => ({
			name: u,
			actions: null,
		}))}
		bx={false}
	>
		{#snippet actions(row)}
			<td class="actions">
				<ButtonGroup style="text" align="right">
					<Button
						title="delete"
						aria-label="delete"
						onclick={() => onDelete(row.name)}
					>
						<Delete />
					</Button>
				</ButtonGroup>
			</td>
		{/snippet}
	</Table>

	<Modal open={openCreateModal} onclose={resetCreateModal}>
		<header>
			<h2 class="h2-mod">Create User</h2>
			<Button onclick={resetCreateModal}>&times;</Button>
		</header>

		{#if newPassword}
			<div class="new-password">
				<p>
					<strong>Your new password:</strong>
					<span class="pw">{newPassword}</span>
				</p>

				<p>!This will only be display once!</p>
			</div>
		{:else}
			<form onsubmit={onCreateSubmit}>
				<Input
					id="username"
					name="username"
					type="text"
					label="username"
					placeholder="Enter username..."
					bind:value={newUsername}
					required
				/>

				{#if error}
					<div class="error">{error}</div>
				{/if}

				<div class="btns">
					<Button type="submit" class="btn">submit</Button>
				</div>
			</form>
		{/if}
	</Modal>
</div>

<style lang="scss">
	.detail {
		padding: 1rem;
		margin-bottom: 1rem;
		border-bottom: 1px solid var(--c-border);
	}

	.actions {
		width: 10%;
	}

	// modal
	#registry :global .modal {
		width: 100%;
		max-width: 28rem;
	}

	header {
		display: flex;
		padding: 1rem;
		justify-content: space-between;
		align-items: center;
		border-bottom: 1px solid var(--c-border);
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
