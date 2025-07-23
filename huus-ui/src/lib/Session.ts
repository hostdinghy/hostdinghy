import {
	Authenticated,
	tokenAuth,
	type Session as SessionData,
	type User,
} from '@/api/users';
import Listeners from 'chuchi-utils/sync/Listeners';
import { getContext } from 'svelte';

/**
 * ## Note
 * If you use this in a component use it with $session to make sure you
 * get updated if someting changes
 */
export default class Session {
	// if shortSession is defined shortUser is defined as well
	inner: SessionData | null;
	user: User | null;
	listeners: Listeners<[]>;

	constructor() {
		this.inner = null;
		this.user = null;
		this.listeners = new Listeners();
	}

	// will not throw
	static async init(): Promise<Session> {
		const me = new Session();

		const auth = await authed();
		if (auth) {
			me.inner = auth.session;
			me.user = auth.user;
		}

		globalThis.SESSION = me;

		return me;
	}

	subscribe(fn: (sess: Session) => void): () => void {
		const rm = this.listeners.add(() => fn(this));
		fn(this);

		return rm;
	}

	isLoggedIn(): boolean {
		return !!this.inner;
	}

	setAuthed(auth: Authenticated) {
		this.inner = auth.session;
		this.user = auth.user;
		localStorage.setItem('SESSION_TOKEN', this.inner.token);
		this.listeners.trigger();
	}
}

async function authed(): Promise<Authenticated | null> {
	const sessionToken = localStorage.getItem('SESSION_TOKEN');
	if (!sessionToken) return null;

	try {
		return await tokenAuth(sessionToken);
	} catch (e) {
		console.error('Error getting session', e);
		return null;
	}
}

export function getSession(): Session {
	return globalThis.SESSION;
}
