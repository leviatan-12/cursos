<template>
    <div class="container">
      <my-header></my-header>
       <hr>
       <my-body></my-body>
        <hr>
        <div class="row">
          <my-footer></my-footer>
        </div>
    </div>
</template>

<script>
  import Header from './Header.vue';
  import Footer from './Footer.vue';
  import Body from './Body.vue';
  export default {
    components: {
      'my-header': Header,
      'my-body': Body,
      'my-footer': Footer
    }
  }
</script>

<style>

</style>
