use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, Schema, ConnectionTrait
};
use serde::Deserialize;
use std::net::SocketAddr;

// 引入 Entity 定義
// 在 main.rs 或 lib.rs 需要宣告 mod entity;
#[path = "../entity.rs"]
mod entity;
use entity::ActiveModel as PostActiveModel;
use entity::Entity as Post;

const DB_URL: &str = "sqlite://posts.db?mode=rwc";

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    text: String,
}

#[derive(Deserialize)]
struct UpdatePost {
    title: String,
    text: String,
}

/// 範例 07: 使用 SeaORM 的 CRUD
/// SeaORM 是 Rust 中最熱門的非同步 ORM，支援 SQLx
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 建立資料庫連線
    let conn = Database::connect(DB_URL).await?;
    println!("Database connected: {}", DB_URL);

    // 2. 建立資料表 (Migration)
    // 實務上通常使用 `sea-orm-cli` 進行 migration
    // 這裡為了範例方便，使用 Schema Helper 動態建立
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    let create_table_stmt = schema.create_table_from_entity(Post);
    
    // 執行 Create Table
    // 注意: create_table_from_entity 產生的是 TableCreateStatement
    // 我們需要確保資料表不存在才建立 (sqlite 支援 IF NOT EXISTS)
    // SeaORM 的 create_table_from_entity 預設不包含 IF NOT EXISTS，需手動處理或直接執行
    // 這裡簡單嘗試執行，忽略錯誤 (如果已存在)
    let _ = conn.execute(builder.build(&create_table_stmt)).await;
    println!("Table schema initialized (if not existed).");

    let state = AppState { conn };

    // 3. 建立路由
    let app = Router::new()
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/{id}", get(get_post).put(update_post).delete(delete_post))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("SeaORM API Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// --- Handlers ---

/// 列出所有文章
async fn list_posts(State(state): State<AppState>) -> impl IntoResponse {
    // 使用 Entity::find() 查詢所有
    let posts = Post::find().all(&state.conn).await;

    match posts {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 取得單一文章
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let post = Post::find_by_id(id).one(&state.conn).await;

    match post {
        Ok(Some(post)) => (StatusCode::OK, Json(post)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 建立文章
async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    // 建立 ActiveModel
    let new_post = PostActiveModel {
        title: ActiveValue::Set(payload.title),
        text: ActiveValue::Set(payload.text),
        ..Default::default() // ID 會自動生成 (NotSet)
    };

    let result = new_post.insert(&state.conn).await;

    match result {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 更新文章
async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePost>,
) -> impl IntoResponse {
    // 先查詢是否存在
    let post = Post::find_by_id(id).one(&state.conn).await;

    match post {
        Ok(Some(post_model)) => {
            // 轉換為 ActiveModel 進行修改
            let mut active_model: PostActiveModel = post_model.into();
            active_model.title = ActiveValue::Set(payload.title);
            active_model.text = ActiveValue::Set(payload.text);

            let result = active_model.update(&state.conn).await;
            match result {
                Ok(updated_post) => (StatusCode::OK, Json(updated_post)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// 刪除文章
async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = Post::delete_by_id(id).exec(&state.conn).await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected == 0 {
                 (StatusCode::NOT_FOUND, "Post not found").into_response()
            } else {
                 (StatusCode::OK, "Post deleted").into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
