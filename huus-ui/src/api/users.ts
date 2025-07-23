import { getSession } from '@/lib/Session';
import { newApi } from './utils';
import { ApiError } from 'chuchi/api';

const api = newApi('/users');

export class Session {
	token!: string;
	timeout!: Date;

	constructor(data: any) {
		Object.assign(this, data);
		this.timeout = new Date(data.timeout);
	}

	isValid() {
		return this.timeout < Date.now();
	}
}

export class User {
	id!: string;
	username!: string;
	name!: string;
	createdOn!: Date;

	constructor(data: any) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
	}
}

export class Authenticated {
	session!: Session;
	user!: User;

	constructor(data: any) {
		this.session = new Session(data.session);
		this.user = new User(data.user);
	}
}

export async function login(username: string, password: string) {
	const d = await api.request('POST', '/login', { username, password });

	return new Authenticated(d);
}

export async function tokenAuth(token: string): Promise<Authenticated> {
	const d = await api.request('POST', '/tokenauth', null, {
		'session-token': token,
	});

	return new Authenticated(d);
}

export function getSessionHeaders() {
	const session = getSession();

	if (!session.inner?.isValid()) {
		throw new ApiError('InvalidSessionToken', 'session-token invalid.');
	}

	return {
		'session-token': session.inner.token,
	};
}
