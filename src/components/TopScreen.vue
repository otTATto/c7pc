<script setup lang="ts">
import { isTauri } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { CirclePlay, Maximize, Minimize, Star } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import logo from '../assets/img/logo.webp';
import pressSpaceKey from '../assets/img/press-space-key.webp';

const emit = defineEmits<{
  start: [];
}>();

const showFullscreenToggle = ref(false);
const isFullscreen = ref(false);

function start() {
  emit('start');
}

function handleKeydown(e: KeyboardEvent) {
  if (e.code === 'Space' && !e.repeat) {
    e.preventDefault();
    start();
  }
}

async function toggleFullscreen(e: MouseEvent) {
  // e.currentTarget はイベントディスパッチ終了後(= await 後)は null になるため先に退避する
  const button = e.currentTarget as HTMLButtonElement | null;
  try {
    const next = !isFullscreen.value;
    await getCurrentWindow().setFullscreen(next);
    isFullscreen.value = next;
  } catch (err) {
    console.error('Failed to toggle fullscreen:', err);
  } finally {
    // フォーカスを外し、直後の Space キーがトグルではなくゲーム開始として働くようにする
    button?.blur();
  }
}

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown);

  if (isTauri()) {
    showFullscreenToggle.value = true;
    isFullscreen.value = await getCurrentWindow().isFullscreen();
  }
});
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div class="flex min-h-screen flex-col items-center justify-center -mt-10 pb-16">
    <button
      v-if="showFullscreenToggle"
      type="button"
      :title="isFullscreen ? 'ウィンドウ表示' : '全画面表示'"
      class="fixed top-4 right-4 inline-flex items-center gap-2 rounded-full bg-gray-200 px-3 py-1.5 text-xs font-bold text-gray-500 transition-colors duration-200 hover:bg-gray-300"
      @click="toggleFullscreen"
    >
      <Minimize v-if="isFullscreen" class="size-4" />
      <Maximize v-else class="size-4" />
      <span>{{ isFullscreen ? 'ウィンドウ表示' : '全画面表示' }}</span>
    </button>

    <img :src="logo" width="500" alt="7CHAL" />

    <div class="mt-20 flex w-full max-w-3xl items-center justify-center px-4">
      <div class="font-zen-kaku font-bold text-gray-500">
        <p>
          <Star class="text-yello inline-block size-5 align-middle mx-1 -translate-y-[3px]" />
          ボタンを押すとカウントアップが始まります
        </p>
        <p>
          <Star class="text-yello inline-block size-5 align-middle mx-1 -translate-y-[3px]" />
          途中からカウントの数字が見えなくなります
        </p>
        <p>
          <Star class="text-yello inline-block size-5 align-middle mx-1 -translate-y-[3px]" />
          <span class="font-mplus-rounded text-water font-black">7.777秒</span>
          のタイミングで
          <span class="font-black">ボタンを押してください！</span>
        </p>
      </div>
    </div>

    <div class="relative mt-10 w-full max-w-3xl px-4">
      <button
        id="start-btn"
        type="button"
        class="bg-salmon w-full rounded-[30px] py-10 transition-transform duration-300 hover:scale-95"
        @click="start"
      >
        <span class="font-zen-kaku inline-flex items-center gap-2 text-3xl font-bold text-white">
          <CirclePlay class="size-8 translate-y-0.25" />
          スタート！
        </span>
      </button>

      <img
        :src="pressSpaceKey"
        width="300"
        alt="すべてのボタンはスペースキーでも押下できます"
        class="pointer-events-none absolute top-0 right-0 -translate-y-70 translate-x-20"
      />
    </div>

    <footer class="fixed bottom-0 left-0 w-full pb-3">
      <p class="font-zen-maru text-center text-sm font-bold text-gray-500">
        © たと / メディアアートサークル C4's・2026
      </p>
    </footer>
  </div>
</template>
