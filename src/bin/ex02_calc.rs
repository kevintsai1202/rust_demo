// use std::io;

/// 定義運算操作的列舉 (Enum)
/// Rust 的 Enum 可以攜帶資料，非常強大
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

/// 執行運算的函數
/// 演示模式匹配 (Pattern Matching)
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                println!("警告: 除數不能為零，回傳 0.0");
                0.0
            } else {
                a / b
            }
        }
    }
}

/// 範例 02: 簡易計算機
/// 演示 Enums, Pattern Matching 與基本數值運算
fn main() {
    println!("=== Rust 計算機範例 ===");
    
    // 這裡我們直接模擬一些操作，實際應用通常會結合解析輸入
    let ops = vec![
        Operation::Add(10.0, 5.0),
        Operation::Subtract(20.0, 7.5),
        Operation::Multiply(3.0, 4.0),
        Operation::Divide(10.0, 2.0),
        Operation::Divide(5.0, 0.0), // 測試除以零
    ];

    for (index, op) in ops.iter().enumerate() {
        let result = match op {
            Operation::Add(a, b) => (*a, *b, "+", calculate(Operation::Add(*a, *b))),
            Operation::Subtract(a, b) => (*a, *b, "-", calculate(Operation::Subtract(*a, *b))),
            Operation::Multiply(a, b) => (*a, *b, "*", calculate(Operation::Multiply(*a, *b))),
            Operation::Divide(a, b) => (*a, *b, "/", calculate(Operation::Divide(*a, *b))),
        };
        
        println!("運算 {}: {} {} {} = {}", index + 1, result.0, result.2, result.1, result.3);
    }
}
