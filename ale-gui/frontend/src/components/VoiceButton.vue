<script setup lang="ts">
defineProps<{
  isRecording: boolean
}>()

defineEmits<{
  toggle: []
}>()
</script>

<template>
  <v-btn
    :class="{ 'recording': isRecording }"
    class="am-voice-btn"
    size="x-large"
    @click="$emit('toggle')"
  >
    <v-icon size="36">
      {{ isRecording ? 'mdi-stop' : 'mdi-microphone' }}
    </v-icon>
  </v-btn>
</template>

<style scoped>
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

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>