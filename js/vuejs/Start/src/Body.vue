<template>
        <div class="row">
            <my-list></my-list>
            <my-details></my-details>
        </div>
</template>

<script>
  import Details from './Details.vue';
  import List from './ListServers.vue';
  export default {
    components: {
      'my-details': Details,
      'my-list': List,
    }
  }
</script>

<style>

</style>
