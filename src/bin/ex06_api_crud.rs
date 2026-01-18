use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

/// 資料庫檔案名稱
const DB_FILE: &str = "my_database.db";

/// 使用者資料模型
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: i64,
    username: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserPayload {
    username: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserPayload {
    username: String,
    email: String,
}

/// 應用程式狀態，包含資料庫連線
/// 由於 Connection 不是 Thread-safe，且 Rusqlite 建議每個 Request 建立連線或使用 Connection Pool
/// 這裡為了教學簡單，演示 "Connection Pool" 的概念
/// 實務上建議使用 `r2d2` 或 `deadpool` 搭配 `rusqlite`
struct AppState {
    // 這裡我們簡化處理：每次操作都打開一個新連線，或者用 Mutex 保護單一連線
    // 但 SQLite 檔案式資料庫在多執行緒下，最佳實踐是使用連線池
    // 為了範例簡單，我們直接在 Handler 中開啟連線，不透過 State 傳遞 Connection
    db_path: String,
}

/// 範例 06: RESTful API + SQLite CRUD
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 初始化資料庫
    init_db(DB_FILE)?;

    // 2. 共享狀態
    let shared_state = Arc::new(AppState {
        db_path: DB_FILE.to_string(),
    });

    // 3. 建立路由
    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(shared_state);

    // 4. 啟動伺服器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API Server running at http://{}", addr);
    println!("Database file: {}", DB_FILE);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// 初始化資料庫表格
fn init_db(path: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id       INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email    TEXT NOT NULL
        )",
        (),
    )?;
    println!("資料庫初始化完成。");
    Ok(())
}

// 輔助函式：取得資料庫連線
fn get_conn(state: &AppState) -> rusqlite::Result<Connection> {
    Connection::open(&state.db_path)
}

// --- Handlers ---

/// 取得所有使用者
async fn list_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match get_conn(&state) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare("SELECT id, username, email FROM users") {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
        })
    });

    match user_iter {
        Ok(iter) => {
            let users: Vec<User> = iter.filter_map(Result::ok).collect();
            (StatusCode::OK, Json(users)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 取得單一使用者
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = match get_conn(&state) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // query_row 回傳單筆資料，若無資料會回傳 Error::QueryReturnedNoRows
    let user = conn.query_row(
        "SELECT id, username, email FROM users WHERE id = ?1",
        params![id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            })
        },
    );

    match user {
        Ok(u) => (StatusCode::OK, Json(u)).into_response(),
        Err(rusqlite::Error::QueryReturnedNoRows) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 建立使用者
async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let conn = match get_conn(&state) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // 執行插入，並取得自動生成的 ID
    let result = conn.execute(
        "INSERT INTO users (username, email) VALUES (?1, ?2)",
        params![payload.username, payload.email],
    );

    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            let new_user = User {
                id,
                username: payload.username,
                email: payload.email,
            };
            (StatusCode::CREATED, Json(new_user)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 更新使用者
async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
    let conn = match get_conn(&state) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let result = conn.execute(
        "UPDATE users SET username = ?1, email = ?2 WHERE id = ?3",
        params![payload.username, payload.email, id],
    );

    match result {
        Ok(affected) => {
            if affected == 0 {
                (StatusCode::NOT_FOUND, "User not found").into_response()
            } else {
                let updated_user = User {
                    id,
                    username: payload.username,
                    email: payload.email,
                };
                (StatusCode::OK, Json(updated_user)).into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 刪除使用者
async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = match get_conn(&state) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let result = conn.execute("DELETE FROM users WHERE id = ?1", params![id]);

    match result {
        Ok(affected) => {
             if affected == 0 {
                (StatusCode::NOT_FOUND, "User not found").into_response()
            } else {
                (StatusCode::OK, "User deleted").into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
