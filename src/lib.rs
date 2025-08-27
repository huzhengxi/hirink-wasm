use wasm_bindgen::prelude::*;

// 模块声明
mod define;

// // 公开导出常量，供其他文件使用
use define::{AUTH_CODE_EXPIRED_CONTENT, AUTH_CODE_EXPIRES, AUTH_START_TIMESTAMP, IS_DEBUG};

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
    }};
}

// 定义内部方法。检查授权码是否过期，检查方法是获取当前时间戳并和授权码的过期时间进行比较。
fn check_auth_code_expired() -> bool {
    // 获取当前时间戳（毫秒）
    let now = js_sys::Date::now();
    // 转换为秒
    let seconds = (now / 1000.0) as u64;
    
    // 将当前时间戳保存到 localStorage
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            // last_time 存在的话转为数字
            let last_time = storage.get_item("cac").unwrap_or(None).map(|s| s.parse::<u64>().unwrap());
            if last_time.is_some() && last_time.unwrap() > seconds {
                return false;
            }
            let _ = storage.set_item("cac", &seconds.to_string());
        }
    }
    
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
    console_log!(
        "Hello, {}!，{}",
        name,
        if is_expired { "没过期" } else { "过期了" }
    );
}

// 当wasm模块被实例化时调用
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Hello from Rust and WebAssembly!");
}

// 获取控制实例类型
#[wasm_bindgen]
pub fn get_control_instance(device_type: Option<String>, ep_id: Option<i32>) -> Option<String> {
    // 检查 license 是否过期
    if !check_auth_code_expired() {
        return None;
    }
    // 检查参数是否为空
    let device_type = device_type?;
    let ep_id = ep_id?;

    // 智能空调网关（空调）- 特殊情况
    if device_type == "1342" {
        return Some("AirConditioner".to_string());
    }

    // 构建类型字符串
    let type_str = format!("{}-{}", device_type, ep_id);

    match type_str.as_str() {
        // 灯光控制
        "1108-1" | "1109-1" => Some("Light".to_string()), // 单色调光驱动 | 冷暖调光驱动
        "1110-1" => Some("FestoonLight".to_string()),     // 幻彩调光驱动

        // 窗帘控制
        "1304-1" => Some("FlatCurtain".to_string()), // 智能平开电机 (平开帘)
        "1305-1" => Some("RollCurtain".to_string()), // 智能卷帘电机 (卷帘)
        "1324-1" => Some("FancyCurtain".to_string()), // 智能梦幻帘电机 (梦幻帘)

        // 地暖
        "1308-2" | "1333-2" | "1311-2" | "1312-2" | "1341-2" => {
            Some("UnderfloorHeating".to_string()) // 强电地暖分机 | 智能温控系列
        }

        // 空调
        "1311-3" | "1312-3" | "1341-3" | "1306-1" | "1310-3" | "1331-3" | "1330-3" => {
            Some("AirConditioner".to_string()) // 智能温控系列 | VRV空调分机 | 风机温控系列
        }

        // 新风
        "1309-1" | "1332-1" | "1330-1" | "1311-1" | "1312-1" | "1341-1" => {
            Some("FreshAirVentilator".to_string()) // 强电新风分机 | 智能温控系列
        }

        _ => None, // 未匹配到的类型返回 None
    }
}

// 重载方法：直接传入类型字符串
#[wasm_bindgen]
pub fn get_control_instance_by_type(type_str: &str) -> Option<String> {
    // 检查 license 是否过期
    if !check_auth_code_expired() {
        return None;
    }
    //  特殊情况：智能空调网关 以 1342 开头
    if type_str.starts_with("1342") {
        return Some("AirConditioner".to_string());
    }
    match type_str {
        // 灯光控制
        "1108-1" | "1109-1" => Some("Light".to_string()),
        "1110-1" => Some("FestoonLight".to_string()),

        // 窗帘控制
        "1304-1" => Some("FlatCurtain".to_string()),
        "1305-1" => Some("RollCurtain".to_string()),
        "1324-1" => Some("FancyCurtain".to_string()),

        // 地暖
        "1308-2" | "1333-2" | "1311-2" | "1312-2" | "1341-2" => {
            Some("UnderfloorHeating".to_string())
        }

        // 空调
        "1311-3" | "1312-3" | "1341-3" | "1306-1" | "1310-3" | "1331-3" | "1330-3" => {
            Some("AirConditioner".to_string())
        }

        // 新风
        "1309-1" | "1332-1" | "1330-1" | "1311-1" | "1312-1" | "1341-1" => {
            Some("FreshAirVentilator".to_string())
        }

        _ => None,
    }
}
