<script setup lang="ts">
import { invoke, isTauri } from '@tauri-apps/api/core';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { getLocalTodayRanking, type RankingData, type RankingEntry } from '../lib/local-ranking';
import { diffSeconds, formatDiff, formatTime } from '../lib/time';

const props = defineProps<{
  resultMs: number;
  resultId: number | null;
}>();

const emit = defineEmits<{
  restart: [];
}>();

const loading = ref(true);
const errorMessage = ref('');
const ranking = ref<RankingData | null>(null);

type Row = { kind: 'entry'; entry: RankingEntry } | { kind: 'ellipsis'; key: 'upper' | 'lower' };

// 表示行を組み立てる:
// - 自分が 1〜10 位: top（最大 10 行）をそのまま表示
//   - 総件数が 10 件を超える場合、10 位より下に隠れた行があることを示す下の ⋮ を追加する
// - 自分が 11 位以下: 上位 10 行 + (必要なら)上の ⋮ + 自分の行 + (必要なら)下の ⋮
//   - 自分が 11 位ちょうどの場合、上位 10 行のすぐ下が自分なので上の ⋮ は省く
//   - 自分が最下位（総件数と同順位）の場合、下に隠れている行が無いので下の ⋮ は省く
const rows = computed<Row[]>(() => {
  const data = ranking.value;
  if (!data) return [];

  const { top, mine, total } = data;

  if (mine.rank <= 10) {
    const topRows: Row[] = top.map((entry) => ({ kind: 'entry', entry }) as const);
    if (total > 10) {
      topRows.push({ kind: 'ellipsis', key: 'lower' });
    }
    return topRows;
  }

  const result: Row[] = top.map((entry) => ({ kind: 'entry', entry }) as const);

  // 自分の 1 つ上(mine.rank - 1)が top 10 に含まれない = 隠れた行があるとき ⋮ を出す
  const hasGapAboveMine = mine.rank - 1 > 10;
  if (hasGapAboveMine) {
    result.push({ kind: 'ellipsis', key: 'upper' });
  }

  result.push({ kind: 'entry', entry: mine });

  const hasRowsBelowMine = mine.rank < total;
  if (hasRowsBelowMine) {
    result.push({ kind: 'ellipsis', key: 'lower' });
  }

  return result;
});

const isTopThree = computed(() => ranking.value !== null && ranking.value.mine.rank <= 3);

function rankColorClass(entry: RankingEntry): string {
  if (!entry.isMine) return '';
  switch (entry.rank) {
    case 1:
      return 'text-medal-gold';
    case 2:
      return 'text-medal-silver';
    case 3:
      return 'text-medal-bronze';
    default:
      return '';
  }
}

function diffText(stoppedMs: number): string {
  return formatDiff(diffSeconds(stoppedMs));
}

function diffColorClass(stoppedMs: number): string {
  const d = diffSeconds(stoppedMs);
  if (d > 0.1) return 'text-salmon';
  if (d < -0.1) return 'text-blue';
  return 'text-sub-yello';
}

// 紙吹雪 1 粒の見た目・落下タイミングをランダムに決めるための静的データ
// テーブルの可読性を妨げないよう、左右の余白帯（0-14% / 86-100%）にのみ配置する
const colors = ['bg-water', 'bg-salmon', 'bg-yello', 'bg-sub-blue', 'bg-stop-red'];
const confettiPieces = Array.from({ length: 24 }, (_, i) => {
  const side = i % 2 === 0 ? 'left' : 'right';
  const marginPercent = Math.random() * 14;
  const left = side === 'left' ? marginPercent : 100 - marginPercent;
  return {
    id: i,
    left,
    color: colors[i % colors.length],
    delay: Math.random() * 4,
    duration: 3 + Math.random() * 2.5,
    rotate: Math.random() * 360,
    side,
  };
});

async function loadRanking() {
  loading.value = true;
  errorMessage.value = '';
  try {
    if (props.resultId === null) {
      throw new Error('結果が見つかりませんでした');
    }
    if (isTauri()) {
      ranking.value = await invoke<RankingData>('get_today_ranking', {
        resultId: props.resultId,
      });
    } else {
      ranking.value = getLocalTodayRanking(props.resultId);
    }
  } catch (err) {
    console.error('Failed to load ranking:', err);
    errorMessage.value = 'ランキングの取得に失敗しました';
  } finally {
    loading.value = false;
  }
}

function restart() {
  emit('restart');
}

function handleKeydown(e: KeyboardEvent) {
  if (e.code === 'Space' && !e.repeat) {
    e.preventDefault();
    restart();
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
  loadRanking();
});
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div
    class="relative flex min-h-screen flex-col items-center justify-center overflow-hidden px-4 py-16"
  >
    <template v-if="isTopThree">
      <div
        v-for="piece in confettiPieces"
        :key="piece.id"
        class="confetti-piece pointer-events-none absolute top-[-40px] size-3 rounded-sm"
        :class="[piece.color, piece.side === 'left' ? 'confetti-left' : 'confetti-right']"
        :style="{
          left: `${piece.left}%`,
          animationDelay: `${piece.delay}s`,
          animationDuration: `${piece.duration}s`,
          transform: `rotate(${piece.rotate}deg)`,
        }"
      />
    </template>

    <div class="font-zen-kaku text-center font-bold text-gray-500">今日のランキング</div>

    <div v-if="loading" class="font-zen-maru text-moca mt-10 font-bold">読み込み中…</div>

    <div v-else-if="errorMessage" class="mt-10 flex flex-col items-center gap-8">
      <div class="font-zen-maru text-moca font-bold">{{ errorMessage }}</div>
      <button
        type="button"
        class="bg-sub-blue rounded-[30px] px-16 py-6 transition-transform duration-300 hover:scale-95"
        @click="restart"
      >
        <span class="font-zen-kaku text-2xl font-bold text-white">はじめにもどる</span>
      </button>
    </div>

    <template v-else-if="ranking">
      <div
        class="font-zen-maru text-moca mt-4 text-center text-sm font-bold"
        data-testid="ranking-total"
      >
        本日のプレイ数: {{ ranking.total }} 件
      </div>

      <table class="font-mplus-rounded mt-8 w-full max-w-xl border-collapse">
        <thead>
          <tr class="border-b-moca/40 border-b-2">
            <th class="text-moca px-3 py-2 text-left text-sm font-bold">順位</th>
            <th class="text-moca px-3 py-2 text-right text-sm font-bold">タイム</th>
            <th class="text-moca px-3 py-2 text-right text-sm font-bold">7.777秒との差</th>
          </tr>
        </thead>
        <tbody>
          <template
            v-for="row in rows"
            :key="row.kind === 'entry' ? `entry-${row.entry.rank}` : row.key"
          >
            <tr v-if="row.kind === 'ellipsis'" class="border-b-moca/20 border-b">
              <td colspan="3" class="text-moca/60 py-2 text-center tracking-widest">⋮</td>
            </tr>
            <tr
              v-else
              class="border-b-moca/20 border-b"
              :class="row.entry.isMine ? 'bg-water/15 font-bold' : ''"
            >
              <td class="px-3 py-2 text-left font-black" :class="rankColorClass(row.entry)">
                {{ row.entry.rank }}
              </td>
              <td class="px-3 py-2 text-right">{{ formatTime(row.entry.stoppedMs) }}秒</td>
              <td class="px-3 py-2 text-right" :class="diffColorClass(row.entry.stoppedMs)">
                {{ diffText(row.entry.stoppedMs) }}秒
              </td>
            </tr>
          </template>
        </tbody>
      </table>

      <button
        id="back2start-btn"
        type="button"
        class="bg-sub-blue mt-16 rounded-[30px] px-16 py-6 transition-transform duration-300 hover:scale-95"
        @click="restart"
      >
        <span class="font-zen-kaku text-2xl font-bold text-white">はじめにもどる</span>
      </button>
    </template>
  </div>
</template>

<style scoped>
@keyframes confetti-fall-left {
  0% {
    transform: translateY(0) rotate(0deg);
    opacity: 1;
  }

  100% {
    transform: translateY(105vh) rotate(360deg);
    opacity: 0.8;
  }
}

@keyframes confetti-fall-right {
  0% {
    transform: translateY(0) rotate(0deg);
    opacity: 1;
  }

  100% {
    transform: translateY(105vh) rotate(-360deg);
    opacity: 0.8;
  }
}

.confetti-piece {
  animation-timing-function: linear;
  animation-iteration-count: infinite;
}

.confetti-left {
  animation-name: confetti-fall-left;
}

.confetti-right {
  animation-name: confetti-fall-right;
}
</style>
