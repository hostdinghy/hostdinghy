import { api } from '.';
import { ApiError, readJsonLines } from '../lib';

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

export type CommandEvent =
	| { type: 'output'; line: string }
	| { type: 'done' }
	| { type: 'error'; message: string };

/**
 * Runs the command and calls `online` for each line of its output as it
 * arrives. Resolves when the command is done.
 *
 * @throws - If the command could not be started or failed
 */
export async function composeCommand(
	appId: string,
	service: string | null,
	command: ComposeCommand,
	online: (line: string) => void = () => {},
): Promise<void> {
	const url = service
		? `/${appId}/compose/service/${service}/${command}`
		: `/${appId}/compose/${command}`;
	const resp = await api.requestRaw('POST', url);

	let result: CommandEvent | null = null;
	try {
		await readJsonLines(resp, (event: CommandEvent) => {
			if (event.type === 'output') online(event.line);
			else result = event;
		});
	} catch (e: any) {
		throw ApiError.newRequest('Lost the connection: ' + e.message);
	}

	result = result as CommandEvent | null;
	if (!result)
		throw ApiError.newRequest(
			'The output ended early, the command might still be running',
		);
	if (result.type === 'error') throw new ApiError(result.message);
}

export const COMMAND_TITLES: Record<ComposeCommand, string> = {
	start: 'Starting',
	up: 'Pulling and starting',
	restart: 'Restarting',
	stop: 'Stopping',
};
