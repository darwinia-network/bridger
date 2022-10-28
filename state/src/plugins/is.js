import is from 'is_js'

export default {
  install: function(Vue) {
    Vue.prototype.$is = is
  }
}

