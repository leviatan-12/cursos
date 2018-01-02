import Vue from 'vue';
import Vuex from 'vuex';
import getters from './getters';
import {CHANGE_MSG, INCREMENTAL_COUNTER} from './mutation_types';

Vue.use(Vuex);

const state = {
  msg: 'Hola vue',
  counter: 0
}

const mutations = {
  [CHANGE_MSG](state, msg){
    state.msg = msg;
  },
  [INCREMENTAL_COUNTER](state){
    state.counter++;
  }
};

export default new Vuex.Store({
  state,mutations,getters,actions
});
