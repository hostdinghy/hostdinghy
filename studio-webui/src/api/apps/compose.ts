import { getSessionHeaders } from '../users';
import { newApi } from '../utils';

const api = newApi('/apps');

export async function get(appId: string) {
	const compose: string = await api.request(
		'GET',
		`/${appId}/compose`,
		null,
		getSessionHeaders(),
	);

	return compose;
}

export type CreateComposeRequest = {
	compose: string;
	createDatabase: boolean;
};

export async function set(appId: string, data: CreateComposeRequest) {
	const compose: any = await api.request(
		'POST',
		`/${appId}/compose`,
		data,
		getSessionHeaders(),
	);

	return compose;
}
