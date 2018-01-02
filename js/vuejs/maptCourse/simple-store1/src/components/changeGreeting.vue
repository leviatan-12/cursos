<template lang="html">
  <input type="text" :value="getMessage" @keyup="changeMessage" @keyup.enter="incrementalCounter">
</template>

<script>
import {mapGetters} from 'vuex';
import {mapActions} from 'vuex';

export default {
    computed: mapGetters(['getMessage']),
    methods: mapActions(['incrementalCounter','changeMessage']),
}
</script>

<style lang="css">
</style>
