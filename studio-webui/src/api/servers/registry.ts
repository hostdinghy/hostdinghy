/// all these request return all registry users from the current server
/// but they are exposed under apps for the moment

import { api } from '.';

export async function loadRegistryUsers(serverId: string): Promise<string[]> {
	return await api.get(`/${serverId}/registry/users`);
}

export async function createRegistryUser(
	serverId: string,
	username: string,
): Promise<{ username: string; password: string }> {
	return await api.post(`/${serverId}/registry/users`, { username });
}

export async function deleteRegistryUser(
	serverId: string,
	username: string,
): Promise<void> {
	return await api.delete(
		`/${serverId}/registry/users/${encodeURIComponent(username)}`,
	);
}
