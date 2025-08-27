use wasm_bindgen::prelude::*;

// 模块声明
mod define;

// // 公开导出常量，供其他文件使用
use define::{AUTH_START_TIMESTAMP, AUTH_CODE_EXPIRES, AUTH_CODE_EXPIRED_CONTENT, IS_DEBUG};

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


// 调用 window.alert
macro_rules! alert {
    ($msg:expr) => {{
        web_sys::window()
            .expect("no global `window` exists")
            .alert_with_message($msg)
            .expect("should have been able to alert");
    }}
}


// 定义内部方法。检查授权码是否过期，检查方法是获取当前时间戳并和授权码的过期时间进行比较。
fn check_auth_code_expired() -> bool {
    // 获取当前时间戳（毫秒）
    let now = js_sys::Date::now();
    // 转换为秒
    let seconds = (now / 1000.0) as u64;
    let expired_time = (AUTH_START_TIMESTAMP + AUTH_CODE_EXPIRES) as u64;
    let diff: i64 = (seconds - expired_time) as i64;
    if IS_DEBUG {
        console_log!("当前时间戳: {}, 过期时间戳：{}", seconds, expired_time);
        // 打印 diff，打印 u64 类型
        console_log!("diff: {}", diff);
    }

    if diff > 0 {
        alert!(AUTH_CODE_EXPIRED_CONTENT)
    }
    return diff < 0;
}

// 导出一个 `greet` 函数从 Rust 到 JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
    let is_expired = check_auth_code_expired();
    console_log!("Hello, {}!，{}", name, if is_expired { "没过期" } else { "过期了" });
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