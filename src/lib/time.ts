/** ねらう正解タイム（秒） */
export const CORRECT_TIME_SECONDS = 7.777;

/**
 * 経過ミリ秒を `SS.mmm` 形式の文字列に整形する
 *
 * 60 秒以上は繰り上げず、秒部分を 60 で割った余りを表示する
 * （このゲームでは 1 分未満のタイムのみを扱うための簡易表示）。
 */
export function formatTime(elapsedMs: number): string {
  const totalMs = Math.max(0, Math.floor(elapsedMs));
  const totalSeconds = Math.floor(totalMs / 1000);
  const s = String(totalSeconds % 60).padStart(2, '0');
  const ms = String(totalMs % 1000).padStart(3, '0');
  return `${s}.${ms}`;
}

/**
 * 経過ミリ秒と正解タイム（7.777 秒）との差を秒単位で計算する
 *
 * 浮動小数点誤差を避けるためミリ秒単位で丸めてから秒に変換する。
 * 正の値はねらいより遅い（押すのが遅かった）、負の値は早いことを表す。
 */
export function diffSeconds(ms: number): number {
  const sec = ms / 1000;
  return Math.round((sec - CORRECT_TIME_SECONDS) * 1000) / 1000;
}

/**
 * 差分（秒）を符号付きの文字列に整形する
 *
 * +0.1 秒を超えて遅い場合のみ `+` を付与する（ResultScreen の現行仕様に合わせる）。
 */
export function formatDiff(diff: number): string {
  return diff > 0.1 ? `+${diff}` : `${diff}`;
}
