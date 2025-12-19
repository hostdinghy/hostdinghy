import { api } from '.';

export async function getFsResource(appId: string): Promise<string> {
	return await api.get(`/${appId}/fs`);
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
