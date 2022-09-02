<script lang="ts">
  let count: number = 0
  const increment = () => {
    count += 1
  }
</script>

<button on:click={increment} class="px-6 py-2 rounded-md bg-orange-200">
  count is {count}
</button>
