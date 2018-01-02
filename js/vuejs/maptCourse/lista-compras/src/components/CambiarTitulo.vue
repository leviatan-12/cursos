<template>
  <div>
    <p>Título de la lista: <input type="text" placeholder="Inserte título" v-model="titulo"></p>
  </div>
</template>

<script type="text/javascript">
  export default {
    props: ['titulo'],
  };
</script>
