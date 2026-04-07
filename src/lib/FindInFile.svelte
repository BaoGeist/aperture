<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

	let { content = '' } = $props<{ content: string }>();

	const dispatch = createEventDispatcher();

	let query = $state('');
	let input: HTMLInputElement;
	let caseSensitive = $state(false);
	let currentMatchIndex = $state(0);
	let matches = $state<{ start: number; end: number; line: number }[]>([]);

	onMount(() => {
		input?.focus();
	});

	// Find all matches when query changes
	$effect(() => {
		if (query && content) {
			findMatches();
		} else {
			matches = [];
			currentMatchIndex = 0;
		}
	});

	function findMatches() {
		const searchText = caseSensitive ? query : query.toLowerCase();
		const searchContent = caseSensitive ? content : content.toLowerCase();
		const foundMatches: { start: number; end: number; line: number }[] = [];
		
		let index = 0;
		while (index !== -1) {
			index = searchContent.indexOf(searchText, index);
			if (index !== -1) {
				// Calculate line number
				const lineNumber = content.substring(0, index).split('\n').length;
				foundMatches.push({
					start: index,
					end: index + query.length,
					line: lineNumber
				});
				index += 1; // Move past this match
			}
		}
		
		matches = foundMatches;
		currentMatchIndex = foundMatches.length > 0 ? 0 : -1;
		
		if (foundMatches.length > 0) {
			scrollToMatch(0);
		}
	}

	function scrollToMatch(index: number) {
		if (matches.length === 0) return;
		
		const match = matches[index];
		dispatch('scrollToMatch', {
			start: match.start,
			end: match.end,
			line: match.line
		});
	}

	function nextMatch() {
		if (matches.length === 0) return;
		
		currentMatchIndex = (currentMatchIndex + 1) % matches.length;
		scrollToMatch(currentMatchIndex);
	}

	function previousMatch() {
		if (matches.length === 0) return;
		
		currentMatchIndex = currentMatchIndex - 1;
		if (currentMatchIndex < 0) {
			currentMatchIndex = matches.length - 1;
		}
		scrollToMatch(currentMatchIndex);
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			dispatch('close');
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (e.shiftKey) {
				previousMatch();
			} else {
				nextMatch();
			}
		}
	}

	function toggleCaseSensitive() {
		caseSensitive = !caseSensitive;
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
	
	{#if query}
		<div class="match-count">
			{#if matches.length > 0}
				{currentMatchIndex + 1} of {matches.length}
			{:else}
				No matches
			{/if}
		</div>
	{/if}
	
	<button 
		class="case-btn" 
		class:active={caseSensitive}
		onclick={toggleCaseSensitive}
		title="Case sensitive"
	>
		Aa
	</button>
	
	<div class="nav-buttons">
		<button 
			class="nav-btn" 
			onclick={previousMatch}
			disabled={matches.length === 0}
			title="Previous match (Shift+Enter)"
		>
			↑
		</button>
		<button 
			class="nav-btn" 
			onclick={nextMatch}
			disabled={matches.length === 0}
			title="Next match (Enter)"
		>
			↓
		</button>
	</div>
	
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
		gap: 6px;
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

	.match-count {
		font-size: 11px;
		color: var(--text-tertiary);
		white-space: nowrap;
		min-width: 60px;
		text-align: center;
	}

	.case-btn {
		width: 28px;
		height: 24px;
		border-radius: 3px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
		font-weight: 500;
		color: var(--text-secondary);
		transition: all 0.15s;
		border: 1px solid transparent;
		background: transparent;
		padding: 0;
		cursor: pointer;
	}

	.case-btn:hover {
		background: var(--bg-hover);
		border-color: var(--border);
	}

	.case-btn.active {
		background: var(--beige-400);
		color: var(--text-primary);
		border-color: var(--beige-500);
	}

	.nav-buttons {
		display: flex;
		gap: 2px;
	}

	.nav-btn {
		width: 24px;
		height: 24px;
		border-radius: 3px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 14px;
		color: var(--text-secondary);
		transition: all 0.15s;
		border: none;
		background: transparent;
		padding: 0;
		cursor: pointer;
	}

	.nav-btn:hover:not(:disabled) {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.nav-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.close-btn {
		width: 24px;
		height: 24px;
		border-radius: 3px;
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
		margin-left: 2px;
	}

	.close-btn:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}
</style>
