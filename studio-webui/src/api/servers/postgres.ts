/// all these request return all registry users from the current server
/// but they are exposed under apps for the moment

import { api } from '.';

export async function loadDatabases(serverId: string): Promise<string[]> {
	return await api.get(`/${serverId}/postgres/databases`);
}

export async function createDatabase(
	serverId: string,
	name: string,
): Promise<{ name: string; password: string }> {
	return await api.post(`/${serverId}/postgres/databases`, { name });
}

export async function newPassword(
	serverId: string,
	database: string,
): Promise<{ name: string; password: string }> {
	return await api.post(
		`/${serverId}/postgres/databases/${database}/password`,
		{},
	);
}

// export async function deleteRegistryUser(
// 	serverId: string,
// 	username: string,
// ): Promise<void> {
// 	return await api.delete(
// 		`/${serverId}/registry/users/${encodeURIComponent(username)}`,
// 	);
// }
