<script setup lang="ts">
import { CirclePlay, Star } from 'lucide-vue-next';
import { onMounted, onUnmounted } from 'vue';
import pressSpaceKey from '../assets/img/PressSpaceKey.webp';
import logo from '../assets/img/logo.webp';

const emit = defineEmits<{
  start: [];
}>();

function start() {
  emit('start');
}

function handleKeydown(e: KeyboardEvent) {
  if (e.code === 'Space' && !e.repeat) {
    e.preventDefault();
    start();
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div class="flex min-h-screen flex-col items-center pb-16">
    <img :src="logo" width="500" class="mt-24" alt="7CHAL" />

    <div class="flex justify-center items-center mt-20 w-3xl px-4">
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
