<script setup lang="ts">
import { ref, computed } from 'vue'

// 模拟状态
const isRecording = ref(false)
const isProcessing = ref(false)
const lastResult = ref('')
const status = ref<'idle' | 'listening' | 'processing' | 'error'>('idle')

const statusText = computed(() => {
  switch (status.value) {
    case 'idle': return '就绪'
    case 'listening': return '正在聆听...'
    case 'processing': return '处理中...'
    case 'error': return '错误'
    default: return '就绪'
  }
})

const statusColor = computed(() => {
  switch (status.value) {
    case 'idle': return 'success'
    case 'listening': return 'primary'
    case 'processing': return 'warning'
    case 'error': return 'error'
    default: return 'success'
  }
})

const toggleRecording = () => {
  if (isRecording.value) {
    isRecording.value = false
    status.value = 'processing'
    // 模拟处理
    setTimeout(() => {
      lastResult.value = '这是一条语音识别结果示例'
      status.value = 'idle'
    }, 2000)
  } else {
    isRecording.value = true
    status.value = 'listening'
  }
}

const describeImage = () => {
  status.value = 'processing'
  setTimeout(() => {
    lastResult.value = '这是一张示例图片的描述'
    status.value = 'idle'
  }, 1500)
}
</script>

<template>
  <div class="am-home">
    <!-- 状态卡片 -->
    <v-card class="am-status-card mb-6" variant="outlined">
      <v-card-text>
        <div class="d-flex align-center">
          <v-icon :color="statusColor" size="48" class="mr-4">
            {{ status === 'listening' ? 'mdi-microphone' : status === 'processing' ? 'mdi-loading' : 'mdi-check-circle' }}
          </v-icon>
          <div>
            <div class="text-h6 font-weight-bold">系统状态</div>
            <div class="text-medium-emphasis">{{ statusText }}</div>
          </div>
        </div>
      </v-card-text>
    </v-card>

    <!-- 语音控制区域 -->
    <v-card class="mb-6">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-microphone</v-icon>
        语音交互
      </v-card-title>
      <v-card-text>
        <div class="d-flex flex-column align-center">
          <v-btn
            :class="{ 'recording': isRecording }"
            class="am-voice-btn mb-4"
            size="x-large"
            @click="toggleRecording"
          >
            <v-icon size="36">
              {{ isRecording ? 'mdi-stop' : 'mdi-microphone' }}
            </v-icon>
          </v-btn>
          <div class="text-center">
            <div class="text-h6 font-weight-bold">
              {{ isRecording ? '点击停止' : '点击开始录音' }}
            </div>
            <div class="text-medium-emphasis">
              支持中英文语音识别
            </div>
          </div>
        </div>
      </v-card-text>
    </v-card>

    <!-- 图像描述区域 -->
    <v-card class="mb-6">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-image</v-icon>
        图像描述
      </v-card-title>
      <v-card-text>
        <v-btn
          color="secondary"
          size="large"
          block
          @click="describeImage"
          :loading="status === 'processing'"
        >
          <v-icon class="mr-2">mdi-camera</v-icon>
          上传图像并描述
        </v-btn>
      </v-card-text>
    </v-card>

    <!-- 结果显示区域 -->
    <v-card v-if="lastResult">
      <v-card-title class="text-h5 font-weight-bold">
        <v-icon class="mr-2">mdi-text-box</v-icon>
        识别结果
      </v-card-title>
      <v-card-text>
        <div class="am-result-text">
          {{ lastResult }}
        </div>
        <v-btn
          variant="outlined"
          class="mt-4"
          @click="lastResult = ''"
        >
          清除结果
        </v-btn>
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>
.am-home {
  max-width: 800px;
  margin: 0 auto;
}

.am-voice-btn {
  width: 100px;
  height: 100px;
  border-radius: 50%;
  background: var(--am-primary);
  color: white;
  box-shadow: var(--am-shadow-4);
  transition: all var(--am-transition);
}

.am-voice-btn:hover {
  transform: scale(1.05);
  box-shadow: var(--am-shadow-8);
}

.am-voice-btn.recording {
  background: var(--am-error);
  animation: pulse 1.5s infinite;
}

.am-result-text {
  font-size: 1.1rem;
  line-height: 1.6;
  color: var(--am-text-high);
  background: var(--am-surface-2);
  padding: var(--am-space-md);
  border-radius: var(--am-radius-md);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>