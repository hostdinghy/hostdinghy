import { Api } from '../lib';

export const api = new Api('/apps');

export type Service = {
	name: string;
	containerName: string;
	state: 'RUNNING';
	stateHr: string;
	routes: {
		rule: string;
		domains: string[];
	}[];
};

export class App {
	id!: string;
	name!: string;
	teamId!: string;
	serverId!: string;
	createdOn!: Date;
	services!: Service[];

	constructor(data: any) {
		Object.assign(this, {
			...data,
			createdOn: new Date(data.createdOn),
		});
	}
}

export async function all() {
	const apps: any[] = await api.get('');
	return apps.map(a => new App(a));
}

export async function byId(id: string) {
	const app = await api.get(`/${id}`);
	return new App(app);
}

export type CreateAppRequest = {
	id: string;
	name: string;
	serverId: string;
};

export async function create(data: CreateAppRequest) {
	const app = await api.post('', data);
	return new App(app);
}
