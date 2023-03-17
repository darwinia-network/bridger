/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */

// Plugins
import { loadFonts } from './webfontloader'
import vuetify from './vuetify'
import pinia from '../store'
import router from '../router'

// @ts-ignore
import VueClipboard from 'vue3-clipboard'
import Subql from './subql'

// Types
import type { App } from 'vue'

export function registerPlugins (app: App) {
  loadFonts()
  app
    .use(vuetify)
    .use(router)
    .use(pinia)
    .use(VueClipboard, {
      autoSetContainer: true,
      appendToBody: true,
    })
    // @ts-ignore
    .use(Subql)
}
