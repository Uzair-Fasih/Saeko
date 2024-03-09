<script>
	import { getContext } from 'svelte';

	const isEmpty = (obj) =>
		[Object, Array].includes((obj || {}).constructor) && !Object.entries(obj || {}).length;
	const { loaded_file } = getContext('store');
</script>

<div id="editor">
	<p>; Saeko v0.10</p>
	{#if !isEmpty($loaded_file)}
		<p id="text-editor">
			{#if $loaded_file.status}
				Stats: <br />
				width: {$loaded_file.data.header.width} <br />
				height: {$loaded_file.data.header.height} <br />
				max_val: {$loaded_file.data.header.max_val}
			{:else}
				<p class="error">Error reading file: {$loaded_file.data}</p>
			{/if}
		</p>
	{:else}
		🦄 try opening a image.ppm file
	{/if}
</div>

<style>
	#editor {
		min-width: 40vw;
		height: 100%;
		margin: 1rem;
		font-size: 0.9rem;
	}

	#text-editor {
		overflow: scroll;
		height: 100%;
	}

	p {
		margin-bottom: 1rem;
	}

	.error {
		color: red;
	}
</style>
