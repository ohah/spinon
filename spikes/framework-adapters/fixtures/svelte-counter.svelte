<script>
  let count = $state(0);
</script>

<main>
  <button aria-label="count" onclick={() => (count += 1)}>count: {count}</button>
</main>
