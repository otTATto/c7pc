<script setup lang="ts">
const emit = defineEmits<{
  stop: [time: string];
}>();

function stopTimer() {
  emit('stop', '00.000');
}
</script>

<template>
  <div class="flex min-h-screen flex-col items-center justify-center">
    <button type="button" @click="stopTimer">ストップ！</button>
  </div>
</template>
