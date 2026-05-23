import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './style.css'

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import '@mdi/font/css/materialdesignicons.css'

const vuetify = createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'light',
    themes: {
      light: {
        colors: {
          primary: '#1565C0',
          secondary: '#00897B',
          accent: '#FF6F00',
          error: '#B00020',
          success: '#2E7D32',
          warning: '#F57F17',
        },
      },
      dark: {
        colors: {
          primary: '#64B5F6',
          secondary: '#5FD0C3',
          accent: '#FFB86B',
          error: '#FFB4AB',
          success: '#81C784',
          warning: '#FFD54F',
        },
      },
    },
  },
})

const app = createApp(App)
app.use(router)
app.use(vuetify)
app.mount('#app')