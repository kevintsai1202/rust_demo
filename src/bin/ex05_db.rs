use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<String>,
}

/// 範例 05: 使用 Rusqlite 操作內嵌 SQLite 資料庫
/// 演示: 建立資料表、插入資料、查詢資料
fn main() -> Result<()> {
    // 建立連線 (使用記憶體資料庫作為練習，若要存檔可改用檔案路徑)
    let conn = Connection::open_in_memory()?;
    
    // 建立 Person 資料表
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  TEXT
        )",
        (), // empty parameters
    )?;

    // 插入資料
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: Some("Rust Developer".to_string()),
    };
    
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    // 插入另一筆
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        ("Alice", "Data Analyst"),
    )?;

    println!("資料已插入。");

    // 查詢資料
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    println!("--- 查詢結果 ---");
    for person in person_iter {
        println!("找到人員: {:?}", person.unwrap());
    }

    Ok(())
}
