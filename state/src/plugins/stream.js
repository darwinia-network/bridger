import Streamjs from 'streamjs'


export default {
  install: function (Vue) {
    Vue.prototype.$stream = Streamjs
  }
}

