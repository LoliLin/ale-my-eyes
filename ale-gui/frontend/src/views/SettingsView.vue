<script setup lang="ts">
import { ref, reactive } from 'vue'

const settings = reactive({
  apiKey: '',
  apiProvider: 'openai',
  inferenceMode: 'adaptive',
  autoDownload: true,
  language: 'zh-CN',
  highContrast: false,
  fontSize: 16,
})

const apiProviders = [
  { title: 'OpenAI', value: 'openai' },
  { title: 'Anthropic', value: 'anthropic' },
  { title: '自定义', value: 'custom' },
]

const inferenceModes = [
  { title: '自适应', value: 'adaptive' },
  { title: '仅本地', value: 'local' },
  { title: '仅云端', value: 'cloud' },
]

const languages = [
  { title: '简体中文', value: 'zh-CN' },
  { title: 'English', value: 'en' },
]

const saveSettings = () => {
  // 保存设置到本地存储
  localStorage.setItem('ale-settings', JSON.stringify(settings))
  alert('设置已保存')
}
</script>

<template>
  <div class="am-settings">
    <v-card class="mb-6">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-key</v-icon>
        API 配置
      </v-card-title>
      <v-card-text>
        <v-select
          v-model="settings.apiProvider"
          :items="apiProviders"
          label="API 提供商"
          class="mb-4"
        />
        <v-text-field
          v-model="settings.apiKey"
          label="API 密钥"
          type="password"
          placeholder="sk-your-api-key-here"
          class="mb-4"
        />
      </v-card-text>
    </v-card>

    <v-card class="mb-6">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-brain</v-icon>
        推理设置
      </v-card-title>
      <v-card-text>
        <v-select
          v-model="settings.inferenceMode"
          :items="inferenceModes"
          label="推理模式"
          class="mb-4"
        />
        <v-switch
          v-model="settings.autoDownload"
          label="自动下载推荐模型"
          color="primary"
          class="mb-4"
        />
      </v-card-text>
    </v-card>

    <v-card class="mb-6">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-palette</v-icon>
        界面设置
      </v-card-title>
      <v-card-text>
        <v-select
          v-model="settings.language"
          :items="languages"
          label="语言"
          class="mb-4"
        />
        <v-slider
          v-model="settings.fontSize"
          label="字体大小"
          min="12"
          max="24"
          step="1"
          class="mb-4"
        />
        <v-switch
          v-model="settings.highContrast"
          label="高对比度模式"
          color="primary"
          class="mb-4"
        />
      </v-card-text>
    </v-card>

    <v-btn
      color="primary"
      size="large"
      block
      @click="saveSettings"
    >
      <v-icon class="mr-2">mdi-content-save</v-icon>
      保存设置
    </v-btn>
  </div>
</template>

<style scoped>
.am-settings {
  max-width: 800px;
  margin: 0 auto;
}
</style>