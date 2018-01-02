import {CHANGE_MSG, INCREMENTAL_COUNTER} from './mutation_types';

export default {
  changeMessage({commit},msg) {
    commit(CHANGE_MSG,msg)
  },
  incrementalCounter({counter}) {
    commit(INCREMENTAL_COUNTER)
  }
}
