<script setup lang="ts">
import { Frown, Laugh, Meh } from 'lucide-vue-next';
import { computed, onMounted, onUnmounted } from 'vue';
import { diffSeconds, formatDiff, formatTime } from '../lib/time';

const props = defineProps<{
  resultMs: number;
}>();

const emit = defineEmits<{
  viewRanking: [];
}>();

const timeText = computed(() => formatTime(props.resultMs));

const distance = computed(() => diffSeconds(props.resultMs));

type MessageKind = 'bad' | 'good' | 'okay';

const messageKind = computed<MessageKind>(() => {
  const d = distance.value;
  if (d > 1 || d < -1) return 'bad';
  if (d < 0.1 && d > -0.1) return 'good';
  return 'okay';
});

const distanceColorClass = computed(() => {
  const d = distance.value;
  if (d > 0.1) return 'text-salmon';
  if (d < -0.1) return 'text-blue';
  return 'text-sub-yello';
});

const distanceText = computed(() => formatDiff(distance.value));

function viewRanking() {
  emit('viewRanking');
}

function handleKeydown(e: KeyboardEvent) {
  if (e.code === 'Space' && !e.repeat) {
    e.preventDefault();
    viewRanking();
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
  <div class="flex min-h-screen flex-col items-center justify-center px-4 -mt-5">
    <div class="font-zen-kaku text-center font-bold text-gray-500">結果は…</div>

    <div class="font-mplus-rounded text-sub-blue mt-3 text-center text-3xl font-bold">
      <span class="text-water text-[50px] font-black">{{ timeText }}</span>
      秒
    </div>

    <div
      class="bg-sub-greige font-zen-maru text-moca mt-10 max-w-xl rounded-[20px] px-8 py-6 text-center font-bold"
    >
      <div class="flex items-center justify-center gap-2 text-3xl font-black">
        <Frown v-if="messageKind === 'bad'" class="size-8" />
        <Laugh v-else-if="messageKind === 'good'" class="size-8" />
        <Meh v-else class="size-8" />
        <span v-if="messageKind === 'bad'">ざんねん！</span>
        <span v-else-if="messageKind === 'good'">すごい！</span>
        <span v-else>おしい…！</span>
      </div>
      <div class="font-mplus-rounded mt-2 text-3xl">
        <span class="text-[60px] font-black" :class="distanceColorClass">{{ distanceText }}</span>
        秒
      </div>
    </div>

    <button
      id="view-ranking-btn"
      type="button"
      class="bg-sub-blue mt-24 rounded-[30px] px-16 py-6 transition-transform duration-300 hover:scale-95"
      @click="viewRanking"
    >
      <span class="font-zen-kaku text-2xl font-bold text-white">今日のランキングを見に行く</span>
    </button>
  </div>
</template>
