<script setup lang="ts">
import { invoke, isTauri } from '@tauri-apps/api/core';
import { CircleStop } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import { saveLocalResult } from '../lib/local-ranking';
import { formatTime } from '../lib/time';

const emit = defineEmits<{
  stop: [elapsedMs: number, resultId: number | null];
}>();

const timeText = ref('00.000');

let startTime = 0;
let intervalId: ReturnType<typeof setInterval> | undefined;
let stopped = false;
const useTauriTimer = isTauri();

function updateDisplay() {
  timeText.value = formatTime(performance.now() - startTime);
}

// IPC が更新間隔(10ms)を超えた場合に invoke が重複し、応答順の逆転で表示が
// 巻き戻るのを防ぐためのロック
let isUpdating = false;

async function updateDisplayFromBackend() {
  if (isUpdating) return;
  isUpdating = true;
  try {
    const elapsedMs = await invoke<number>('get_elapsed_ms');
    if (stopped) return;
    timeText.value = formatTime(elapsedMs);
  } finally {
    isUpdating = false;
  }
}

function stopTimer() {
  if (stopped) return;
  stopped = true;
  if (intervalId !== undefined) {
    clearInterval(intervalId);
    intervalId = undefined;
  }

  if (useTauriTimer) {
    invoke<number>('stop_timer').then(async (elapsedMs) => {
      timeText.value = formatTime(elapsedMs);
      try {
        const resultId = await invoke<number>('save_result', { stoppedMs: elapsedMs });
        emit('stop', elapsedMs, resultId);
      } catch (err) {
        console.error('Failed to save result:', err);
        emit('stop', elapsedMs, null);
      }
    });
  } else {
    updateDisplay();
    // 保存値と表示値の丸めを揃えるため、floor したミリ秒に一本化する
    const elapsedMs = Math.max(0, Math.floor(performance.now() - startTime));
    const resultId = saveLocalResult(elapsedMs);
    emit('stop', elapsedMs, resultId);
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
  // start_timer の解決前にアンマウントされた場合でも setInterval が起動しないようにする
  stopped = true;
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
