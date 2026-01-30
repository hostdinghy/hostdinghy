import App from './App.svelte';
import * as routes from './pages/routes';
import { handleRoute } from './main';
import { SsrCache } from 'chuchi/ssr';
import { Router as ChuchiRouter } from 'chuchi';
import { mount, tick } from 'svelte';
import { Writable } from 'chuchi/stores';
import Session from './lib/Session';
import LoadProps from './lib/LoadProps';
import type { Router } from './lib';

async function main() {
	const cache = new SsrCache();
	const router: Router = new ChuchiRouter();

	const context = new Map();
	context.set('router', router);

	routes.register(router);

	const session = await Session.init();
	context.set('session', session);

	let mounted = false;
	const loadingStore = new Writable<number>(0);
	const pageStore = new Writable<any>(null);

	const startLoading = () => loadingStore.set(loadingStore.get() + 1);
	const stopLoading = () => loadingStore.set(loadingStore.get() - 1);

	router.onRoute(async (req, route, routing) => {
		startLoading();
		const loadProps = new LoadProps({
			router,
			route,
			req,
			cache,
			session,
		});
		const { page, redirect } = await handleRoute(req, route, loadProps);

		if (redirect) {
			// todo handle the request?
			router.open(redirect);
			stopLoading();
			return;
		}

		if (await routing.dataReady()) {
			stopLoading();
			return;
		}

		pageStore.set(page);
		stopLoading();

		if (!mounted) {
			mounted = true;
			mount(App, {
				target: document.body,
				props: { page: pageStore, loading: loadingStore },
				context,
			});
		}

		await tick();

		routing.domReady();
	});

	router.initClient();
}
main();
