<template>
  <div>
    <p>Server status {{ status }}</p>
    <button type="button" @click="status='Normal'">Change</button>
  </div>
</template>

<script>
  export default {
    data: function() {
      return {
        status: 'critico'
      }
    }
  }
</script>

<style>

</style>
