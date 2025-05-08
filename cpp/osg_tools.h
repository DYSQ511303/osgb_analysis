#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// OSGB解析
const char* parse_osgb(const char* filepath);
void free_osgb_result(char* ptr);

// OSGT转换
int convert_osgb_to_osgt(const char* input, const char* output);

#ifdef __cplusplus
}
#endif