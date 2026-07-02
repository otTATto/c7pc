<script setup lang="ts">
const emit = defineEmits<{
  start: [];
}>();

function start() {
  emit('start');
}
</script>

<template>
  <div class="flex min-h-screen flex-col items-center justify-center">
    <button type="button" @click="start">スタート！</button>
  </div>
</template>
