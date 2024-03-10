<script>
	import { onMount, setContext } from 'svelte';
	import { writable } from 'svelte/store';

	import Button from '../components/mb-dropdown.svelte';
	import init, { open_file } from '../pkg/ppm_compiler.js';

	let loaded_file = writable({});
	let show_editor = writable(true);
	let show_viewer = writable(true);

	$: setContext('store', { loaded_file, show_editor, show_viewer });

	// Setup WASM module
	onMount(() => {
		init();

		window.onkeydown = function (e) {
			e.preventDefault();
			e.stopPropagation();
			if (e.ctrlKey && e.key == 'u') {
				open_file(setLoadedFile);
			} else if (e.ctrlKey && e.key == 's') {
				saveFile();
			} else if (e.ctrlKey && e.key == 'k') {
				toggleEditor();
			} else if (e.ctrlKey && e.shiftKey && e.key == 'p') {
				toggleViewer();
			}
		};

		window.onclick = () => {
			show_modal = false;
		};
	});

	function setLoadedFile(file) {
		loaded_file.set(JSON.parse(file));
	}

	function saveFile() {
		alert('Still WIP! For now, you can right click and save 🙏');
	}

	function toggleEditor() {
		show_editor.update((curr) => !curr);
	}

	function toggleViewer() {
		show_viewer.update((curr) => !curr);
	}

	const fileOptions = [
		{ label: 'Open', shortcut: '^U', action: () => open_file(setLoadedFile) },
		{ label: 'Save', shortcut: '^S', action: saveFile }
	];

	const viewOptions = [
		{ label: 'Toggle Details', shortcut: '^K', action: toggleEditor },
		{ label: 'Toggle Viewer', shortcut: '^Shift P', action: toggleViewer }
	];

	let show_modal = false;

	function toggleModal(e) {
		e.preventDefault();
		e.stopPropagation();
		show_modal = true;
	}
</script>

<div id="menu-bar">
	<p id="logo">saeko</p>
	<Button options={fileOptions}>File</Button>
	<Button options={viewOptions}>View</Button>
	<button on:click={toggleModal}>About</button>
</div>

<slot />

{#if show_modal}
	<div id="about-modal">
		<div>
			<div style="text-align: center; padding: 0px 40px;">
				<h2>🌸 ;saeko</h2>
				<p>
					A tiny supercharged PPM viewer. Created by <a
						target="_blank"
						href="https://github.com/Uzair-Fasih">Uzair Fasih</a
					>.
				</p>
			</div>
		</div>
	</div>
{/if}

<style>
	#menu-bar {
		display: flex;
		flex-direction: row;
		background-color: #2e2e2e;
		align-items: center;
	}

	#logo {
		background: var(--accent-color);
		padding: 0.5rem 1rem;
	}

	button {
		padding: 0.5rem 1rem;
		background: none;
		color: #fff;
		position: relative;
	}
	button:hover {
		cursor: pointer;
		background: #fff;
		color: #1a1a1a;
	}

	#about-modal {
		position: absolute;
		inset: 0;
		margin: auto;
		width: 100vw;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.25);
	}

	#about-modal > div {
		position: absolute;
		inset: 0;
		margin: auto;
		width: 400px;
		height: 200px;
		background-color: hsla(0, 0%, 14%, 1);
		border-radius: 5px;
		display: grid;
		place-items: center;
		padding: 10px 20px;
	}

	#about-modal p {
		font-size: 13px;
	}

	#about-modal a {
		color: var(--accent-color);
		text-decoration: underline;
	}

	h2 {
		font-size: 28px;
		word-spacing: -20px;
		margin-bottom: 15px;
	}
</style>
