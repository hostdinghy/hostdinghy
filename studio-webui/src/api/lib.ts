import { getSession } from '@/lib/Session';

const DEF_ORIGIN = 'http://hostdinghy';

export class Api {
	addr: string;

	/**
	 * Create a new API instance
	 * @param uri The base URI of the API
	 * needs to start with /
	 */
	constructor(uri: string) {
		if (!uri.startsWith('/')) throw new Error('uri needs to start with /');

		/* @ts-ignore */
		this.addr = window.API_ADDR + 'api' + uri;
	}

	/**
	 * Send a request to the server
	 *
	 * @param method - The method of the request
	 * @param path - The path of the request
	 * @param data - The data to be sent if the method is get the data will
	 * be sent as query params
	 * @param headers - The headers to be sent
	 * @param opts - The additional options to be sent to fetch
	 *
	 * @returns The response of the request
	 *
	 * @throws - If the request fails
	 */
	async request(
		method: string,
		path: string,
		data: object | null = null,
		headers: object = {},
		opts: any = {},
	): Promise<any> {
		const validPath =
			path === '' || (path.startsWith('/') && !path.endsWith('/'));
		if (!validPath) {
			console.log('path', path);
			throw new Error(
				'path needs to either be empty or start with / and not end with /',
			);
		}

		let err: ApiError;

		// since URL does not accept an url with out a host we need to add
		// a default host, which we will remove later
		// the URL is used in the first place to allow to modify the
		// search params
		// todo: there might still be an issue with relative and absolute
		// paths, but i'm not sure fetch supports relative paths?
		const url = new URL(this.addr + path, DEF_ORIGIN);

		try {
			const fetchParams = {
				headers,
				method,
				...opts,
			};
			fetchParams.headers['content-type'] = 'application/json';

			const session = getSession();
			// session might not be defined
			if (session?.inner?.isValid()) {
				fetchParams.headers['session-token'] = session.inner.token;
			}

			// don't send a body if the method is get
			if (method.toLowerCase() === 'get') {
				const searchParams = url.searchParams;

				for (const [key, value] of Object.entries(data ?? {})) {
					if (value !== undefined && value !== null)
						searchParams.set(key, value);
				}
			} else {
				fetchParams.body = JSON.stringify(data);
			}

			let urlStr = url.toString();
			if (urlStr.startsWith(DEF_ORIGIN))
				urlStr = urlStr.substring(DEF_ORIGIN.length);

			const resp = await fetch(urlStr, fetchParams);

			opts.responseStatus = resp.status;
			opts.responseHeaders = resp.headers;

			if (resp.ok) {
				return await resp.json();
			} else {
				// we've got and error
				const errObj = await resp.json();
				err = ApiError.fromJson(errObj);
			}
		} catch (e: any) {
			console.error('request error raw', e);
			err = ApiError.fromAny(e);
		}

		console.error('request error', err);
		throw err;
	}

	async get(
		path: string,
		data: object | null = null,
		headers: object = {},
		opts: any = {},
	): Promise<any> {
		return this.request('GET', path, data, headers, opts);
	}

	async post(
		path: string,
		data: object | null = null,
		headers: object = {},
		opts: any = {},
	): Promise<any> {
		return this.request('POST', path, data, headers, opts);
	}

	async put(
		path: string,
		data: object | null = null,
		headers: object = {},
		opts: any = {},
	): Promise<any> {
		return this.request('PUT', path, data, headers, opts);
	}

	async delete(
		path: string,
		data: object | null = null,
		headers: object = {},
		opts: any = {},
	): Promise<any> {
		return this.request('DELETE', path, data, headers, opts);
	}
}

export type ComposeError =
	| {
			type: 'PARSING';
			detail: string;
	  }
	| {
			type: 'UNEXPECTED_TRAEFIK_ROUTER_NAME';
			detail: {
				unexpected: string[];
				expected: string;
			};
	  }
	| {
			type: 'INVALID_IMAGE';
			detail: {
				image: string;
				expected: string;
			};
	  };

export type ErrorType =
	| {
			type:
				| 'LOGIN_INCORRECT'
				| 'MISSING_SESSION_TOKEN'
				| 'INVALID_SESSION_TOKEN'
				| 'INVALID_USER'
				| 'INSUFFICIENT_RIGHTS'
				| 'NOT_FOUND';
	  }
	| {
			type: 'COMPOSE';
			detail: ComposeError;
	  }
	| {
			type: 'INTERNAL_API_SERVER';
			detail: string;
	  }
	| {
			type: 'INTERNAL';
			detail: string;
	  }
	| {
			type: 'REQUEST';
			detail: string;
	  };

export class ApiError extends Error {
	inner: ErrorType;

	constructor(message: string, type: ErrorType | null = null) {
		super(message);

		this.inner = type ?? {
			type: 'INTERNAL',
			detail: message,
		};
	}

	get type(): ErrorType['type'] {
		return this.inner?.type;
	}

	static fromJson(err: any) {
		return new ApiError(err.message, err.error);
	}

	static fromAny(err: any) {
		// todo better
		return new ApiError(err.message);
	}
}

export function errorToStr(err: any): string {
	if (!(err instanceof ApiError)) return err.message;

	const e = err.inner;
	switch (e.type) {
		case 'LOGIN_INCORRECT':
			return 'Username or password is incorrect.';
		default:
			return err.message;
	}
}
