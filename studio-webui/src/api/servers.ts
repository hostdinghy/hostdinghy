import { Api } from './lib';

const api = new Api('/servers');

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
	const apps: any[] = await api.get('');

	return apps.map(a => new Server(a));
}

export interface CreateServerRequest {
	name: string;
	addr: string;
	apiToken: string;
	tlsCert: string;
}

export async function create(data: CreateServerRequest) {
	const app: any = await api.post('', data);

	return new Server(app);
}
