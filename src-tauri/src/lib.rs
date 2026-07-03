use std::sync::Mutex;
use std::time::Instant;

/// カウントアップタイマーの計測状態
///
/// 開始時刻を保持する。未開始の場合は `None`。
struct TimerState(Mutex<Option<Instant>>);

/// タイマーを開始する Tauri コマンド
///
/// 現在時刻を計測開始時刻として記録する。既に計測中の場合も
/// 上書きしてリスタートする。
#[tauri::command]
fn start_timer(state: tauri::State<TimerState>) {
    *state.0.lock().unwrap() = Some(Instant::now());
}

/// 計測開始からの経過ミリ秒を返す Tauri コマンド
///
/// 表示更新など目安の用途に使う。未開始の場合は 0 を返す。
#[tauri::command]
fn get_elapsed_ms(state: tauri::State<TimerState>) -> u64 {
    match *state.0.lock().unwrap() {
        Some(start) => start.elapsed().as_millis() as u64,
        None => 0,
    }
}

/// タイマーを停止する Tauri コマンド
///
/// コマンド実行時点の経過ミリ秒を公式記録として返し、状態を
/// 未開始（`None`）に戻す。未開始だった場合は 0 を返す。
#[tauri::command]
fn stop_timer(state: tauri::State<TimerState>) -> u64 {
    let mut guard = state.0.lock().unwrap();
    let elapsed = match *guard {
        Some(start) => start.elapsed().as_millis() as u64,
        None => 0,
    };
    *guard = None;
    elapsed
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(TimerState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            start_timer,
            get_elapsed_ms,
            stop_timer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
