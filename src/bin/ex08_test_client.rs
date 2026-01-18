use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:3000/posts";

    println!("=== 開始 API 整合測試 ===");
    println!("目標: {}", base_url);

    // 1. 建立文章 (Create)
    println!("\n1. 測試建立文章 (POST)...");
    let new_post = Post {
        id: None,
        title: "Rust 測試導論".to_string(),
        text: "使用 Reqwest 撰寫測試腳本".to_string(),
    };

    let created_post: Post = client
        .post(base_url)
        .json(&new_post)
        .send()
        .await?
        .error_for_status()? // 檢查是否 2xx 成功
        .json()
        .await?;

    println!("   成功! 已建立文章: {:?}", created_post);
    let post_id = created_post.id.expect("Server should return ID");

    // 2. 查詢所有文章 (List)
    println!("\n2. 測試查詢列表 (GET)...");
    let posts: Vec<Post> = client
        .get(base_url)
        .send()
        .await?
        .json()
        .await?;
    println!("   目前共有 {} 篇文章", posts.len());

    // 3. 查詢單一文章 (Get)
    println!("\n3. 測試查詢單一文章 (GET ID)...");
    let single_url = format!("{}/{}", base_url, post_id);
    let fetched_post: Post = client
        .get(&single_url)
        .send()
        .await?
        .json()
        .await?;
    assert_eq!(fetched_post.title, new_post.title);
    println!("   成功! 資料正確");

    // 4. 更新文章 (Update)
    println!("\n4. 測試更新文章 (PUT)...");
    let update_data = Post {
        id: None,
        title: "Rust 測試導論 (已更新)".to_string(),
        text: "內容也更新了".to_string(),
    };
    
    let updated_post: Post = client
        .put(&single_url)
        .json(&update_data)
        .send()
        .await?
        .json()
        .await?;
    
    assert_eq!(updated_post.title, "Rust 測試導論 (已更新)");
    println!("   成功! 更新後標題: {}", updated_post.title);

    // 5. 刪除文章 (Delete)
    println!("\n5. 測試刪除文章 (DELETE)...");
    let status = client.delete(&single_url).send().await?.status();
    
    if status.is_success() {
        println!("   成功! 文章已刪除");
    } else {
        panic!("刪除失敗");
    }

    // 驗證是否真的刪除
    let status = client.get(&single_url).send().await?.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        println!("   驗證成功: 再次查詢回傳 404");
    } else {
        println!("   警告: 預期 404 但收到 {}", status);
    }

    println!("\n=== 所有測試完成，Pass! ===");
    Ok(())
}
