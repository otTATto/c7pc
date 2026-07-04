mod db;

use std::sync::Mutex;
use std::time::Instant;

use db::{init_db, query_today_ranking, save_result as db_save_result, DbPath, RankingData};
use tauri::Manager;

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

/// 計測結果を SQLite に保存する Tauri コマンド
///
/// 保存した行の id を返す。DB 接続・書き込みに失敗した場合はエラー文字列を返す。
#[tauri::command]
fn save_result(stopped_ms: u64, db_path: tauri::State<DbPath>) -> Result<i64, String> {
    db_save_result(&db_path.0, stopped_ms).map_err(|e| e.to_string())
}

/// 今日のランキングを取得する Tauri コマンド
///
/// # Parameters
/// * `result_id` - 自分の結果として順位付けの対象にする行の id
#[tauri::command]
fn get_today_ranking(result_id: i64, db_path: tauri::State<DbPath>) -> Result<RankingData, String> {
    query_today_ranking(&db_path.0, result_id)
}

fn resolve_db_path(app: &tauri::App) -> Result<String, String> {
    let db_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to get app data dir: {e}"))?;
    std::fs::create_dir_all(&db_dir).map_err(|e| format!("failed to create app data dir: {e}"))?;
    Ok(db_dir.join("c7pc.db").to_string_lossy().into_owned())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(TimerState(Mutex::new(None)))
        .setup(|app| {
            let db_path = resolve_db_path(app)?;
            init_db(&db_path).map_err(|e| format!("failed to initialize database: {e}"))?;
            app.manage(DbPath(db_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_timer,
            get_elapsed_ms,
            stop_timer,
            save_result,
            get_today_ranking
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
