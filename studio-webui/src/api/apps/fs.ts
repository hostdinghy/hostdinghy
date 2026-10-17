import { api } from '.';

export type FsKind = 'file' | 'directory' | 'symlink' | 'other';

export class FsEntry {
	name!: string;
	kind!: FsKind;
	/** bytes, 0 for directories */
	size!: number;
	/** permission bits, e.g. 0o644 */
	mode!: number;
	uid!: number;
	gid!: number;
	modified: Date;
	linkTarget!: string | null;

	constructor(data: any) {
		Object.assign(this, data);
		this.modified = new Date(data.modified);
	}

	isDir(): boolean {
		return this.kind === 'directory';
	}

	/** rwxr-xr-x */
	modeStr(): string {
		const chars = 'rwxrwxrwx';
		let s = '';
		for (let i = 0; i < 9; i++)
			s += this.mode & (1 << (8 - i)) ? chars[i] : '-';
		return s;
	}

	modeOctal(): string {
		return this.mode.toString(8).padStart(3, '0');
	}
}

export type FsInfo = {
	entry: FsEntry;
	/** sorted, directories first, null if it isn't a directory */
	children: FsEntry[] | null;
};

/** `path` is relative to the app directory, '' is the app directory */
export async function info(appId: string, path: string): Promise<FsInfo> {
	const res = await api.get(`/${appId}/fs`, { path });
	return {
		entry: new FsEntry(res.entry),
		children: res.children?.map((c: any) => new FsEntry(c)) ?? null,
	};
}

export async function read(appId: string, path: string): Promise<Response> {
	return await api.requestRaw('GET', `/${appId}/fs/read`, { path });
}

/** a file, or a directory as .tar.gz */
export async function download(appId: string, path: string): Promise<Response> {
	return await api.requestRaw('GET', `/${appId}/fs/download`, { path });
}

export async function write(
	appId: string,
	path: string,
	file: File,
	overwrite: boolean,
	progress: ((percent: number) => void) | null = null,
): Promise<void> {
	const query = new URLSearchParams({
		path,
		overwrite: String(overwrite),
	});
	await api.requestWithFile(
		'PUT',
		`/${appId}/fs/write?${query}`,
		file,
		progress,
	);
}

export async function mkdir(appId: string, path: string): Promise<void> {
	await api.post(`/${appId}/fs/mkdir`, { path });
}

/** directories with everything inside */
export async function remove(appId: string, path: string): Promise<void> {
	await api.post(`/${appId}/fs/delete`, { path });
}

/** `to` is the new path, it must not exist */
export async function move(
	appId: string,
	from: string,
	to: string,
): Promise<void> {
	await api.post(`/${appId}/fs/move`, { from, to });
}

export async function copy(
	appId: string,
	from: string,
	to: string,
): Promise<void> {
	await api.post(`/${appId}/fs/copy`, { from, to });
}

export async function permissions(
	appId: string,
	req: {
		path: string;
		mode?: number;
		uid?: number;
		gid?: number;
		recursive: boolean;
	},
): Promise<void> {
	await api.post(`/${appId}/fs/permissions`, req);
}

/** packs path into a new zip file `to` */
export async function zip(
	appId: string,
	path: string,
	to: string,
): Promise<void> {
	await api.post(`/${appId}/fs/zip`, { path, to });
}

/** extracts a .zip, .tar or .tar.gz into a new directory `to` */
export async function extract(
	appId: string,
	path: string,
	to: string,
): Promise<void> {
	await api.post(`/${appId}/fs/extract`, { path, to });
}

export function isArchive(name: string): boolean {
	return /\.(zip|tar|tar\.gz|tgz)$/i.test(name);
}

export function joinPath(dir: string, name: string): string {
	return dir ? `${dir}/${name}` : name;
}

export function parentPath(path: string): string {
	const idx = path.lastIndexOf('/');
	return idx === -1 ? '' : path.slice(0, idx);
}

export function humanSize(bytes: number): string {
	const units = ['B', 'KB', 'MB', 'GB', 'TB'];
	let i = 0;
	while (bytes >= 1024 && i < units.length - 1) {
		bytes /= 1024;
		i++;
	}
	return `${i === 0 ? bytes : bytes.toFixed(1)} ${units[i]}`;
}
