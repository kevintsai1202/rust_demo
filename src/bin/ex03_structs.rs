/// 一個代表待辦事項的結構體 (Struct)
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

impl TodoItem {
    /// 建構子：建立新的 TodoItem
    fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    /// 完成事項
    fn complete(&mut self) {
        self.completed = true;
    }
}

/// 管理 Todo 列表的結構體
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) {
        let id = (self.items.len() as u32) + 1;
        let item = TodoItem::new(id, title);
        self.items.push(item);
    }

    fn list_items(&self) {
        println!("--- 待辦事項列表 ---");
        if self.items.is_empty() {
            println!("(目前沒有事項)");
            return;
        }
        for item in &self.items {
            let status = if item.completed { "[x]" } else { "[ ]" };
            println!("{} {}. {}", status, item.id, item.title);
        }
        println!("--------------------");
    }

    fn complete_item(&mut self, id: u32) {
        // 使用迭代器查找並修改
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.complete();
            println!("已完成事項: {}", item.title);
        } else {
            println!("找不到 ID 為 {} 的事項", id);
        }
    }
}

/// 範例 03: 待辦事項清單
/// 演示 Struct, Impl, Vector 集合與所有權 (Ownership) 概念
fn main() {
    let mut todo_list = TodoList::new();

    // 新增事項
    todo_list.add_item(String::from("學習 Rust 基礎語法"));
    todo_list.add_item(String::from("練習 Struct 與 Impl"));
    todo_list.add_item(String::from("撰寫單元測試"));

    // 顯示列表
    todo_list.list_items();

    // 完成第一項
    todo_list.complete_item(1);

    // 再次顯示
    todo_list.list_items();
}
