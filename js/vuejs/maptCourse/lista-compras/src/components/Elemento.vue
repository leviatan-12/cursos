<template>
    <div :class="{tachado:elemento.eliminado}">
      <p><input type="checkbox" :checked="elemento.eliminado" @click="elemento.eliminado=!elemento.eliminado">{{ elemento.nombre }}</p>
    </div>
</template>

<script type="text/javascript">
  export default {
    props: ['elemento'],
  };
</script>

<style scoped media="screen">
  .tachado {
    text-decoration: line-through;
  }
</style>
