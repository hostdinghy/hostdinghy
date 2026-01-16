<script lang="ts">
	import Button from '@/components/Button.svelte';
	import Field from './Field.svelte';

	let {
		id,
		name,
		label,
		file = $bindable(),
		bx,
	}: {
		id: string;
		name: string;
		label: string;
		file: File | null;
		/** border x */
		bx?: boolean;
	} = $props();

	let activeDrag = $state(0);
	let areaEl: HTMLDivElement;
	// svelte-ignore non_reactive_update
	let fileInput: HTMLInputElement;

	function onclick(e: MouseEvent) {
		e.preventDefault();
		fileInput.click();
	}

	function ondrop(e: DragEvent) {
		e.preventDefault();
		activeDrag = 0;
		sendFiles(e.dataTransfer!.files);
	}

	function ondragover(e: Event) {
		e.preventDefault();
		e.stopPropagation();
	}

	function ondragenter(e: Event) {
		e.preventDefault();
		activeDrag++;
	}

	function ondragleave(e: Event) {
		e.preventDefault();
		activeDrag = Math.max(0, activeDrag - 1);
	}

	function onfilechange(_e: Event) {
		sendFiles(fileInput.files!);
	}

	function sendFiles(fileList: FileList) {
		const files = Array.from(fileList).filter(file => {
			// remove empty unknown files
			return file.size > 0 && file.type.length > 0;
		});

		if (files.length === 0) return alert('No file selected');

		if (files.length > 1) return alert('Only one file is allowed');

		file = files[0];
	}
</script>

<Field {id} {label} {bx}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		bind:this={areaEl}
		class="upload"
		class:over={activeDrag > 0}
		{ondrop}
		{ondragover}
		{ondragenter}
		{ondragleave}
	>
		{#if file}
			<p>{file.name}</p>
			<Button type="button" style="text" onclick={() => (file = null)}>
				Remove file
			</Button>
		{:else}
			<p id="{id}-help" class="help">Drag and drop file or</p>
			<Button
				type="button"
				style="text"
				{onclick}
				aria-describedby="{id}-help"
			>
				Select file
			</Button>

			<input
				bind:this={fileInput}
				type="file"
				{id}
				class="sr-only"
				{name}
				aria-describedby="{id}-help"
				onchange={onfilechange}
			/>
		{/if}
	</div>
</Field>

<style lang="scss">
	.upload {
		padding: 1rem 1.5rem;

		:global(.btn) {
			margin-top: 0.5rem;
		}
	}

	.over {
		background-color: rgb(from var(--c-accent) r g b / 10%);
	}

	.help {
		color: var(--c-label);
	}
</style>
