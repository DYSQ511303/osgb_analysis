use std::ffi::{CString, CStr};
use std::os::raw::c_char;

// OSGB解析相关FFI
unsafe extern "C" {
    unsafe fn parse_osgb(filepath: *const c_char) -> *mut c_char;
    unsafe fn free_osgb_result(ptr: *mut c_char);
    
    // 新增的OSGT转换函数
    unsafe fn convert_osgb_to_osgt(input: *const c_char, output: *const c_char) -> i32;
}

/// 安全封装OSGB解析函数
pub fn parse_osgb_file(input_path: &str) -> Result<String, String> {
    let c_input = CString::new(input_path).map_err(|e| e.to_string())?;
    
    unsafe {
        let c_result = parse_osgb(c_input.as_ptr());
        let result = CStr::from_ptr(c_result).to_string_lossy().into_owned();
        free_osgb_result(c_result);
        
        if result.contains("\"error\"") {
            Err(result)
        } else {
            Ok(result)
        }
    }
}

/// 安全封装OSGT转换函数
pub fn convert_to_osgt(input_path: &str, output_path: &str) -> Result<(), String> {
    let c_input = CString::new(input_path).map_err(|e| e.to_string())?;
    let c_output = CString::new(output_path).map_err(|e| e.to_string())?;

    match unsafe { convert_osgb_to_osgt(c_input.as_ptr(), c_output.as_ptr()) } {
        0 => Ok(()),
        1 => Err(format!("Failed to read OSGB file: {}", input_path)),
        2 => Err(format!("Failed to write OSGT file: {}", output_path)),
        _ => Err("Unknown conversion error".to_string()),
    }
}