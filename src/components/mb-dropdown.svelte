<script>
	export let options = [];
</script>

<div class="mb-dropdown">
	<button>
		<slot />
	</button>
	<div class="dropdown">
		{#each options as option (option.label)}
			<button on:click={option.action}>
				<p>{option.label}</p>
				<p class="shortcut">{option.shortcut}</p>
			</button>
		{/each}
	</div>
</div>

<style>
	.mb-dropdown {
		position: relative;
	}

	.mb-dropdown > button {
		padding: 0.5rem 1rem;
		background: none;
		color: #fff;
		position: relative;
	}
	.mb-dropdown:hover > button {
		cursor: pointer;
		background: #fff;
		color: #1a1a1a;
	}

	.dropdown {
		position: absolute;
		display: flex;
		flex-direction: column;
		bottom: -200%;
		min-width: 10rem;
		width: max-content;
		background: #fff;
		display: none;
	}
	.mb-dropdown:hover .dropdown {
		display: flex;
	}

	.dropdown > button {
		color: #1a1a1a;
		padding: 0.5rem 1rem;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
	}
	.dropdown > button:hover {
		background: #cccccc;
		cursor: pointer;
	}

	.shortcut {
		color: #8a8a8a;
		margin-left: 1rem;
	}
</style>
