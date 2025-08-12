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
