<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	let { tabs = $bindable([]), activeTabIndex = $bindable(0) } = $props<{
		tabs: Array<{ name: string; dirty: boolean }>;
		activeTabIndex: number;
	}>();

	const dispatch = createEventDispatcher();

	function selectTab(index: number) {
		dispatch('selectTab', index);
	}

	function closeTab(index: number, e: MouseEvent) {
		e.stopPropagation();
		dispatch('closeTab', index);
	}
</script>

<div class="tab-bar">
	{#each tabs as tab, index}
		<div
			class="tab"
			class:active={index === activeTabIndex}
			onclick={() => selectTab(index)}
		>
			<span class="tab-name">
				{#if tab.dirty}
					<span class="dirty-indicator">●</span>
				{/if}
				{tab.name}
			</span>
			<button class="tab-close" onclick={(e) => closeTab(index, e)}>×</button>
		</div>
	{/each}
</div>

<style>
	.tab-bar {
		display: flex;
		background: var(--bg-secondary);
		border-bottom: 1px solid var(--border);
		overflow-x: auto;
		height: 36px;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0 12px;
		border-right: 1px solid var(--border);
		min-width: 120px;
		max-width: 200px;
		background: var(--bg-tertiary);
		color: var(--text-secondary);
		transition: all 0.15s;
		border-top: none;
		border-left: none;
		border-bottom: 2px solid transparent;
	}

	.tab:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.tab.active {
		background: var(--bg-primary);
		color: var(--text-primary);
		border-bottom: 2px solid var(--beige-600);
	}

	.tab-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 12px;
	}

	.dirty-indicator {
		color: var(--beige-700);
		font-size: 16px;
		line-height: 1;
	}

	.tab-close {
		width: 16px;
		height: 16px;
		border-radius: 2px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 18px;
		line-height: 1;
		opacity: 0.6;
		transition: all 0.15s;
		padding: 0;
		border: none;
		background: transparent;
	}

	.tab-close:hover {
		background: var(--beige-400);
		opacity: 1;
	}
</style>
