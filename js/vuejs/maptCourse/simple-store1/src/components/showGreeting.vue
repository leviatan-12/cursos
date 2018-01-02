<template lang="html">
  <div>
    <h1> {{ getMessage }} </h1>
    <p>El mensaje se ha cambiado {{ getCounter }} veces</p>
  </div>
</template>

<script>
import {mapGetters} from 'vuex';

export default {
    computed: mapGetters(['getMessage','getCounter']),
}
</script>
