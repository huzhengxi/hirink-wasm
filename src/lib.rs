use wasm_bindgen::prelude::*;

// 导入 `console.log` 函数从 `web-sys` crate
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 定义一个宏来简化控制台日志记录
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// 导出一个 `greet` 函数从 Rust 到 JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}!", name);
}

// 添加两个数字的函数
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 斐波那契数列函数
#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// 当wasm模块被实例化时调用
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Hello from Rust and WebAssembly!");
}

// license 校验
#[wasm_bindgen]
pub fn license_check(license_key: &str) -> bool {
    // 假设这里是一个真实的验证逻辑
    license_key == "valid_license_key"
        
}