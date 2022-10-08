import Vue from 'vue'
import VueRouter from 'vue-router'
import HomeView from '../views/home.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    component: HomeView
  },
  {
    path: '/status/:bridge',
    component: () => import('../views/status/index.vue')
  }
]

const router = new VueRouter({
  routes,
  mode: 'history',
})

export default router
