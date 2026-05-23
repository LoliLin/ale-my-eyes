<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const drawer = ref(false)
const theme = ref<'light' | 'dark'>('light')

const toggleTheme = () => {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
  document.documentElement.setAttribute('data-theme', theme.value)
}

const navItems = [
  { title: '首页', icon: 'mdi-home', path: '/' },
  { title: '模型管理', icon: 'mdi-brain', path: '/models' },
  { title: '设置', icon: 'mdi-cog', path: '/settings' },
]
</script>

<template>
  <v-app :theme="theme">
    <!-- 顶部导航栏 -->
    <v-app-bar color="primary" density="comfortable">
      <v-app-bar-nav-icon @click="drawer = !drawer" />
      <v-app-bar-title class="font-weight-bold">
        Ale, My Eyes!
      </v-app-bar-title>
      <v-spacer />
      <v-btn icon @click="toggleTheme">
        <v-icon>{{ theme === 'light' ? 'mdi-weather-night' : 'mdi-weather-sunny' }}</v-icon>
      </v-btn>
    </v-app-bar>

    <!-- 侧边导航栏 -->
    <v-navigation-drawer v-model="drawer" temporary>
      <v-list nav>
        <v-list-item
          v-for="item in navItems"
          :key="item.path"
          :prepend-icon="item.icon"
          :title="item.title"
          :to="item.path"
          @click="drawer = false"
        />
      </v-list>
    </v-navigation-drawer>

    <!-- 主要内容 -->
    <v-main>
      <v-container fluid>
        <router-view />
      </v-container>
    </v-main>

    <!-- 底部导航栏 (移动端) -->
    <v-bottom-navigation grow>
      <v-btn v-for="item in navItems" :key="item.path" :to="item.path">
        <v-icon>{{ item.icon }}</v-icon>
        <span>{{ item.title }}</span>
      </v-btn>
    </v-bottom-navigation>
  </v-app>
</template>

<style scoped>
.v-bottom-navigation {
  display: none;
}

@media (max-width: 960px) {
  .v-bottom-navigation {
    display: flex;
  }
}
</style>