use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

pub struct DbPath(pub String);

/// ランキング上の 1 件分のエントリ
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RankingEntry {
    pub rank: usize,
    pub stopped_ms: i64,
    pub is_mine: bool,
}

/// `get_today_ranking` の戻り値
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingData {
    pub top: Vec<RankingEntry>,
    pub mine: RankingEntry,
    pub total: usize,
}

/// SQLite データベースを初期化する
///
/// `db_path` が存在しない場合はファイルを作成し、`results` テーブルを作成する。
/// 既にテーブルが存在する場合は何もしない。
pub fn init_db(db_path: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS results (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            stopped_ms    INTEGER NOT NULL,
            recorded_at   TEXT    NOT NULL,
            recorded_date TEXT    NOT NULL
        );",
    )
}

/// 計測結果を SQLite に保存し、挿入した行の id を返す
///
/// `recorded_at` は `YYYY-MM-DD HH:MM:SS`、`recorded_date` は `YYYY-MM-DD` で
/// いずれもローカル時刻から算出する。
pub fn save_result(db_path: &str, stopped_ms: u64) -> rusqlite::Result<i64> {
    let conn = Connection::open(db_path)?;
    let now = Local::now();
    let recorded_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let recorded_date = now.format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO results (stopped_ms, recorded_at, recorded_date) VALUES (?1, ?2, ?3)",
        params![stopped_ms as i64, recorded_at, recorded_date],
    )?;
    Ok(conn.last_insert_rowid())
}

/// 今日（ローカル日付）の全結果を対象に、指定した `result_id` の順位を含む
/// ランキングデータを取得する
///
/// 順位付けは `|stopped_ms - 7777|` の昇順、同値の場合は `id` の昇順とする。
///
/// # Errors
/// DB 接続・クエリに失敗した場合、または `result_id` が今日の結果の中に
/// 見つからない場合にエラーを返す。
pub fn query_today_ranking(db_path: &str, result_id: i64) -> Result<RankingData, String> {
    const TARGET_MS: i64 = 7777;

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut stmt = conn
        .prepare(
            "SELECT id, stopped_ms FROM results
             WHERE recorded_date = ?1
             ORDER BY ABS(stopped_ms - ?2) ASC, id ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![today, TARGET_MS], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
        })
        .map_err(|e| e.to_string())?;

    let mut ordered: Vec<(i64, i64)> = Vec::new();
    for row in rows {
        ordered.push(row.map_err(|e| e.to_string())?);
    }

    let total = ordered.len();

    let mine_index = ordered
        .iter()
        .position(|(id, _)| *id == result_id)
        .ok_or_else(|| "指定された結果が見つかりませんでした".to_string())?;

    let top = ordered
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, (id, stopped_ms))| RankingEntry {
            rank: i + 1,
            stopped_ms: *stopped_ms,
            is_mine: *id == result_id,
        })
        .collect();

    let mine = RankingEntry {
        rank: mine_index + 1,
        stopped_ms: ordered[mine_index].1,
        is_mine: true,
    };

    Ok(RankingData { top, mine, total })
}
