<template >
  <div>
    <elemento v-for="ele in lista" :elemento="ele" :key="ele.nombre"></elemento>
  </div>
</template>

<script type="text/javascript">
  import Elemento from './Elemento';

  export default {
    props: ['lista'],
    components: {
      Elemento,
    },
  };
</script>
