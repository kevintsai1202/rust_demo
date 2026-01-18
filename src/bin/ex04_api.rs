use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// 使用者資料模型
/// Derive 巨集自動實作序列化與反序列化
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
}

/// 建立使用者的請求 Payload
/// 不需要 id，因為由伺服器生成
#[derive(Debug, Deserialize)]
struct CreateUserPayload {
    username: String,
    email: String,
}

/// 範例 04: 簡易 RESTful API
/// 使用 Axum 框架
#[tokio::main]
async fn main() {
    // 建立路由
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", get(doc_get_users).post(doc_create_user));

    // 定義監聽位址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API 伺服器啟動於 http://{}", addr);

    // 啟動伺服器
    // axum 0.8+ 使用 serve 方式略有不同，這裡假設使用最新版或相容舊版寫法
    // 根據 cargo add 結果，我們應該檢查 axum 版本。
    // 如果是 0.7.x:
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// 根路徑處理器
async fn root_handler() -> &'static str {
    "Hello, Rust API!"
}

/// GET /users 處理器 (回傳範例資料)
async fn doc_get_users() -> impl IntoResponse {
    // 模擬資料
    let users = vec![
        User {
            id: 1,
            username: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            username: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    // Json 包裝器會自動將 Struct 轉為 JSON 回傳
    (StatusCode::OK, Json(users))
}

/// POST /users 處理器
/// Json<CreateUserPayload> 會自動解析 Request Body
async fn doc_create_user(Json(payload): Json<CreateUserPayload>) -> impl IntoResponse {
    // 這裡單純回傳一個模擬建立成功的 User
    let user = User {
        id: 1337,
        username: payload.username,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(user))
}
