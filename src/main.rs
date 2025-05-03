use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// 声明外部 C 函数
unsafe extern "C" {
    unsafe fn hello_from_cpp() -> *mut c_char;
}

fn main() {
    // 调用 C++ 函数
    let c_msg = unsafe { hello_from_cpp() };
    
    // 转换 C 字符串为 Rust 字符串
    let msg = unsafe { CStr::from_ptr(c_msg) }.to_string_lossy();
    println!("{}", msg);

    // 释放 C++ 分配的内存
    unsafe { libc::free(c_msg as *mut libc::c_void) };
}