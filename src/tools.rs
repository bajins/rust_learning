use std::{ffi::OsStr, iter::once, os::windows::prelude::OsStrExt};

/**
 * 宽字符转换: Windows使用的字符编码是宽字符，所以需要将源字符改成宽字符模式
 * 
 * @param text 需要转换字符
 * @return 宽字符指针
 */
pub fn text_wide(text: &str) -> *const u16 {
    let win_text: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    return win_text.as_ptr();
}