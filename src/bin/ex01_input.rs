use std::io;

/// 範例 01: 基礎輸入輸出
/// 演示如何讀取使用者輸入並進行格式化輸出
fn main() {
    println!("=== 歡迎使用 Rust 互動範例 ===");
    println!("請輸入您的名字:");

    // 建立一個可變變數來儲存輸入
    // String::new() 建立一個空的字串
    let mut name = String::new();

    // 讀取標準輸入
    // read_line 會將內容追加到字串後
    // expect 處理潛在的錯誤（如果讀取失敗）
    io::stdin()
        .read_line(&mut name)
        .expect("讀取輸入失敗");

    // trim() 去除前後空白與換行符號
    let name = name.trim();

    println!("你好, {}! 歡迎來到 Rust 的世界。", name);
    println!("這是一個使用 println! 巨集 (Macro) 的範例。");
}
