#include "hello.h"
#include <string>

const char* hello_from_cpp() {
    std::string msg = "LOVE YOU";
    return strdup(msg.c_str()); // 返回拷贝，Rust 需手动释放
}