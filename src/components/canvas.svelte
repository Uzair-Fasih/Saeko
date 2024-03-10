<script>
	import { getContext, afterUpdate } from 'svelte';

	const isEmpty = (obj) =>
		[Object, Array].includes((obj || {}).constructor) && !Object.entries(obj || {}).length;
	const { loaded_file } = getContext('store');

	let canvas = null;
	afterUpdate(() => {
		// ...the DOM is now in sync with the data
		if (!canvas) return;
		const ctx = canvas.getContext('2d');
		const canvasWidth = canvas.width;
		const canvasHeight = canvas.height;

		ctx.clearRect(0, 0, canvasWidth, canvasHeight);
		const id = ctx.getImageData(0, 0, canvasWidth, canvasHeight);
		const pixels = id.data;
		const paint = $loaded_file.data.data.pixels;

		for (let idx = 0, jdx = 0; idx < paint.length; idx++) {
			pixels[jdx++] = paint[idx][0];
			pixels[jdx++] = paint[idx][1];
			pixels[jdx++] = paint[idx][2];
			pixels[jdx++] = 255;
		}

		ctx.putImageData(id, 0, 0);
	});
</script>

<div class="container">
	{#if !isEmpty($loaded_file) && $loaded_file.status}
		<canvas
			bind:this={canvas}
			width={$loaded_file.data.header.width}
			height={$loaded_file.data.header.height}
		></canvas>
	{:else}
		<h1>; saeko</h1>
	{/if}
</div>

<style>
	.container {
		min-width: 60vw;
		height: 100%;
		width: 100%;
		background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='20' height='20'><circle cx='10' cy='10' r='1' fill='%23555'></circle></svg>");
		background-size: 10px 10px;
		background-position: -4px -4px;
		display: grid;
		place-items: center;
	}

	h1 {
		font-size: 64px;
		color: #b9b9b9;
	}

	canvas {
		background: white;
	}
</style>
