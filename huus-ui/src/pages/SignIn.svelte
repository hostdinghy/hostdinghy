<script module lang="ts">
	export const requiresRights = [];

	export const loadProps: LoadPropsFn = (_props, lp) => {
		if (lp.session.isLoggedIn()) {
			lp.setRedirect('/');
		}
	};
</script>

<script lang="ts">
	import { login } from '@/api/users';
	import Input from '@/form/Input.svelte';
	// import { toast } from '@/layout/Toasts.svelte';
	import type { LoadPropsFn } from '@/lib/LoadProps';
	import { getSession } from '@/lib/Session';
	import { getRouter } from '@/main';

	const session = getSession();
	const router = getRouter();
	const req = router.currentRequest;

	const { url }: { url?: string } = $props();
	$inspect('url', url);

	let username = $state('');
	let password = $state('');
	let error = $state('');

	async function onsubmit(e: Event) {
		e.preventDefault();

		try {
			const auth = await login(username, password);
			session.setAuthed(auth);

			// nows lets either redirect to url or to /me
			const url = $req?.search.get('url') ?? '/';
			router.open(url);
		} catch (e) {
			console.error(e);
			error = 'Username or password is incorrect.';
			return;
		}
	}
</script>

<div id="signin">
	<div class="center">
		<h1>Huus Login</h1>
		<form {onsubmit}>
			<Input
				id="username"
				name="username"
				type="text"
				label="username"
				placeholder="Enter username..."
				bind:value={username}
				required
			/>

			<Input
				id="password"
				name="password"
				type="password"
				label="Password"
				placeholder="Enter password..."
				bind:value={password}
				required
			/>

			{#if error}
				<div class="error">{error}</div>
			{/if}

			<button type="submit" class="btn mt">submit</button>
		</form>
	</div>
</div>

<style lang="scss">
	#signin {
		display: flex;
		width: 100%;
		min-height: 100vh;
		justify-content: center;
		align-items: center;
	}

	h1 {
		font-size: 1.25rem;
		margin-bottom: 2rem;
		text-transform: uppercase;
	}

	.error {
		margin-top: 1.5rem;
		color: var(--red);
	}
</style>
