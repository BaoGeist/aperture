<script lang="ts">
	import { onMount } from 'svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import TabBar from '$lib/TabBar.svelte';
	import Editor from '$lib/Editor.svelte';
	import Footer from '$lib/Footer.svelte';
	import QuickOpen from '$lib/QuickOpen.svelte';
	import FindInFile from '$lib/FindInFile.svelte';
	import ZoxideJump from '$lib/ZoxideJump.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let currentPath = $state('');
	let tabs = $state<Array<{ path: string; content: string; dirty: boolean; name: string }>>([]);
	let activeTabIndex = $state(0);
	let showQuickOpen = $state(false);
	let showFind = $state(false);
	let showZoxide = $state(false);
	let gitBranch = $state('');
	let sidebarComponent: any;

	onMount(() => {
		window.addEventListener('keydown', handleKeyDown);
		return () => window.removeEventListener('keydown', handleKeyDown);
	});

	function handleKeyDown(e: KeyboardEvent) {
		// Ctrl+K for quick open
		if (e.ctrlKey && e.key === 'k') {
			e.preventDefault();
			showQuickOpen = !showQuickOpen;
		}
		// Ctrl+F for find
		if (e.ctrlKey && e.key === 'f') {
			e.preventDefault();
			showFind = !showFind;
		}
		// Ctrl+O for zoxide jump
		if (e.ctrlKey && e.key === 'o') {
			e.preventDefault();
			showZoxide = !showZoxide;
		}
		// Ctrl+S for save
		if (e.ctrlKey && e.key === 's') {
			e.preventDefault();
			saveCurrentTab();
		}
		// Ctrl+W to close tab
		if (e.ctrlKey && e.key === 'w') {
			e.preventDefault();
			if (tabs.length > 0) {
				closeTab(activeTabIndex);
			}
		}
		// Ctrl+A for select all - let browser handle it naturally
		// (no preventDefault, just allow it to work)
	}

	function openFile(event: CustomEvent) {
		const { path, content } = event.detail;
		const existingIndex = tabs.findIndex(t => t.path === path);
		if (existingIndex !== -1) {
			activeTabIndex = existingIndex;
		} else {
			const name = path.split('/').pop() || 'untitled';
			tabs = [...tabs, { path, content, dirty: false, name }];
			activeTabIndex = tabs.length - 1;
		}
		showQuickOpen = false;
	}

	function closeTab(index: number) {
		tabs = tabs.filter((_, i) => i !== index);
		if (activeTabIndex >= tabs.length) {
			activeTabIndex = Math.max(0, tabs.length - 1);
		}
	}

	function updateTabContent(event: CustomEvent) {
		if (tabs[activeTabIndex]) {
			tabs[activeTabIndex].content = event.detail;
			tabs[activeTabIndex].dirty = true;
		}
	}

	async function saveCurrentTab() {
		if (tabs[activeTabIndex]) {
			const tab = tabs[activeTabIndex];
			try {
				await invoke('write_file', { path: tab.path, content: tab.content });
				tabs[activeTabIndex].dirty = false;
			} catch (e) {
				console.error('Failed to save:', e);
			}
		}
	}

	function selectTab(event: CustomEvent) {
		activeTabIndex = event.detail;
	}

	function setCurrentPath(event: CustomEvent) {
		currentPath = event.detail;
	}

	function setBranch(event: CustomEvent) {
		gitBranch = event.detail;
	}

	function handleZoxideSelect(event: CustomEvent) {
		const path = event.detail;
		currentPath = path;
		showZoxide = false;
		// Trigger sidebar to reload with new path
		if (sidebarComponent) {
			sidebarComponent.loadDirectory?.(path);
		}
	}
</script>

<div class="app">
	<Sidebar 
		bind:this={sidebarComponent}
		on:openFile={openFile}
		on:pathChange={setCurrentPath}
		on:branchChange={setBranch}
	/>
	
	<div class="main">
		<TabBar 
			bind:tabs={tabs}
			bind:activeTabIndex={activeTabIndex}
			on:selectTab={selectTab}
			on:closeTab={(e) => closeTab(e.detail)}
		/>
		
		{#if tabs.length > 0}
			<Editor 
				bind:content={tabs[activeTabIndex].content}
				filePath={tabs[activeTabIndex].path}
				on:change={updateTabContent}
			/>
		{:else}
			<div class="empty-state">
				<div class="empty-content">
					<h1>aperture</h1>
					<p>Open a file from the sidebar or use keyboard shortcuts:</p>
					<div class="shortcuts">
						<div><kbd>Ctrl+K</kbd> Quick open</div>
						<div><kbd>Ctrl+O</kbd> Jump to directory (zoxide)</div>
						<div><kbd>Ctrl+F</kbd> Find in file</div>
						<div><kbd>Ctrl+S</kbd> Save</div>
						<div><kbd>Ctrl+A</kbd> Select all</div>
					</div>
				</div>
			</div>
		{/if}

		<Footer gitBranch={gitBranch} tabCount={tabs.length} />
	</div>
</div>

<!-- Modals rendered outside app container for proper overlay -->
{#if showQuickOpen}
	<QuickOpen 
		currentPath={currentPath}
		on:close={() => showQuickOpen = false}
		on:select={openFile}
	/>
{/if}

{#if showFind && tabs.length > 0}
	<FindInFile on:close={() => showFind = false} />
{/if}

{#if showZoxide}
	<ZoxideJump 
		on:close={() => showZoxide = false}
		on:select={handleZoxideSelect}
	/>
{/if}

<style>
	:global(body) {
		position: relative;
	}

	.app {
		display: flex;
		flex-direction: row;
		width: 100%;
		height: 100vh;
		overflow: hidden;
	}

	.main {
		flex: 1;
		display: flex;
		flex-direction: column;
		height: 100%;
		min-width: 0;
		overflow: hidden;
	}

	.empty-state {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-primary);
	}

	.empty-content {
		text-align: center;
		color: var(--text-secondary);
	}

	.empty-content h1 {
		font-size: 48px;
		font-weight: 300;
		letter-spacing: -0.5px;
		color: var(--beige-400);
		margin-bottom: 16px;
	}

	.empty-content p {
		font-size: 14px;
		color: var(--text-tertiary);
		margin-bottom: 16px;
	}

	.shortcuts {
		display: inline-block;
		text-align: left;
		font-size: 13px;
		color: var(--text-secondary);
	}

	.shortcuts div {
		margin: 6px 0;
	}

	kbd {
		background: var(--bg-tertiary);
		border: 1px solid var(--border);
		border-radius: 3px;
		padding: 2px 6px;
		font-family: inherit;
		font-size: 12px;
		color: var(--text-primary);
		margin-right: 8px;
		display: inline-block;
		min-width: 60px;
	}
</style>
