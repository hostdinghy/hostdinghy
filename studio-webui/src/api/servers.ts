import { Api } from './lib';

const api = new Api('/servers');

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

export async function loadServers() {
	const servers: unknown[] = await api.get('');
	return servers.map(a => new Server(a));
}

export interface CreateServerRequest {
	name: string;
	domain: string;
	apiToken: string;
	tlsCert: string;
}

export async function create(data: CreateServerRequest) {
	const app = await api.post('', data);
	return new Server(app);
}
