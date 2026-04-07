<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { createHighlighter, type Highlighter } from 'shiki';

	let { content = $bindable(''), filePath = '' } = $props<{ content: string; filePath?: string }>();

	const dispatch = createEventDispatcher();
	let textarea: HTMLTextAreaElement;
	let highlighter: Highlighter | null = null;
	let highlightedCode = $state('');
	let useHighlighting = $state(true);

	// Detect language from file extension
	function detectLanguage(path: string): string {
		const ext = path.split('.').pop()?.toLowerCase();
		const langMap: Record<string, string> = {
			'js': 'javascript',
			'ts': 'typescript',
			'jsx': 'jsx',
			'tsx': 'tsx',
			'py': 'python',
			'rs': 'rust',
			'go': 'go',
			'java': 'java',
			'c': 'c',
			'cpp': 'cpp',
			'cs': 'csharp',
			'rb': 'ruby',
			'php': 'php',
			'html': 'html',
			'css': 'css',
			'scss': 'scss',
			'json': 'json',
			'md': 'markdown',
			'yaml': 'yaml',
			'yml': 'yaml',
			'xml': 'xml',
			'sh': 'bash',
			'bash': 'bash',
			'sql': 'sql',
			'svelte': 'svelte'
		};
		return langMap[ext || ''] || 'plaintext';
	}

	let currentLanguage = $state('plaintext');

	onMount(async () => {
		if (textarea) {
			textarea.focus();
		}

		// Initialize Shiki
		try {
			highlighter = await createHighlighter({
				themes: ['min-light'],
				langs: [
					'javascript', 'typescript', 'jsx', 'tsx', 
					'python', 'rust', 'go', 'java', 'c', 'cpp',
					'html', 'css', 'json', 'markdown', 'bash',
					'svelte'
				]
			});
			updateHighlighting();
		} catch (e) {
			console.error('Failed to load syntax highlighter:', e);
			useHighlighting = false;
		}
	});

	function updateHighlighting() {
		if (!highlighter || !useHighlighting) {
			highlightedCode = '';
			return;
		}

		currentLanguage = detectLanguage(filePath);
		
		try {
			highlightedCode = highlighter.codeToHtml(content || '', {
				lang: currentLanguage,
				theme: 'min-light'
			});
		} catch (e) {
			console.error('Highlighting error:', e);
			highlightedCode = '';
		}
	}

	// Update highlighting when content or filePath changes
	$effect(() => {
		if (content !== undefined || filePath) {
			updateHighlighting();
		}
	});

	function handleInput(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		dispatch('change', target.value);
	}

	function handleTab(e: KeyboardEvent) {
		if (e.key === 'Tab') {
			e.preventDefault();
			const target = e.target as HTMLTextAreaElement;
			const start = target.selectionStart;
			const end = target.selectionEnd;
			const newContent = content.substring(0, start) + '  ' + content.substring(end);
			content = newContent;
			dispatch('change', newContent);
			setTimeout(() => {
				target.selectionStart = target.selectionEnd = start + 2;
			}, 0);
		}
	}

	// Calculate line numbers
	let lineCount = $derived(content.split('\n').length);
	let lineNumbers = $derived(
		Array.from({ length: Math.max(lineCount, 1) }, (_, i) => i + 1)
	);
</script>

<div class="editor-container">
	<div class="line-numbers">
		{#each lineNumbers as num}
			<div class="line-number">{num}</div>
		{/each}
	</div>

	<div class="editor-wrapper">
		{#if useHighlighting && highlightedCode}
			<div class="syntax-highlight">
				{@html highlightedCode}
			</div>
		{/if}
		<textarea
			bind:this={textarea}
			bind:value={content}
			oninput={handleInput}
			onkeydown={handleTab}
			spellcheck="false"
			autocomplete="off"
			class:highlighted={useHighlighting && highlightedCode}
		/>
	</div>
</div>

<style>
	.editor-container {
		flex: 1;
		display: flex;
		overflow: auto;
		height: 100%;
		background: var(--bg-primary);
	}

	.line-numbers {
		width: 50px;
		background: var(--bg-secondary);
		border-right: 1px solid var(--border);
		padding: 16px 8px 16px 4px;
		text-align: right;
		font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
		font-size: 14px;
		line-height: 1.6;
		color: var(--text-tertiary);
		user-select: none;
		flex-shrink: 0;
	}

	.line-number {
		/* Line height matches textarea */
	}

	.editor-wrapper {
		flex: 1;
		position: relative;
		min-height: 100%;
	}

	.syntax-highlight {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		padding: 16px;
		pointer-events: none;
		font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
		font-size: 14px;
		line-height: 1.6;
		white-space: pre-wrap;
		word-wrap: break-word;
	}

	.syntax-highlight :global(*) {
		pointer-events: none !important;
	}

	.syntax-highlight :global(pre) {
		margin: 0;
		padding: 0;
		background: transparent !important;
		font-family: inherit;
		font-size: inherit;
		line-height: inherit;
	}

	.syntax-highlight :global(code) {
		font-family: inherit;
		font-size: inherit;
		line-height: inherit;
		background: transparent !important;
	}

	textarea {
		width: 100%;
		min-height: 100%;
		padding: 16px;
		background: transparent;
		color: var(--text-primary);
		font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
		font-size: 14px;
		line-height: 1.6;
		resize: none;
		border: none;
		outline: none;
		tab-size: 2;
		position: relative;
		display: block;
		box-sizing: border-box;
		overflow: hidden; /* Disable textarea's own scrollbar */
	}

	textarea.highlighted {
		color: transparent;
		caret-color: var(--text-primary);
	}

	textarea::selection {
		background: rgba(207, 192, 168, 0.4); /* Semi-transparent beige */
		color: transparent; /* Keep text transparent even when selected */
	}

	textarea.highlighted::selection {
		background: rgba(207, 192, 168, 0.4);
		color: transparent; /* Don't show the textarea text layer */
	}
</style>
