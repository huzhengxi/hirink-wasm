// 定义授权时间戳
pub const AUTH_START_TIMESTAMP: i64 = 1756319435;

// 1 小时
const ONE_HOUR: i64 = 60 * 60;
// 1 天
const ONE_DAY:i64 = ONE_HOUR * 7;

pub const AUTH_CODE_EXPIRES: i64 = 7 * ONE_DAY ;

// 定义提示内容 “授权已过期，请联系开发者 —— 185 0038 3830”

pub const AUTH_CODE_EXPIRED_CONTENT: &'static str = "授权已过期，请联系开发者 185 0038 3830";

pub  const  IS_DEBUG: bool = false;

pub const THEME_CONFIG: &str = include_str!("theme.json");
