<script lang="ts">
	let { gitBranch = '', tabCount = 0 } = $props<{ gitBranch: string; tabCount: number }>();
</script>

<footer class="footer">
	<div class="footer-section">
		{#if gitBranch}
			<span class="git-branch">git: {gitBranch}</span>
		{/if}
	</div>
	<div class="footer-section right">
		{#if tabCount > 0}
			<span>{tabCount} {tabCount === 1 ? 'file' : 'files'} open</span>
		{/if}
	</div>
</footer>

<style>
	.footer {
		height: 24px;
		background: var(--bg-tertiary);
		border-top: 1px solid var(--border);
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 12px;
		font-size: 11px;
		color: var(--text-tertiary);
	}

	.footer-section {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.footer-section.right {
		margin-left: auto;
	}

	.git-branch {
		color: var(--text-secondary);
	}
</style>
