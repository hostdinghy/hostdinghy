/**
 * Saves a response as a file, streamed to disk if the browser can
 * (showSaveFilePicker), otherwise through a blob.
 *
 * @returns false if the user cancelled the save dialog
 */
export async function saveResponse(
	resp: Response,
	filename: string,
): Promise<boolean> {
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
		await resp.body!.pipeTo(writable);
		return true;
	}

	const blob = await resp.blob();
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
