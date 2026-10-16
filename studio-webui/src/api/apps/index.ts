import { Api, readJsonLines } from '../lib';
import { loadServers, type Server } from '../servers/index';

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
	server: Server | null;
	createdOn: Date;
	servicesStates!: ServiceState[];

	constructor(data: any, server: Server | null = null) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
		this.server = server;
	}
}

export async function all() {
	const [servers, apps] = await Promise.all([loadServers(), api.get('')]);
	return apps.map((a: any) => new AppSummary(a, servers.get(a.serverId)));
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

	canStart(): boolean {
		return ['EXITED', 'PAUSED', 'UNHEALTHY', 'EMPTY'].includes(this.state);
	}

	canRestart(): boolean {
		return ['RUNNING', 'UNHEALTHY', 'PAUSED'].includes(this.state);
	}

	canStop(): boolean {
		return ['RUNNING', 'RESTARTING'].includes(this.state);
	}
}

export class App {
	id!: string;
	name!: string;
	teamId!: string;
	server: Server;
	createdOn: Date;
	services: Service[];

	constructor(data: any, server: Server) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
		this.server = server;
		this.services = data.services.map((s: any) => new Service(s));
	}
}

export async function byId(id: string) {
	const [servers, app] = await Promise.all([
		loadServers(),
		api.get(`/${id}`),
	]);
	return new App(app, servers.getOrFail(app.serverId));
}

export type CreateAppRequest = {
	id: string;
	name: string;
	serverId: string;
};

export class AppShort {
	id!: string;
	name!: string;
	teamId!: string;
	serverId!: string;
	createdOn: Date;

	constructor(data: any) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
	}
}

export async function create(data: CreateAppRequest) {
	const app = await api.post('', data);
	return new AppShort(app);
}

export type LogLine = {
	/** e.g. web-1 */
	container: string;
	service: string;
	/** RFC 3339 UTC with nanoseconds, sorts as a string */
	time: string;
	/** may contain ansi escape codes */
	line: string;
};

/**
 * Streams the last `lines` lines of each container (all if null) and with
 * `follow` the new ones until `signal` aborts. The lines of different
 * containers don't arrive in order.
 */
export async function logs(
	id: string,
	lines: number | null,
	follow: boolean,
	online: (line: LogLine) => void,
	signal: AbortSignal,
): Promise<void> {
	const resp = await api.requestRaw(
		'GET',
		`/${id}/logs`,
		{ lines, follow },
		{},
		{ signal },
	);
	await readJsonLines(resp, online);
}
