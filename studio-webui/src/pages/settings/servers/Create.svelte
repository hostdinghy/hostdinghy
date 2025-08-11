<script lang="ts">
	import { create } from '@/api/servers';
	import Input from '@/form/Input.svelte';
	import Select from '@/form/Select.svelte';
	import { getRouter } from '@/main';
	import type { ResolvedProps } from '@/lib/LoadProps';
	import { errorToStr } from '@/api/lib';
	import Textarea from '@/form/Textarea.svelte';

	const router = getRouter();

	let name = $state('');
	let domain = $state('');
	let apiToken = $state('');
	let tlsCert = $state('');
	let error = $state('');

	async function onsubmit(e: Event) {
		e.preventDefault();

		error = '';

		try {
			const app = await create({
				name,
				domain,
				apiToken,
				tlsCert,
			});
			router.open('/settings/servers');
		} catch (e) {
			console.error('Error creating app:', e);
			error = errorToStr(e);
		}
	}
</script>

<svelte:head>
	<title>HostDinghy</title>
</svelte:head>

<div id="createApp">
	<div class="center">
		<h1>Create Server</h1>
		<form {onsubmit}>
			<Input
				id="name"
				name="name"
				type="text"
				label="name"
				placeholder="Enter server name..."
				bind:value={name}
				required
			/>

			<Input
				id="domain"
				name="domain"
				type="text"
				label="domain"
				placeholder="Enter domain..."
				bind:value={domain}
				required
			/>

			<Input
				id="api-token"
				name="api-token"
				type="text"
				label="api token"
				placeholder="Enter API token..."
				bind:value={apiToken}
				required
			/>

			<Textarea
				id="tls-cert"
				name="tls-cert"
				label="TLS cert"
				placeholder="Enter TLS certificate..."
				bind:value={tlsCert}
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
	#createApp {
		display: flex;
		width: 100%;
		min-height: 100vh;
		justify-content: center;
		align-items: center;
	}

	h1 {
		font-size: 1.25rem;
		margin-bottom: 2rem;
	}

	form {
		width: 30rem;
	}

	.error {
		margin-top: 1.5rem;
		color: var(--red);
	}
</style>
