use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use clap::{App, Arg};


// 声明外部 C 函数
unsafe extern "C" {
    unsafe fn parse_osgb(filepath: *const c_char) -> *mut c_char;
    unsafe fn free_osgb_result(ptr: *mut c_char);
}

fn main() {
    let matches = App::new("OSGB Parser")
        .version("0.1.0")
        .about("Parses OSGB files and outputs their structure")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input OSGB file")
            .required(true)
            .index(1))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let c_input = CString::new(input_file).unwrap();

    // 调用 C++ 解析函数
    let c_result = unsafe { parse_osgb(c_input.as_ptr()) };
    
    // 转换结果并输出
    let result = unsafe { CStr::from_ptr(c_result) }.to_string_lossy();
    println!("{}", result);

    // 释放内存
    unsafe { free_osgb_result(c_result) };
}