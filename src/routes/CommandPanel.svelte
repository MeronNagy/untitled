<script lang="ts">
    let { title, children } = $props();
</script>

<div class="command-panel">
    <h2>{title}</h2>
    {@render children?.()}
</div>

<style>
    .command-panel {
        margin-bottom: 2rem;
        padding-bottom: 1rem;
        padding-left: 1rem;
        padding-right: 1rem;
        background-color: rgba(31, 31, 31, 0.8);
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }
</style>
