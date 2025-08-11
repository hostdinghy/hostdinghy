import { Api } from './lib';

const api = new Api('/servers');

/**
 * Servers is a bit different then the other apis
 *
 * It is a singleton which you can get by calling loadServers
 * it will cache its data and can be used to get a server for example
 * by its id
 */
export default class Servers {
	inner: Server[];
	lookup: Map<string, Server>;

	constructor(servers: Server[]) {
		this.inner = servers;
		this.lookup = new Map(servers.map(s => [s.id, s]));
	}

	all(): Server[] {
		return this.inner;
	}

	first(): Server | null {
		return this.inner[0] ?? null;
	}

	get(id: string): Server | null {
		return this.lookup.get(id) ?? null;
	}

	_insert(server: Server) {
		this.inner.push(server);
		this.lookup.set(server.id, server);
	}
}

export class Server {
	id!: string;
	teamId!: string;
	name!: string;
	domain!: string;
	registryDomain!: string | null;
	/** if this is empty the server could not be reached */
	version!: string | null;
	createdOn: Date;

	constructor(data: any) {
		Object.assign(this, data);
		// make sure we always set the type createdOn
		this.createdOn = new Date(data.created_on);
	}
}

let cachedServers: Servers | null = null;

export async function loadServers() {
	if (cachedServers) return cachedServers;

	const servers: unknown[] = await api.get('');
	cachedServers = new Servers(servers.map(a => new Server(a)));

	return cachedServers;
}

export type CreateServerRequest = {
	name: string;
	domain: string;
	apiToken: string;
	tlsCert: string;
};

export async function loadServer(id: string) {
	const servers = await loadServers();
	return servers.get(id);
}

export async function create(data: CreateServerRequest) {
	const app = await api.post('', data);
	const server = new Server(app);

	if (cachedServers) cachedServers._insert(server);

	return server;
}
