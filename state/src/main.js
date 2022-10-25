import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import router from './router'

import VueClipboard from 'vue-clipboard2'
import VueLocalStorage from 'vue-localstorage'
import VueFilterDateFormat from '@vuejs-community/vue-filter-date-format'
import VueFilterDateParse from '@vuejs-community/vue-filter-date-parse'
import VueEllipsis from 'vue-ellipsis-component'

import Is from './plugins/is'
import Subql from './plugins/subql'
import Stream from './plugins/stream'
import Eth2 from './plugins/eth2'

import './plugins/logger'
import './plugins/toast'
import './plugins/axios'
import './styles/app.css'

Vue.config.productionTip = false

Vue.use(VueFilterDateFormat);
Vue.use(VueFilterDateParse);
Vue.use(VueLocalStorage, {
  name: 'storage',
  bind: true
});
Vue.use(VueClipboard);
Vue.use(VueEllipsis);
Vue.use(Stream);
Vue.use(Is);
Vue.use(Subql);
Vue.use(Eth2);


new Vue({
  vuetify,
  router,
  render: h => h(App)
}).$mount('#app')
