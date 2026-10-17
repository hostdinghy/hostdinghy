/**
 * Saves a response as a file, streamed to disk if the browser can
 * (showSaveFilePicker), otherwise through a blob.
 *
 * @param onProgress - called with the received bytes and the total from
 * the content-length (null if unknown, like a packed directory)
 *
 * @returns false if the user cancelled the save dialog
 */
export async function saveResponse(
	resp: Response,
	filename: string,
	onProgress: (received: number, total: number | null) => void = () => {},
): Promise<boolean> {
	const length = resp.headers.get('content-length');
	const total = length ? parseInt(length) : null;
	let received = 0;
	const body = resp.body!.pipeThrough(
		new TransformStream<Uint8Array, Uint8Array>({
			transform(chunk, ctrl) {
				received += chunk.byteLength;
				onProgress(received, total);
				ctrl.enqueue(chunk);
			},
		}),
	);

	// @ts-ignore
	if (typeof window.showSaveFilePicker === 'function') {
		let fileHandle;
		try {
			// @ts-ignore
			fileHandle = await window.showSaveFilePicker({
				suggestedName: filename,
			});
		} catch (e) {
			if ((e as DOMException)?.name === 'AbortError') return false;
			throw e;
		}

		const writable = await fileHandle.createWritable();
		await body.pipeTo(writable);
		return true;
	}

	const blob = await new Response(body).blob();
	const el = document.createElement('a');
	const url = URL.createObjectURL(blob);
	document.body.appendChild(el);
	el.href = url;
	el.download = filename;
	el.click();
	el.remove();
	URL.revokeObjectURL(url);
	return true;
}

/** the filename of a content-disposition header */
export function dispositionFilename(resp: Response): string | null {
	const header = resp.headers.get('content-disposition') ?? '';
	return header.match(/filename="([^"]+)"/)?.[1] ?? null;
}
