<script module lang="ts">
	/** files up to this size are opened in the editor */
	export const MAX_EDIT_SIZE = 2 * 1024 * 1024;

	const LANGUAGES: Record<string, string> = {
		yml: 'yaml',
		yaml: 'yaml',
		json: 'json',
		js: 'javascript',
		mjs: 'javascript',
		ts: 'typescript',
		css: 'css',
		scss: 'scss',
		html: 'html',
		xml: 'xml',
		svg: 'xml',
		md: 'markdown',
		sh: 'shell',
		sql: 'sql',
		ini: 'ini',
		toml: 'ini',
		conf: 'ini',
		env: 'ini',
		php: 'php',
		py: 'python',
		rs: 'rust',
		dockerfile: 'dockerfile',
	};

	export function languageFor(name: string): string {
		const lower = name.toLowerCase();
		if (lower === 'dockerfile') return 'dockerfile';
		if (lower.startsWith('.env')) return 'ini';
		const ext = lower.split('.').pop() ?? '';
		return LANGUAGES[ext] ?? 'plaintext';
	}

	/** null if it isn't utf-8 text */
	export function decodeText(buf: ArrayBuffer): string | null {
		try {
			const text = new TextDecoder('utf-8', { fatal: true }).decode(buf);
			return text.includes('\0') ? null : text;
		} catch {
			return null;
		}
	}
</script>

<script lang="ts">
	import Editor from '@/components/Editor.svelte';

	let {
		name,
		text,
		readonly = false,
		onsave,
	}: {
		name: string;
		/** null for binary or too large files */
		text: string | null;
		readonly?: boolean;
		onsave: (text: string) => void;
	} = $props();

	let editor: Editor | undefined = $state();

	export function save() {
		editor?.save();
	}

	export function setValue(v: string) {
		editor?.setValue(v);
	}

	/** undefined if there is no editor (binary file) */
	export function getValue(): string | undefined {
		return editor?.getValue();
	}
</script>

{#if text === null}
	<p class="msg">
		This file is binary or larger than 2 MB, it can only be downloaded.
	</p>
{:else}
	<div class="editor" class:readonly>
		<Editor
			bind:this={editor}
			value={text}
			language={languageFor(name)}
			onsave={v => (readonly ? null : onsave(v))}
		/>
	</div>
{/if}

<style lang="scss">
	.msg {
		padding: 1rem;
		color: rgb(from var(--c-text) r g b / 70%);
	}

	.editor {
		display: flex;
		height: calc(100vh - 16rem);
		min-height: 20rem;
	}
</style>
