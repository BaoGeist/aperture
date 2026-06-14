<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { currentPath = '' } = $props<{ currentPath: string }>();

	const dispatch = createEventDispatcher();

	let query = $state('');
	let files = $state<string[]>([]);
	let filteredFiles = $state<string[]>([]);
	let selectedIndex = $state(0);
	let input: HTMLInputElement;
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		input?.focus();
		await loadFiles();
	});

	async function loadFiles() {
		try {
			loading = true;
			error = '';
			const searchPath = currentPath || await invoke('get_home_dir');
			console.log('Quick open searching in:', searchPath);
			files = await invoke('quick_open', { path: searchPath });
			console.log('Found files:', files.length);
			filteredFiles = files.slice(0, 50);
			loading = false;
		} catch (e) {
			console.error('Quick open failed:', e);
			error = String(e);
			loading = false;
		}
	}

	function handleInput() {
		if (!query) {
			filteredFiles = files.slice(0, 50);
		} else {
			const lowerQuery = query.toLowerCase();
			filteredFiles = files
				.filter(f => f.toLowerCase().includes(lowerQuery))
				.slice(0, 50);
		}
		selectedIndex = 0;
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			dispatch('close');
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, filteredFiles.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === 'Enter' && filteredFiles[selectedIndex]) {
			selectFile(filteredFiles[selectedIndex]);
		}
	}

	async function selectFile(path: string) {
		try {
			const searchPath = currentPath || await invoke('get_home_dir');
			const fullPath = `${searchPath}/${path}`;
			console.log('Opening file:', fullPath);
			const content = await invoke('read_file', { path: fullPath });
			dispatch('select', { path: fullPath, content });
		} catch (e) {
			console.error('Failed to open file:', e);
			error = `Failed to open: ${e}`;
		}
	}
</script>

<div class="overlay" onclick={() => dispatch('close')}>
	<div class="modal" onclick={(e) => e.stopPropagation()}>
		<div class="header">
			<span class="title">Quick Open</span>
			{#if !loading && files.length > 0}
				<span class="count">{filteredFiles.length} / {files.length} files</span>
			{/if}
		</div>
		<input
			bind:this={input}
			bind:value={query}
			oninput={handleInput}
			onkeydown={handleKeyDown}
			placeholder="Type to search files..."
			class="search-input"
		/>
		<div class="results">
			{#if loading}
				<div class="empty-state">Loading files...</div>
			{:else if error}
				<div class="empty-state error">
					Error: {error}
					<br />
					<small>Make sure 'fd' is installed: sudo apt install fd-find</small>
				</div>
			{:else if filteredFiles.length === 0}
				<div class="empty-state">
					{#if query}
						No files match "{query}"
					{:else}
						No files found in current directory
					{/if}
				</div>
			{:else}
				{#each filteredFiles as file, index}
					<button
						class="result-item"
						class:selected={index === selectedIndex}
						onclick={() => selectFile(file)}
						onmouseenter={() => selectedIndex = index}
					>
						{file}
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
		max-height: 400px;
		box-shadow: 0 8px 32px rgba(42, 36, 31, 0.2);
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.header {
		padding: 8px 16px;
		border-bottom: 1px solid var(--border);
		background: var(--bg-tertiary);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.title {
		font-size: 12px;
		font-weight: 500;
		color: var(--text-secondary);
	}

	.count {
		font-size: 11px;
		color: var(--text-tertiary);
	}

	.search-input {
		width: 100%;
		padding: 12px 16px;
		border-bottom: 1px solid var(--border);
		font-size: 14px;
		border-left: none;
		border-right: none;
		border-top: none;
		outline: none;
		background: transparent;
	}

	.search-input:focus {
		outline: none;
	}

	.results {
		flex: 1;
		overflow-y: auto;
		max-height: 320px;
	}

	.empty-state {
		padding: 32px 16px;
		text-align: center;
		color: var(--text-tertiary);
		font-size: 13px;
	}

	.empty-state.error {
		color: var(--beige-800);
	}

	.empty-state small {
		font-size: 11px;
		color: var(--text-tertiary);
	}

	.result-item {
		width: 100%;
		padding: 8px 16px;
		text-align: left;
		font-family: monospace;
		font-size: 13px;
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
</style>
