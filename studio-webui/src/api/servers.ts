import { getSessionHeaders } from './users';
import { newApi } from './utils';

const api = newApi('/servers');

export class Server {
	id!: string;
	name!: string;
	teamId!: string;
	addr!: string;
	apiToken!: string;
	tlsCert!: string;
	createdOn!: Date;

	constructor(data: any) {
		Object.assign(this, {
			...data,
			createdOn: new Date(data.created_on),
		});
	}
}

export async function all() {
	const apps: any[] = await api.request('GET', '', null, getSessionHeaders());

	return apps.map(a => new Server(a));
}

export interface CreateServerRequest {
	name: string;
	addr: string;
	apiToken: string;
	cert: string;
}

export async function create(data: CreateServerRequest) {
	const app: any = await api.request('POST', '', data, getSessionHeaders());

	return new Server(app);
}
