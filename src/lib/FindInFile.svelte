<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

	const dispatch = createEventDispatcher();

	let query = $state('');
	let input: HTMLInputElement;

	onMount(() => {
		input?.focus();
	});

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			dispatch('close');
		}
	}
</script>

<div class="find-bar">
	<input
		bind:this={input}
		bind:value={query}
		onkeydown={handleKeyDown}
		placeholder="Find in file..."
		class="find-input"
	/>
	<button class="close-btn" onclick={() => dispatch('close')}>×</button>
</div>

<style>
	.find-bar {
		position: absolute;
		top: 8px;
		right: 8px;
		background: var(--bg-secondary);
		border: 1px solid var(--border);
		border-radius: 6px;
		display: flex;
		align-items: center;
		padding: 4px 4px 4px 12px;
		box-shadow: 0 2px 8px rgba(42, 36, 31, 0.1);
		z-index: 10;
	}

	.find-input {
		width: 200px;
		padding: 4px;
		font-size: 13px;
		border: none;
		background: transparent;
	}

	.close-btn {
		width: 24px;
		height: 24px;
		border-radius: 2px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 18px;
		color: var(--text-secondary);
		transition: all 0.15s;
		border: none;
		background: transparent;
		padding: 0;
		cursor: pointer;
	}

	.close-btn:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}
</style>
