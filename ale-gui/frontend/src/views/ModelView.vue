<script setup lang="ts">
import { ref, reactive } from 'vue'

const models = reactive([
  {
    id: 'whisper-tiny',
    name: 'Whisper Tiny',
    size: '75MB',
    status: 'downloaded',
    purpose: '基础语音识别',
  },
  {
    id: 'whisper-small',
    name: 'Whisper Small',
    size: '244MB',
    status: 'available',
    purpose: '高质量语音识别',
  },
  {
    id: 'piper-zh_CN',
    name: 'Piper 中文语音',
    size: '50MB',
    status: 'downloaded',
    purpose: '中文语音合成',
  },
])

const downloading = ref<string | null>(null)

const downloadModel = (modelId: string) => {
  downloading.value = modelId
  // 模拟下载
  setTimeout(() => {
    const model = models.find(m => m.id === modelId)
    if (model) {
      model.status = 'downloaded'
    }
    downloading.value = null
  }, 2000)
}

const deleteModel = (modelId: string) => {
  const model = models.find(m => m.id === modelId)
  if (model) {
    model.status = 'available'
  }
}
</script>

<template>
  <div class="am-models">
    <v-card class="mb-6">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-brain</v-icon>
        模型管理
      </v-card-title>
      <v-card-text>
        <v-list>
          <v-list-item
            v-for="model in models"
            :key="model.id"
            class="mb-2"
          >
            <template #prepend>
              <v-icon :color="model.status === 'downloaded' ? 'success' : 'grey'">
                {{ model.status === 'downloaded' ? 'mdi-check-circle' : 'mdi-download' }}
              </v-icon>
            </template>

            <v-list-item-title class="font-weight-bold">
              {{ model.name }}
            </v-list-item-title>
            <v-list-item-subtitle>
              {{ model.purpose }} • {{ model.size }}
            </v-list-item-subtitle>

            <template #append>
              <v-btn
                v-if="model.status === 'available'"
                color="primary"
                variant="outlined"
                size="small"
                :loading="downloading === model.id"
                @click="downloadModel(model.id)"
              >
                下载
              </v-btn>
              <v-btn
                v-else
                color="error"
                variant="outlined"
                size="small"
                @click="deleteModel(model.id)"
              >
                删除
              </v-btn>
            </template>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card>
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-information</v-icon>
        模型说明
      </v-card-title>
      <v-card-text>
        <div class="text-medium-emphasis">
          <p class="mb-2"><strong>Whisper Tiny</strong>: 轻量级语音识别模型，适合低性能设备</p>
          <p class="mb-2"><strong>Whisper Small</strong>: 中等质量语音识别，平衡性能和准确度</p>
          <p><strong>Piper</strong>: 本地语音合成，支持中英文</p>
        </div>
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>
.am-models {
  max-width: 800px;
  margin: 0 auto;
}
</style>