<script lang="ts">
	import { onMount } from 'svelte';
	import TitleBar from '$lib/TitleBar.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import TabBar from '$lib/TabBar.svelte';
	import Editor from '$lib/Editor.svelte';
	import Footer from '$lib/Footer.svelte';
	import QuickOpen from '$lib/QuickOpen.svelte';
	import FindInFile from '$lib/FindInFile.svelte';
	import ZoxideJump from '$lib/ZoxideJump.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

	let currentPath = $state('');
	let initialSidebarPath = $state(''); // Set by CLI args before Sidebar mounts
	let tabs = $state<Array<{ path: string; content: string; dirty: boolean; name: string }>>([]);
	let activeTabIndex = $state(0);
	let showQuickOpen = $state(false);
	let showFind = $state(false);
	let showZoxide = $state(false);
	let gitBranch = $state('');
	let sidebarComponent: any;
	let editorComponent: any;
	let changedLines = $state<Map<number, string>>(new Map());
	let sidebarVisible = $state(true);
	
	// Font size state (14px default)
	let fontSize = $state(14);
	
	// Track if components are ready
	let componentsReady = $state(false);
	
	// Load fontSize from localStorage
	onMount(() => {
		// Load saved font size
		const savedFontSize = localStorage.getItem('aperture-font-size');
		if (savedFontSize) {
			fontSize = parseInt(savedFontSize, 10);
		}
		
		// Load saved sidebar visibility
		const savedSidebarVisible = localStorage.getItem('aperture-sidebar-visible');
		if (savedSidebarVisible !== null) {
			sidebarVisible = savedSidebarVisible === 'true';
		}
		
		window.addEventListener('keydown', handleKeyDown);
		
		// Mark components as ready after a short delay
		setTimeout(() => {
			componentsReady = true;
		}, 100);
		
		// Listen for CLI arguments
		const appWindow = getCurrentWebviewWindow();
		const unlisten = appWindow.listen<{args: string[], cwd: string}>('cli-args', async (event) => {
			const { args, cwd } = event.payload;
			
			// Process first arg to determine if it's a directory
			if (args.length > 0) {
				try {
					const absolutePath: string = await invoke('resolve_path', { path: args[0], cwd });
					
					// Try to check if it's a directory
					try {
						await invoke('read_dir', { path: absolutePath });
						// It's a directory - set as initial sidebar path
						initialSidebarPath = absolutePath;
						
						// If sidebar is already mounted, reload it
						if (sidebarComponent) {
							sidebarComponent.loadDirectory(absolutePath);
						}
						return; // Don't process other args yet
					} catch {
						// It's a file, continue to normal processing
					}
				} catch (e) {
					console.error('Failed to process CLI arg:', e);
				}
			}
			
			// Wait for components to be ready for file opening
			let attempts = 0;
			while (!componentsReady && attempts < 50) {
				await new Promise(resolve => setTimeout(resolve, 100));
				attempts++;
			}
			
			if (!componentsReady) {
				console.error('Components not ready after 5 seconds');
				return;
			}
			
			for (const arg of args) {
				await openFileFromCLI(arg, cwd);
			}
		});
		
		return () => {
			window.removeEventListener('keydown', handleKeyDown);
			unlisten.then(fn => fn());
		};
	});

	async function openFileFromCLI(pathArg: string, cwd: string) {
		try {
			// Resolve to absolute path using the CWD from when the command was run
			const absolutePath: string = await invoke('resolve_path', { path: pathArg, cwd });
			
			// Check if it's a directory
			const homeDir: string = await invoke('get_home_dir');
			try {
				const files = await invoke('read_dir', { path: absolutePath });
				// It's a directory - navigate to it
				currentPath = absolutePath;
				if (sidebarComponent) {
					sidebarComponent.loadDirectory(absolutePath);
				}
			} catch {
				// It's a file - try to open it
				try {
					const content: string = await invoke('read_file', { path: absolutePath });
					const name = absolutePath.split('/').pop() || 'untitled';
					
				// Check if already open
				const existingIndex = tabs.findIndex(t => t.path === absolutePath);
				if (existingIndex !== -1) {
					activeTabIndex = existingIndex;
				} else {
					tabs = [...tabs, { path: absolutePath, content, dirty: false, name }];
					activeTabIndex = tabs.length - 1;
				}
				fetchChangedLines(absolutePath);
				} catch (e) {
					console.error('Failed to open file from CLI:', pathArg, e);
				}
			}
		} catch (e) {
			console.error('Failed to resolve path:', pathArg, e);
		}
	}

	function handleKeyDown(e: KeyboardEvent) {
		// Ctrl+K for quick open
		if (e.ctrlKey && e.key === 'k') {
			e.preventDefault();
			showQuickOpen = !showQuickOpen;
		}
		// Ctrl+B for sidebar toggle
		if (e.ctrlKey && e.key === 'b') {
			e.preventDefault();
			sidebarVisible = !sidebarVisible;
			localStorage.setItem('aperture-sidebar-visible', sidebarVisible.toString());
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
		// Ctrl+Z for undo and Ctrl+Shift+Z for redo - handled by Editor component
		// (custom undo/redo stack with 500ms debounce)
		// Ctrl+A for select all - native browser support
		// (no preventDefault needed - let it work naturally)
		
		// Font size controls
		if (e.ctrlKey && (e.key === '=' || e.key === '+')) {
			e.preventDefault();
			increaseFontSize();
		}
		if (e.ctrlKey && (e.key === '-' || e.key === '_')) {
			e.preventDefault();
			decreaseFontSize();
		}
		if (e.ctrlKey && e.key === '0') {
			e.preventDefault();
			resetFontSize();
		}
		
		// New file
		if (e.ctrlKey && e.key === 'n') {
			e.preventDefault();
			createNewFile();
		}
	}

	function increaseFontSize() {
		if (fontSize < 32) {
			fontSize += 2;
			saveFontSize();
		}
	}

	function decreaseFontSize() {
		if (fontSize > 8) {
			fontSize -= 2;
			saveFontSize();
		}
	}

	function resetFontSize() {
		fontSize = 14;
		saveFontSize();
	}

	function saveFontSize() {
		localStorage.setItem('aperture-font-size', fontSize.toString());
	}

	let untitledCounter = 1;

	function createNewFile() {
		const name = `Untitled-${untitledCounter}`;
		const path = `untitled-${untitledCounter}`;
		untitledCounter++;
		
		tabs = [...tabs, { path, content: '', dirty: false, name }];
		activeTabIndex = tabs.length - 1;
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
		fetchChangedLines(path);
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
				fetchChangedLines(tab.path);
			} catch (e) {
				console.error('Failed to save:', e);
			}
		}
	}

	function selectTab(event: CustomEvent) {
		activeTabIndex = event.detail;
		if (tabs[activeTabIndex]) {
			fetchChangedLines(tabs[activeTabIndex].path);
		}
	}

	async function fetchChangedLines(filePath: string) {
		if (!filePath || filePath.startsWith('untitled-')) {
			changedLines = new Map();
			return;
		}
		try {
			const dir = filePath.substring(0, filePath.lastIndexOf('/'));
			const result: Array<{ line: number; status: string }> = await invoke('git_line_diff', { path: dir, filePath });
			const map = new Map<number, string>();
			for (const item of result) {
				map.set(item.line, item.status);
			}
			changedLines = map;
		} catch {
			changedLines = new Map();
		}
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

	function handleScrollToMatch(event: CustomEvent) {
		const { start, end } = event.detail;
		if (editorComponent) {
			editorComponent.scrollToPosition(start, end);
		}
	}
</script>

<TitleBar />

<div class="app">
	{#if sidebarVisible}
		<Sidebar 
			bind:this={sidebarComponent}
			initialPath={initialSidebarPath}
			on:openFile={openFile}
			on:pathChange={setCurrentPath}
			on:branchChange={setBranch}
		/>
	{/if}
	
	<div class="main">
		<TabBar 
			bind:tabs={tabs}
			bind:activeTabIndex={activeTabIndex}
			on:selectTab={selectTab}
			on:closeTab={(e) => closeTab(e.detail)}
			on:newFile={createNewFile}
		/>
		
		{#if tabs.length > 0}
			<Editor 
				bind:this={editorComponent}
				bind:content={tabs[activeTabIndex].content}
				filePath={tabs[activeTabIndex].path}
				fontSize={fontSize}
				changedLines={changedLines}
				on:change={updateTabContent}
			/>
		{:else}
			<div class="empty-state">
				<div class="empty-content">
					<h1>aperture</h1>
					<p>Open a file from the sidebar or use keyboard shortcuts:</p>
					<div class="shortcuts">
						<div><kbd>Ctrl+N</kbd> New file</div>
						<div><kbd>Ctrl+K</kbd> Quick open</div>
						<div><kbd>Ctrl+O</kbd> Jump to directory (zoxide)</div>
						<div><kbd>Ctrl+F</kbd> Find in file</div>
						<div><kbd>Ctrl+S</kbd> Save</div>
						<div><kbd>Ctrl++/-/0</kbd> Font size</div>
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
	<FindInFile 
		content={tabs[activeTabIndex].content}
		on:close={() => showFind = false}
		on:scrollToMatch={handleScrollToMatch}
	/>
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
		margin: 0;
		padding: 0;
		overflow: hidden;
	}

	.app {
		display: flex;
		flex-direction: row;
		width: 100%;
		height: calc(100vh - 32px);
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
