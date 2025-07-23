<script>
	import { getSession } from '@/lib/Session';
	import { getRouter } from '@/main';

	const session = getSession();
	const req = getRouter().currentRequest;

	let open = $state(false);

	$effect(() => {
		open = !$req;
	});
</script>

<header>
	{#if $session.isLoggedIn()}
		{$session.user?.name}
		[{$session.user?.username}]
	{/if}
</header>
