<script setup lang="ts">
import { ref } from 'vue';
import PlayScreen from './components/PlayScreen.vue';
import ResultScreen from './components/ResultScreen.vue';
import TopScreen from './components/TopScreen.vue';

type Screen = 'top' | 'play' | 'result';

const screen = ref<Screen>('top');
const resultTime = ref('00.000');

function goToPlay() {
  screen.value = 'play';
}

function goToResult(time: string) {
  resultTime.value = time;
  screen.value = 'result';
}

function goToTop() {
  screen.value = 'top';
}
</script>

<template>
  <main>
    <TopScreen v-if="screen === 'top'" @start="goToPlay" />
    <PlayScreen v-else-if="screen === 'play'" @stop="goToResult" />
    <ResultScreen v-else :time="resultTime" @restart="goToTop" />
  </main>
</template>
