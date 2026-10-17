<script>
	import Button from '@/components/Button.svelte';
	import Crumbs from '@/components/Crumbs.svelte';
	import { getSession } from '@/lib/Session';
	import { getRouter } from '@/main';

	const session = getSession();
	const router = getRouter();

	let { breadcrumbs = [{ label: '🛶 HostDinghy', url: '/' }] } = $props();

	function signout() {
		// todo: message server to invalidate session
		session.invalidate();
		router.open('/signin');
	}
</script>

<header class="wrap">
	<Crumbs {breadcrumbs} />
	{#if $session.isLoggedIn()}
		<div class="user-group group">
			<a href="/settings/account" class="hover:underline">
				{$session.user?.name}
				<span class="text-label">
					[{$session.user?.username}]
				</span>
			</a>
			<Button onclick={signout}>logout</Button>
		</div>
	{/if}
</header>

<style>
	.text-label {
		color: var(--c-label);
	}

	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 2rem;
	}

	.group {
		display: flex;
		gap: 1rem;
		align-items: center;
	}
</style>
