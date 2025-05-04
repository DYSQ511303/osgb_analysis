#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// 解析 OSGB 文件并返回 JSON 格式的元数据
const char* parse_osgb(const char* filepath);

// 释放 C++ 分配的内存
void free_osgb_result(char* ptr);

#ifdef __cplusplus
}
#endif