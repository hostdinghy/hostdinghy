<script lang="ts">
	import { errorToStr } from '@/api/lib';
	import { save } from '@/api/users';
	import Header from '@/components/Header.svelte';
	import Input from '@/form/Input.svelte';
	import { toast } from '@/layout/Toasts.svelte';
	import { getSession } from '@/lib/Session';

	const session = getSession();

	let name = $state($session.user?.name ?? '');
	let password = $state('');
	let passwordRepeat = $state('');
	let error = $state('');

	async function onsubmit(e: Event) {
		e.preventDefault();

		error = '';

		if (password && password !== passwordRepeat) {
			error = 'Passwords do not match';
			return;
		}

		try {
			const auth = await save(name, password || null);

			session.setAuthed(auth);

			password = '';
			passwordRepeat = '';

			const ref = toast({
				message: 'Account saved',
				status: 'success',
			});
			setTimeout(() => ref.remove(), 2000);
		} catch (e) {
			console.error('Error saving user:', e);
			error = errorToStr(e);
		}
	}
</script>

<div class="account">
	<Header>
		<h1>
			{name}
			<span class="c-label">[{$session.user?.username}]</span>
		</h1>
	</Header>

	<form {onsubmit}>
		<Input
			id="name"
			name="name"
			type="text"
			label="name"
			placeholder="Enter name..."
			bind:value={name}
			required
			bx={false}
		/>

		<Input
			id="password"
			name="password"
			type="text"
			label="password"
			placeholder="Enter password..."
			bind:value={password}
			bx={false}
		/>

		{#if password}
			<Input
				id="password-repeat"
				name="passwordRepeat"
				type="text"
				label="repeat password"
				placeholder="Repeat password..."
				bind:value={passwordRepeat}
				required
				bx={false}
			/>
		{/if}

		{#if error}
			<div class="error">{error}</div>
		{/if}

		<div class="btns">
			<button type="submit" class="btn mt">save</button>
		</div>
	</form>
</div>

<style lang="scss">
	.error {
		margin-top: 1.5rem;
		padding: 0 1rem;
		color: var(--red);
	}

	.btns {
		padding-left: 1rem;
	}
</style>
