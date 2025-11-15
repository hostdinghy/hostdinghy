import { api } from '.';

export async function loadCompose(appId: string): Promise<string> {
	return await api.get(`/${appId}/compose`);
}

export async function createCompose(
	appId: string,
	compose: string,
): Promise<string> {
	return await api.post(`/${appId}/compose`, {
		compose,
		createDatabase: false,
	});
}

export type ComposeCommand = 'start' | 'up' | 'restart' | 'stop';

export async function composeCommand(
	appId: string,
	service: string | null,
	command: ComposeCommand,
): Promise<void> {
	const url = service
		? `/${appId}/compose/service/${service}/${command}`
		: `/${appId}/compose/${command}`;
	return await api.post(url);
}
