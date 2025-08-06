import { Api } from '../lib';
import { api } from './index';

export async function get(appId: string): Promise<string> {
	return await api.get(`/${appId}/compose`);
}

export async function set(appId: string, compose: string): Promise<string> {
	return await api.post(`/${appId}/compose`, {
		compose,
		createDatabase: false,
	});
}
