use rusqlite::{Connection, Result, params};
use std::sync::Mutex;
use crate::models::UrlEntry;

pub type DbPool = Mutex<Connection>;

/// Создаёт подключение и таблицу
pub fn init_db() -> Result<DbPool> {
    let conn = Connection::open("urls.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            code        TEXT NOT NULL UNIQUE,
            original_url TEXT NOT NULL,
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            clicks      INTEGER DEFAULT 0
        )",
        [],
    )?;
    
    Ok(Mutex::new(conn))
}

/// Сохраняет новую ссылку
pub fn insert_url(conn: &Connection, code: &str, url: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO urls (code, original_url) VALUES (?1, ?2)",
        params![code, url],
    )?;
    Ok(())
}

/// Ищет ссылку по коду и увеличивает счётчик
pub fn get_url_by_code(conn: &Connection, code: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare(
        "SELECT original_url FROM urls WHERE code = ?1"
    )?;
    
    let result: Result<String> = stmt.query_row(params![code], |row| row.get(0));
    
    match result {
        Ok(url) => {
            // Увеличиваем счётчик кликов
            conn.execute(
                "UPDATE urls SET clicks = clicks + 1 WHERE code = ?1",
                params![code],
            )?;
            Ok(Some(url))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Проверяет, существует ли код
pub fn code_exists(conn: &Connection, code: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM urls WHERE code = ?1",
        params![code],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Получает статистику по ссылке
pub fn get_stats(conn: &Connection, code: &str) -> Result<Option<UrlEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, code, original_url, created_at, clicks FROM urls WHERE code = ?1"
    )?;
    
    let result = stmt.query_row(params![code], |row| {
        Ok(UrlEntry {
            id: row.get(0)?,
            code: row.get(1)?,
            original_url: row.get(2)?,
            created_at: row.get(3)?,
            clicks: row.get(4)?,
        })
    });
    
    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}