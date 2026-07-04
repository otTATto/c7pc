<script setup lang="ts">
import { ref } from 'vue';
import PlayScreen from './components/PlayScreen.vue';
import RankingScreen from './components/RankingScreen.vue';
import ResultScreen from './components/ResultScreen.vue';
import TopScreen from './components/TopScreen.vue';

type Screen = 'top' | 'play' | 'result' | 'ranking';

const screen = ref<Screen>('top');
const resultMs = ref(0);
const resultId = ref<number | null>(null);

function goToPlay() {
  screen.value = 'play';
}

function goToResult(elapsedMs: number, id: number | null) {
  resultMs.value = elapsedMs;
  resultId.value = id;
  screen.value = 'result';
}

function goToRanking() {
  screen.value = 'ranking';
}

function goToTop() {
  screen.value = 'top';
}
</script>

<template>
  <main>
    <TopScreen v-if="screen === 'top'" @start="goToPlay" />
    <PlayScreen v-else-if="screen === 'play'" @stop="goToResult" />
    <ResultScreen
      v-else-if="screen === 'result'"
      :result-ms="resultMs"
      @view-ranking="goToRanking"
    />
    <RankingScreen v-else :result-ms="resultMs" :result-id="resultId" @restart="goToTop" />
  </main>
</template>
