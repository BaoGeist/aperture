<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	const dispatch = createEventDispatcher();

	let query = $state('');
	let paths = $state<string[]>([]);
	let selectedIndex = $state(0);
	let input: HTMLInputElement;

	onMount(async () => {
		input?.focus();
		await loadPaths();
	});

	async function loadPaths() {
		try {
			paths = await invoke('zoxide_query', { query: query || '' });
		} catch (e) {
			console.error('Zoxide query failed:', e);
			paths = [];
		}
	}

	async function handleInput() {
		await loadPaths();
		selectedIndex = 0;
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			dispatch('close');
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, paths.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === 'Enter' && paths[selectedIndex]) {
			selectPath(paths[selectedIndex]);
		}
	}

	function selectPath(path: string) {
		dispatch('select', path);
	}
</script>

<div class="overlay" onclick={() => dispatch('close')}>
	<div class="modal" onclick={(e) => e.stopPropagation()}>
		<div class="header">
			<span class="title">Jump to Directory (zoxide)</span>
		</div>
		<input
			bind:this={input}
			bind:value={query}
			oninput={handleInput}
			onkeydown={handleKeyDown}
			placeholder="Type to filter directories..."
			class="search-input"
		/>
		<div class="results">
			{#if paths.length === 0}
				<div class="no-results">
					{#if query}
						No matching directories found
					{:else}
						No directories found. Use <code>z</code> to navigate and build your database!
					{/if}
				</div>
			{:else}
				{#each paths as path, index}
					<button
						class="result-item"
						class:selected={index === selectedIndex}
						onclick={() => selectPath(path)}
						onmouseenter={() => selectedIndex = index}
					>
						<span class="icon">▸</span>
						<span class="path">{path}</span>
					</button>
				{/each}
			{/if}
		</div>
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(42, 36, 31, 0.3);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-top: 100px;
		z-index: 9999;
	}

	.modal {
		background: var(--bg-primary);
		border: 1px solid var(--border);
		border-radius: 6px;
		width: 600px;
		max-height: 500px;
		box-shadow: 0 8px 32px rgba(42, 36, 31, 0.2);
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.header {
		padding: 12px 16px;
		border-bottom: 1px solid var(--border);
		background: var(--bg-tertiary);
	}

	.title {
		font-size: 13px;
		font-weight: 500;
		color: var(--text-secondary);
	}

	.search-input {
		width: 100%;
		padding: 12px 16px;
		border-bottom: 1px solid var(--border);
		font-size: 14px;
		border-left: none;
		border-right: none;
		border-top: none;
	}

	.results {
		flex: 1;
		overflow-y: auto;
		max-height: 400px;
	}

	.no-results {
		padding: 32px 16px;
		text-align: center;
		color: var(--text-tertiary);
		font-size: 13px;
	}

	.no-results code {
		background: var(--bg-tertiary);
		padding: 2px 6px;
		border-radius: 3px;
		font-size: 12px;
	}

	.result-item {
		width: 100%;
		padding: 10px 16px;
		display: flex;
		align-items: center;
		gap: 12px;
		text-align: left;
		color: var(--text-primary);
		transition: background 0.1s;
		border: none;
		background: transparent;
		cursor: pointer;
	}

	.result-item:hover,
	.result-item.selected {
		background: var(--bg-hover);
	}

	.icon {
		font-size: 12px;
		flex-shrink: 0;
		color: var(--text-tertiary);
	}

	.path {
		font-family: monospace;
		font-size: 13px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
