<script>
	import { onMount } from 'svelte';
	import { getContext } from 'svelte';

	import Editor from '../components/editor.svelte';
	import Canvas from '../components/canvas.svelte';

	const { show_editor, show_viewer } = getContext('store');
</script>

<main>
	<div id="editor-pane" style={`max-width: ${$show_editor ? '100%' : '0%'}`}>
		<Editor />
	</div>

	<div id="viewer-pane" style={`max-width: ${$show_viewer ? '100%' : '0%'}`}>
		<Canvas />
	</div>
</main>

<style>
	main {
		display: flex;
		flex-direction: row;
		height: 100%;
	}

	#editor-pane {
		flex: 1 1 40%;
		height: 100%;
		transition: 1s ease;
		overflow: hidden;
	}

	#viewer-pane {
		flex: 1 1 60%;
		height: 100%;
		transition: 1s ease;
		overflow: hidden;
	}
</style>
