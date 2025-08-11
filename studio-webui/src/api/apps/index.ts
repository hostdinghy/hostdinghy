import { Api } from '../lib';

export const api = new Api('/apps');

export type ServiceState =
	| 'EMTPY'
	| 'CREATED'
	| 'UNHEALTHY'
	| 'RUNNING'
	| 'PAUSED'
	| 'RESTARTING'
	| 'EXITED'
	| 'REMOVING'
	| 'DEAD'
	| 'UNKNOWN';

export class AppSummary {
	id!: string;
	name!: string;
	teamId!: string;
	serverId!: string;
	createdOn: Date;
	servicesStates!: ServiceState[];

	constructor(data: any) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
	}
}

export async function all() {
	const apps: any[] = await api.get('');
	return apps.map(a => new AppSummary(a));
}

export class Service {
	name!: string;
	containerName!: string;
	state!: ServiceState;
	stateHr!: string;
	routes!: {
		rule: string;
		domains: string[];
	}[];

	constructor(data: any) {
		Object.assign(this, data);
	}
}

export class App {
	id!: string;
	name!: string;
	teamId!: string;
	serverId!: string;
	createdOn: Date;
	services: Service[];

	constructor(data: any) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
		this.services = data.services.map((s: any) => new Service(s));
	}
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
