<template>
  <div>
    <show-greeting :msg="msg"></show-greeting>
    <change-greeting :msg="msg"></change-greeting>
  </div>
</template>

<script>
import ShowGreeting from './components/showGreeting';
import ChangeGreeting from './components/changeGreeting';
import store from './vuex/store';

export default {
  components: {ShowGreeting, ChangeGreeting},
  store
}
</script>
