<script setup lang="ts">
defineProps<{
  time: string;
}>();

const emit = defineEmits<{
  restart: [];
}>();

function restart() {
  emit('restart');
}
</script>

<template>
  <div class="flex min-h-screen flex-col items-center justify-center">
    <p>{{ time }}</p>
    <button type="button" @click="restart">はじめにもどる</button>
  </div>
</template>
