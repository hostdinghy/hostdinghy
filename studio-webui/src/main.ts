import type { Request, Route, Router } from 'chuchi';
import * as routes from './pages/routes';
import type { SsrCache } from 'chuchi/ssr';
import { getContext } from 'svelte';
import type { Cookies } from 'chuchi/cookies';
import type LoadProps from './lib/LoadProps';
import type { Writable } from 'chuchi/stores';

export function getRouter(): Router {
	return getContext('router');
}

export function getCookies(): Cookies {
	return getContext('cookies');
}

export function getRequest(): Writable<Request> {
	return getRouter().currentRequest;
}

export type RoutePage = {
	layout: any;
	component: any;
	props: Record<string, any>;
};

export type RouteResponse = {
	status: number;
	page?: RoutePage;
	redirect?: string;
};

const ERROR_404: RouteResponse = {
	status: 404,
	page: {
		layout: null,
		component: routes.NotFound,
		props: {},
	},
};

// should return { status, props,  }
export async function handleRoute(
	req: Request,
	route: Route | null,
	loadProps: LoadProps,
): Promise<RouteResponse> {
	if (!route) return ERROR_404;

	let comp: any;
	let pageProps: Record<string, any>;
	try {
		comp = await route.load(req);

		const requiresRights =
			comp.requiresRights !== undefined ? comp.requiresRights : 'normal';
		const requiresUser = Array.isArray(requiresRights)
			? requiresRights.length > 0
			: !!requiresRights;

		if (requiresUser && !loadProps.session.isLoggedIn()) {
			return {
				status: 302,
				redirect:
					'/signin?' +
					new URLSearchParams({ url: req.url.pathname }).toString(),
			};
		}

		pageProps = route.toProps(req);

		if (typeof comp.loadProps === 'function') {
			const nProps = await comp.loadProps(pageProps, loadProps);
			if (nProps) pageProps = nProps;
		}

		if (loadProps.redirect) {
			return {
				status: loadProps.redirect.status,
				redirect: loadProps.redirect.url,
			};
		}
	} catch (e) {
		console.log('error', e);
		return {
			status: 500,
			page: {
				layout: null,
				component: routes.NotFound,
				props: {},
			},
		};
	}

	return {
		status: 200,
		page: {
			layout: comp?.layout ?? null,
			component: comp.default,
			props: pageProps,
		},
	};
}
