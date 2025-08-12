<script module lang="ts">
	import {
		createRegistryUser,
		deleteRegistryUser,
		loadRegistryUsers,
	} from '@/api/apps/registry';
	import Button from '@/components/Button.svelte';
	import Table from '@/components/Table.svelte';

	export async function loadProps({ app }: AppLayoutProps) {
		return {
			users: await loadRegistryUsers(app.id),
		};
	}
</script>

<script lang="ts">
	import Modal from '@/components/Modal.svelte';
	import type { AppLayoutProps } from '@/layout/AppLayout.svelte';
	import Input from '@/form/Input.svelte';
	import { errorToStr } from '@/api/lib';
	import { toast } from '@/layout/Toasts.svelte';

	let { app, users = $bindable() }: AppLayoutProps<typeof loadProps> =
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
				app.id,
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
			await deleteRegistryUser(app.id, username);
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
	<h1>Registry</h1>

	<div class="detail">
		<dl>
			<dt>Domain</dt>
			<dd>{app.server?.registryDomain ?? '-'}</dd>
		</dl>
	</div>

	<Table
		headers={[
			{ key: 'name', value: 'Name' },
			{ key: 'actions', value: 'Actions' },
		]}
		rows={users.map(u => ({
			name: u,
			actions: null,
		}))}
	>
		{#snippet toolbar()}
			<h2 class="h2-table">Users</h2>
			<Button onclick={() => (openCreateModal = true)}>add</Button>
		{/snippet}

		{#snippet actions(row)}
			<td class="actions">
				<Button onclick={() => onDelete(row.name)}>delete</Button>
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
	h1 {
		padding: 1rem;
		font-size: 1.125rem;
	}

	.detail {
		padding: 1rem;
		margin-bottom: 1rem;
	}

	dl {
		display: grid;
		grid-template-columns: 10rem 1fr;
	}

	.h2-table {
		padding: 1rem;
		flex: 1;
		font-size: 1.125rem;
	}

	.actions {
		padding: 0.5rem 1rem;
		width: 13rem;
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

	.h2-mod {
		font-size: 1.25rem;
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
