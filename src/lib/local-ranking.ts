// Tauri 環境が無い場合（ブラウザでの開発時）に SQLite の代わりとして使う
// モジュールスコープの簡易ストア。リロードで消えてよい。

export interface RankingEntry {
  rank: number;
  stoppedMs: number;
  isMine: boolean;
}

export interface RankingData {
  top: RankingEntry[];
  mine: RankingEntry;
  total: number;
}

interface StoredResult {
  id: number;
  stoppedMs: number;
  recordedDate: string;
}

const results: StoredResult[] = [];
let nextId = 1;

function todayKey(): string {
  const now = new Date();
  const y = now.getFullYear();
  const m = String(now.getMonth() + 1).padStart(2, '0');
  const d = String(now.getDate()).padStart(2, '0');
  return `${y}-${m}-${d}`;
}

/** 結果をローカルストアに保存し、擬似 id を返す */
export function saveLocalResult(stoppedMs: number): number {
  const id = nextId++;
  results.push({ id, stoppedMs, recordedDate: todayKey() });
  return id;
}

const TARGET_MS = 7777;

/** ローカルストアから今日のランキングを算出する（get_today_ranking のブラウザ版フォールバック） */
export function getLocalTodayRanking(resultId: number): RankingData {
  const today = todayKey();
  const ordered = results
    .filter((r) => r.recordedDate === today)
    .slice()
    .sort((a, b) => {
      const diff = Math.abs(a.stoppedMs - TARGET_MS) - Math.abs(b.stoppedMs - TARGET_MS);
      if (diff !== 0) return diff;
      return a.id - b.id;
    });

  const total = ordered.length;
  const mineIndex = ordered.findIndex((r) => r.id === resultId);
  if (mineIndex === -1) {
    throw new Error('指定された結果が見つかりませんでした');
  }

  const top: RankingEntry[] = ordered.slice(0, 10).map((r, i) => ({
    rank: i + 1,
    stoppedMs: r.stoppedMs,
    isMine: r.id === resultId,
  }));

  const mine: RankingEntry = {
    rank: mineIndex + 1,
    stoppedMs: ordered[mineIndex].stoppedMs,
    isMine: true,
  };

  return { top, mine, total };
}
