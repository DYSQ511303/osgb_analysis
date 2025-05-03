#pragma once

#ifdef __cplusplus
extern "C" {  // 确保 C 语言兼容的符号命名
#endif

// 声明一个可被 Rust 调用的函数
const char* hello_from_cpp();

#ifdef __cplusplus
}
#endif