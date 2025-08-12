import type { Request } from 'chuchi';
import type { SsrCache } from 'chuchi/ssr';
import type Session from './Session';
import type { Snippet } from 'svelte';
import type { Route, Router } from '.';

export default class LoadProps {
	router: Router;
	route: Route | null;
	req: Request;
	cache: SsrCache;
	session: Session;
	redirect: { status: number; url: string } | null;

	constructor(obj: {
		router: Router;
		route: Route | null;
		req: Request;
		cache: SsrCache;
		session: Session;
	}) {
		this.router = obj.router;
		this.route = obj.route;
		this.req = obj.req;
		this.cache = obj.cache;
		this.session = obj.session;
		this.redirect = null;
	}

	setRedirect(url: string, status = 302) {
		this.redirect = { status, url };
	}
}

// not sure the types here are good
export type LoadPropsFn<
	P extends Record<string, any> = object,
	R extends Record<string, any> = object,
> = (props: P, lp: LoadProps) => Promise<R | null | void> | void;

export type ResolvedProps<T extends (...args: any) => any> = Awaited<
	ReturnType<T>
>;

export type LayoutProps<T extends (...args: any) => any> = ResolvedProps<T> & {
	children: Snippet;
};
