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
    path: '/state/:bridge',
    component: () => import('../views/state/index.vue')
  }
]

const router = new VueRouter({
  routes,
  mode: 'history',
})

export default router
