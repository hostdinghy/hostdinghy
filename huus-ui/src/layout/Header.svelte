<script>
	import Button from '@/components/Button.svelte';
	import { getSession } from '@/lib/Session';
	import { getRouter } from '@/main';

	const session = getSession();
	const router = getRouter();
	const req = getRouter().currentRequest;

	let { breadcrumbs = [{ label: '🛶 HostDinghy', url: '/' }] } = $props();

	let open = $state(false);

	$effect(() => {
		open = !$req;
	});

	function signout() {
		// todo: message server to invalidate session
		session.invalidate();
		router.open('/signin');
	}
</script>

<header class="wrap">
	<div class="crumbs">
		{#each breadcrumbs as breadcrumb}
			<a href={breadcrumb.url}>
				<span class="hover:underline">
					{breadcrumb.label}
				</span>
			</a>
		{/each}
	</div>
	{#if $session.isLoggedIn()}
		<div class="user-group group">
			{$session.user?.name}
			<span class="text-label">
				[{$session.user?.username}]
			</span>
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
		gap: 0.5rem;
		align-items: center;
	}

	.crumbs {
		display: flex;
		a + a:before {
			content: '/';
			padding-left: 0.5rem;
			padding-right: 0.5rem;
			opacity: 0.4;
		}
	}
</style>
