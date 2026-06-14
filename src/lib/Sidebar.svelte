<script lang="ts">
	import { onMount, createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { initialPath = '' } = $props<{ initialPath?: string }>();

	const dispatch = createEventDispatcher();

	interface FileEntry {
		name: string;
		path: string;
		is_dir: boolean;
		is_git_repo: boolean;
		children?: FileEntry[];
		expanded?: boolean;
	}

	interface GitStatus {
		path: string;
		status: string;
	}

	let currentPath = $state('');
	let rootFiles = $state<FileEntry[]>([]);
	let gitStatuses = $state<Map<string, string>>(new Map());
	let sidebarWidth = $state(240);
	let isResizing = $state(false);

	onMount(async () => {
		const savedWidth = localStorage.getItem('aperture-sidebar-width');
		if (savedWidth) {
			sidebarWidth = parseInt(savedWidth, 10);
		}
		const startPath = initialPath || await getHomePath();
		currentPath = startPath;
		await loadDirectory(startPath);
		await checkGitStatus(startPath);
		checkGitBranch(startPath);
	});

	async function getHomePath(): Promise<string> {
		try {
			return await invoke('get_home_dir');
		} catch {
			return '/home';
		}
	}

	export async function loadDirectory(path: string) {
		try {
			const files: FileEntry[] = await invoke('read_dir', { path });
			rootFiles = files.map(f => ({ ...f, expanded: false, children: [] }));
			currentPath = path;
			dispatch('pathChange', path);
			await checkGitStatus(path);
		} catch (e) {
			console.error('Failed to read directory:', e);
		}
	}

	async function checkGitBranch(path: string) {
		try {
			const branch = await invoke('get_git_branch', { path });
			dispatch('branchChange', branch);
		} catch {
			dispatch('branchChange', '');
		}
	}

	async function checkGitStatus(path: string) {
		try {
			const statuses: GitStatus[] = await invoke('git_status', { path });
			const statusMap = new Map<string, string>();
			statuses.forEach(s => {
				// Handle both absolute and relative paths
				const fullPath = s.path.startsWith('/') ? s.path : `${path}/${s.path}`;
				statusMap.set(fullPath, s.status);
				statusMap.set(s.path, s.status); // Also store relative path
			});
			gitStatuses = statusMap;
		} catch (e) {
			gitStatuses = new Map();
		}
	}

	async function toggleDirectory(file: FileEntry) {
		if (!file.is_dir) return;

		file.expanded = !file.expanded;

		if (file.expanded && (!file.children || file.children.length === 0)) {
			try {
				const children: FileEntry[] = await invoke('read_dir', { path: file.path });
				file.children = children.map(f => ({ ...f, expanded: false, children: [] }));
			} catch (e) {
				console.error('Failed to read subdirectory:', e);
			}
		}

		// Force reactivity
		rootFiles = [...rootFiles];
	}

	async function navigateIntoFolder(file: FileEntry) {
		if (!file.is_dir) return;
		
		await loadDirectory(file.path);
		await checkGitStatus(file.path);
		checkGitBranch(file.path);
	}

	async function handleFileClick(file: FileEntry) {
		if (file.is_dir) {
			// Single click on folder: toggle accordion
			await toggleDirectory(file);
		} else {
			// Single click on file: open it
			try {
				const content = await invoke('read_file', { path: file.path });
				dispatch('openFile', { path: file.path, content });
			} catch (e) {
				console.error('Failed to read file:', e);
			}
		}
	}

	async function handleFileDoubleClick(file: FileEntry) {
		if (file.is_dir) {
			// Double click on folder: navigate into it
			await navigateIntoFolder(file);
		}
	}

	async function navigateUp() {
		const parts = currentPath.split('/').filter(p => p);
		if (parts.length > 1) {
			const parentPath = '/' + parts.slice(0, -1).join('/');
			await loadDirectory(parentPath);
			await checkGitStatus(parentPath);
			checkGitBranch(parentPath);
		}
	}

	function startResize(e: MouseEvent) {
		e.preventDefault();
		isResizing = true;
		const startX = e.clientX;
		const startWidth = sidebarWidth;

		function onMouseMove(e: MouseEvent) {
			const delta = e.clientX - startX;
			sidebarWidth = Math.min(500, Math.max(150, startWidth + delta));
		}

		function onMouseUp() {
			isResizing = false;
			localStorage.setItem('aperture-sidebar-width', sidebarWidth.toString());
			document.removeEventListener('mousemove', onMouseMove);
			document.removeEventListener('mouseup', onMouseUp);
		}

		document.addEventListener('mousemove', onMouseMove);
		document.addEventListener('mouseup', onMouseUp);
	}

	function getGitStatusColor(filePath: string): string {
		const status = gitStatuses.get(filePath);
		if (!status) return '';
		
		switch (status) {
			case 'modified': return 'var(--git-modified)';
			case 'untracked': return 'var(--git-added)';
			case 'added': return 'var(--git-added)';
			case 'deleted': return 'var(--git-deleted)';
			default: return '';
		}
	}

	function getGitStatusIcon(filePath: string): string {
		const status = gitStatuses.get(filePath);
		if (!status) return '';
		
		switch (status) {
			case 'modified': return 'M';
			case 'untracked': return 'U';
			case 'added': return 'A';
			case 'deleted': return 'D';
			default: return '';
		}
	}

	const STATUS_PRIORITY: Record<string, number> = {
		'deleted': 0,
		'modified': 1,
		'added': 2,
		'untracked': 3
	};

	function getFolderGitStatus(folderPath: string): { color: string; icon: string; count: number } | null {
		let worstStatus = '';
		let worstPriority = Infinity;
		let count = 0;
		const prefix = folderPath.endsWith('/') ? folderPath : folderPath + '/';

		for (const [filePath, status] of gitStatuses) {
			if (filePath.startsWith(prefix) || filePath === folderPath) {
				count++;
				const priority = STATUS_PRIORITY[status] ?? 99;
				if (priority < worstPriority) {
					worstPriority = priority;
					worstStatus = status;
				}
			}
		}

		if (count === 0) return null;

		let color = '';
		let icon = '';
		switch (worstStatus) {
			case 'deleted':
				color = 'var(--git-deleted)';
				icon = 'D';
				break;
			case 'modified':
				color = 'var(--git-modified)';
				icon = 'M';
				break;
			case 'added':
				color = 'var(--git-added)';
				icon = 'A';
				break;
			case 'untracked':
				color = 'var(--git-added)';
				icon = 'U';
				break;
		}

		return { color, icon, count };
	}

	function renderFileTree(files: FileEntry[], depth: number = 0): any {
		return files.map(file => {
			let statusColor = '';
			let statusIcon = '';
			let folderStatus: { color: string; icon: string; count: number } | null = null;

			if (file.is_dir) {
				folderStatus = getFolderGitStatus(file.path);
				if (folderStatus) {
					statusColor = folderStatus.color;
					statusIcon = folderStatus.icon;
				}
			} else {
				statusColor = getGitStatusColor(file.path);
				statusIcon = getGitStatusIcon(file.path);
			}

			return {
				file,
				depth,
				statusColor,
				statusIcon,
				folderStatus,
				children: file.expanded && file.children ? renderFileTree(file.children, depth + 1) : []
			};
		});
	}

	$effect(() => {
		// Re-render when rootFiles changes
		fileTreeItems = flattenTree(renderFileTree(rootFiles));
	});

	let fileTreeItems = $state<any[]>([]);

	function flattenTree(items: any[]): any[] {
		const result: any[] = [];
		for (const item of items) {
			result.push(item);
			if (item.children && item.children.length > 0) {
				result.push(...flattenTree(item.children));
			}
		}
		return result;
	}
</script>

<aside class="sidebar" style="width: {sidebarWidth}px">
	<div class="sidebar-header">
		<button class="nav-up" onclick={navigateUp} title="Go up one level">
			<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path d="M8 3L3 8H6V13H10V8H13L8 3Z" fill="currentColor"/>
			</svg>
		</button>
		<div class="current-path" title={currentPath}>
			{currentPath.split('/').pop() || '/'}
		</div>
	</div>

	<div class="file-list">
	{#each fileTreeItems as { file, depth, statusColor, statusIcon, folderStatus }}
		<button
			class="file-item"
			class:is-dir={file.is_dir}
			style="padding-left: {8 + depth * 16}px"
			onclick={() => handleFileClick(file)}
			ondblclick={() => handleFileDoubleClick(file)}
		>
			<span class="file-icon">
				{#if file.is_dir}
					<svg 
						width="12" 
						height="12" 
						viewBox="0 0 16 16" 
						fill="none" 
						xmlns="http://www.w3.org/2000/svg"
						class="chevron"
						class:expanded={file.expanded}
					>
						<path d="M6 4L10 8L6 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
					</svg>
				{:else}
					<svg width="12" height="12" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
						<circle cx="8" cy="8" r="2" fill="currentColor"/>
					</svg>
				{/if}
			</span>
			<span class="file-name" style={statusColor ? `color: ${statusColor}` : ''}>{file.name}</span>
			{#if statusIcon}
				<span class="git-status" style="color: {statusColor}" title={folderStatus ? `${folderStatus.count} changed` : statusIcon}>
					{#if folderStatus && folderStatus.count > 1}
						{folderStatus.count}
					{:else}
						{statusIcon}
					{/if}
				</span>
			{/if}
		</button>
	{/each}
	</div>
	<div class="resize-handle" class:active={isResizing} onmousedown={startResize}></div>
</aside>

<style>
	.sidebar {
		background: var(--bg-secondary);
		border-right: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		position: relative;
		flex-shrink: 0;
	}

	.sidebar-header {
		padding: 8px;
		border-bottom: 1px solid var(--border);
		display: flex;
		align-items: center;
		gap: 8px;
		background: var(--bg-tertiary);
	}

	.nav-up {
		width: 24px;
		height: 24px;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-secondary);
		transition: all 0.15s;
		border: 1px solid transparent;
		background: transparent;
		padding: 0;
	}

	.nav-up:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.current-path {
		flex: 1;
		font-size: 12px;
		color: var(--text-secondary);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-list {
		flex: 1;
		min-height: 0;
		overflow-y: auto;
		padding: 4px;
	}

	.file-item {
		width: 100%;
		padding: 4px 8px;
		display: flex;
		align-items: center;
		gap: 6px;
		border-radius: 4px;
		text-align: left;
		color: var(--text-primary);
		transition: background 0.15s;
		border: none;
		background: transparent;
	}

	.file-item:hover {
		background: var(--bg-hover);
	}

	.file-item.is-dir {
		font-weight: 500;
	}

	.file-icon {
		color: var(--text-tertiary);
		width: 12px;
		height: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.chevron {
		transition: transform 0.2s ease;
		transform: rotate(0deg);
	}

	.chevron.expanded {
		transform: rotate(90deg);
	}

	.file-name {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		flex: 1;
	}

	.git-status {
		font-size: 10px;
		font-weight: 600;
		margin-left: auto;
		flex-shrink: 0;
	}

	.resize-handle {
		position: absolute;
		top: 0;
		right: 0;
		width: 4px;
		height: 100%;
		cursor: col-resize;
		z-index: 10;
	}

	.resize-handle:hover,
	.resize-handle.active {
		background: var(--beige-400);
	}
</style>
