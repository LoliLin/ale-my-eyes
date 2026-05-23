<script setup lang="ts">
defineProps<{
  status: 'idle' | 'listening' | 'processing' | 'error'
  text: string
}>()
</script>

<template>
  <v-card class="am-status-card" variant="outlined">
    <v-card-text>
      <div class="d-flex align-center">
        <v-icon
          :color="status === 'idle' ? 'success' : status === 'listening' ? 'primary' : status === 'processing' ? 'warning' : 'error'"
          size="48"
          class="mr-4"
        >
          {{ status === 'listening' ? 'mdi-microphone' : status === 'processing' ? 'mdi-loading' : status === 'error' ? 'mdi-alert' : 'mdi-check-circle' }}
        </v-icon>
        <div>
          <div class="text-h6 font-weight-bold">系统状态</div>
          <div class="text-medium-emphasis">{{ text }}</div>
        </div>
      </div>
    </v-card-text>
  </v-card>
</template>

<style scoped>
.am-status-card {
  border-left: 4px solid var(--am-primary);
  transition: all var(--am-transition);
}

.am-status-card:hover {
  box-shadow: var(--am-shadow-4);
}
</style>