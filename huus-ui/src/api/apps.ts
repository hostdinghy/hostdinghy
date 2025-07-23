import { getSessionHeaders } from './users';
import { newApi } from './utils';

const api = newApi('/apps');

export class App {
	id!: string;
	name!: string;
	teamId!: string;
	serverId!: string;
	createdOn!: Date;

	constructor(data: any) {
		Object.assign(this, {
			...data,
			createdOn: new Date(data.createdOn),
		});
	}
}

export async function all() {
	const apps: any[] = await api.request('GET', '', null, getSessionHeaders());

	return apps.map(a => new App(a));
}

export async function byId(id: string) {
	const app: any = await api.request(
		'GET',
		`/${id}`,
		null,
		getSessionHeaders(),
	);

	return new App(app);
}

export type CreateAppRequest = {
	id: string;
	name: string;
	serverId: string;
};

export async function create(data: CreateAppRequest) {
	const app: any = await api.request('POST', '', data, getSessionHeaders());

	return new App(app);
}
