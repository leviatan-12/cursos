<template>
  <div id="app">
    <h2>{{ titulo }}</h2>
    <nuevo-elemento @agregar="agregar"></nuevo-elemento>
    <lista-elementos  :lista="listaCompra"></lista-elementos>
    <cambiar-titulo :titulo="titulo"></cambiar-titulo>
  </div>
</template>

<script>
  import CambiarTitulo from './components/CambiarTitulo';
  import NuevoElemento from './components/NuevoElemento';
  import ListaElementos from './components/ListaElementos';

  export default {
    components: {
      CambiarTitulo,
      NuevoElemento,
      ListaElementos,
    },
    data() {
      return {
        titulo: 'Mi lista',
        listaCompra: [
          { nombre: 'Plátano', eliminado: false },
          { nombre: 'Fresas', eliminado: false },
          { nombre: 'Piñas', eliminado: true },
        ],
      };
    },
    methods: {
      agregar(text) {
        this.listaCompra.push({ nombre: text, eliminado: false });
      },
    },
  };
</script>
