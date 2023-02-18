import { createRouter, createWebHistory } from 'vue-router'
import Body from "@/components/Body.vue";


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Body,
      props: route => ({ query: route.query, view: "artists" })
    },
    {
      path: '/albums',
      name: 'albums',
      component: Body,
      props: route => ({ query: route.query, view: "albums" })
    },
  ]
})

export default router
