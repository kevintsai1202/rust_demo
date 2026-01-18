# 規格文件 - Rust 學習範例

## 1. 架構與選型
- **語言**: Rust
- **專案類型**: 學習範例集合 (CLI 與 Web API)
- **Web 框架**: Axum (用於 ex04_api)
  - 選擇原因: 效能高、生態系完整、與 Tokio 整合佳
- **非同步 Runtime**: Tokio
- **序列化**: Serde (JSON)

## 2. 資料模型
### User
- `id`: u64 (使用者唯一識別碼)
- `username`: String (使用者名稱)
- `email`: String (電子郵件)

## 3. 關鍵流程
1. **API 啟動**: 綁定 3000 port
2. **GET /**: 回傳歡迎訊息
3. **GET /users**: 回傳使用者列表 (In-memory 模擬)
4. **POST /users**: 建立新使用者 (回傳建立之資料)

## 4. 虛擬碼
```rust
// Main
fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user));
    
    bind("0.0.0.0:3000").serve(app).await;
}
```

## 5. 系統脈絡圖 (Context Diagram)
[User] -> [Rust API Server]

## 6. 容器/部署概觀
- 本機運行: `cargo run --bin ex04_api`

## 7. 模組關係圖
- `main` depends on `axum`, `tokio`, `serde`

## 8. 序列圖
(省略簡化)

## 9. ER 圖
User {
    u64 id PK
    string username
    string email
}

## 10. 類別圖
Struct User {
    +id: u64
    +username: String
    +email: String
}

## 11. 流程圖
Start -> Init Router -> Bind Port -> Await Request -> Handle Request -> Return Response

## 12. 狀態圖
Server: Check -> Running -> Stopped
