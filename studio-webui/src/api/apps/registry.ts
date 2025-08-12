/// all these request return all registry users from the current server
/// but they are exposed under apps for the moment

import { api } from '.';

export async function loadRegistryUsers(appId: string): Promise<string[]> {
	return await api.get(`/${appId}/registry/users`);
}

export async function createRegistryUser(
	appId: string,
	username: string,
): Promise<{ username: string; password: string }> {
	return await api.post(`/${appId}/registry/users`, { username });
}

export async function deleteRegistryUser(
	appId: string,
	username: string,
): Promise<void> {
	return await api.delete(
		`/${appId}/registry/users/${encodeURIComponent(username)}`,
	);
}
