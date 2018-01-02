<template>
  <div>
    <input type="text" v-model="nuevoEle">
    <button type="button" @click="agregarNuevo">Agregar!</button>
  </div>
</template>

<script type="text/javascript">
  export default {
    methods: {
      agregarNuevo() {
        if (this.nuevoEle.trim().length > 0) {
          this.$emit('agregar', this.nuevoEle);
          this.nuevoEle = '';
        }
      },
    },
    data() {
      return {
        nuevoEle: '',
      };
    },
  };

</script>
