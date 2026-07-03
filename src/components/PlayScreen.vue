<script setup lang="ts">
import { invoke, isTauri } from '@tauri-apps/api/core';
import { CircleStop } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';

const emit = defineEmits<{
  stop: [time: string];
}>();

const timeText = ref('00.000');

let startTime = 0;
let intervalId: ReturnType<typeof setInterval> | undefined;
let stopped = false;
const useTauriTimer = isTauri();

function formatTime(elapsedMs: number): string {
  const totalMs = Math.max(0, Math.floor(elapsedMs));
  const totalSeconds = Math.floor(totalMs / 1000);
  const s = String(totalSeconds % 60).padStart(2, '0');
  const ms = String(totalMs % 1000).padStart(3, '0');
  return `${s}.${ms}`;
}

function updateDisplay() {
  timeText.value = formatTime(performance.now() - startTime);
}

async function updateDisplayFromBackend() {
  const elapsedMs = await invoke<number>('get_elapsed_ms');
  if (stopped) return;
  timeText.value = formatTime(elapsedMs);
}

function stopTimer() {
  if (stopped) return;
  stopped = true;
  if (intervalId !== undefined) {
    clearInterval(intervalId);
    intervalId = undefined;
  }

  if (useTauriTimer) {
    invoke<number>('stop_timer').then((elapsedMs) => {
      emit('stop', formatTime(elapsedMs));
    });
  } else {
    updateDisplay();
    emit('stop', timeText.value);
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.code === 'Space' && !e.repeat) {
    e.preventDefault();
    stopTimer();
  }
}

onMounted(() => {
  if (useTauriTimer) {
    invoke('start_timer').then(() => {
      if (stopped) return;
      intervalId = setInterval(updateDisplayFromBackend, 10);
    });
  } else {
    startTime = performance.now();
    updateDisplay();
    intervalId = setInterval(updateDisplay, 10);
  }
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  if (intervalId !== undefined) {
    clearInterval(intervalId);
  }
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div class="flex min-h-screen flex-col items-center justify-center gap-16 px-4">
    <div
      class="font-mplus-rounded text-water animate-[fadeout_3s_ease-in-out_1s_forwards] text-center text-[200px] leading-none font-black"
    >
      {{ timeText }}
    </div>

    <div class="font-zen-kaku text-center font-bold text-gray-500">
      <span class="font-mplus-rounded text-water font-black">7.777秒</span>
      でボタンを押してね！
    </div>

    <button
      id="stop-btn"
      type="button"
      class="bg-stop-red w-full max-w-3xl rounded-[30px] py-10 transition-transform duration-300 hover:scale-95"
      @click="stopTimer"
    >
      <span class="font-zen-kaku inline-flex items-center gap-2 text-3xl font-bold text-white">
        <CircleStop class="size-8" />
        ストップ！
      </span>
    </button>
  </div>
</template>
