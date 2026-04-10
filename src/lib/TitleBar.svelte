<script lang="ts">
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

	const appWindow = getCurrentWebviewWindow();

	async function minimize() {
		await appWindow.minimize();
	}

	async function toggleMaximize() {
		await appWindow.toggleMaximize();
	}

	async function close() {
		await appWindow.close();
	}
</script>

<div class="titlebar" data-tauri-drag-region>
	<div class="titlebar-title" data-tauri-drag-region>
		aperture
	</div>
	
	<div class="titlebar-controls">
		<button class="titlebar-button" onclick={minimize} title="Minimize">
			<svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
				<rect x="2" y="5.5" width="8" height="1" fill="currentColor"/>
			</svg>
		</button>
		
		<button class="titlebar-button" onclick={toggleMaximize} title="Maximize">
			<svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
				<rect x="2" y="2" width="8" height="8" stroke="currentColor" stroke-width="1" fill="none"/>
			</svg>
		</button>
		
		<button class="titlebar-button close" onclick={close} title="Close">
			<svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path d="M2 2L10 10M10 2L2 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
			</svg>
		</button>
	</div>
</div>

<style>
	.titlebar {
		height: 32px;
		background: var(--bg-secondary);
		border-bottom: 1px solid var(--border);
		display: flex;
		align-items: center;
		justify-content: space-between;
		user-select: none;
		flex-shrink: 0;
	}

	.titlebar-title {
		flex: 1;
		padding: 0 12px;
		font-size: 12px;
		color: var(--text-secondary);
		font-weight: 500;
	}

	.titlebar-controls {
		display: flex;
		height: 100%;
	}

	.titlebar-button {
		width: 46px;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: var(--text-secondary);
		transition: all 0.15s;
		cursor: pointer;
		padding: 0;
	}

	.titlebar-button:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.titlebar-button.close:hover {
		background: #e81123;
		color: white;
	}
</style>
