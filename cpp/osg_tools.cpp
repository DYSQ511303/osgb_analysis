#include "osg_tools.h"
#include <osgDB/ReadFile>
#include <osgDB/WriteFile>
#include <osgDB/Registry>
#include <osg/Node>
#include <osg/Geode>
#include <osg/Geometry>
#include <sstream>
#include <string>
#include <vector>
#include <cstdlib>

// OSGB解析实现
const char* parse_osgb(const char* filepath) {
    // 读取 OSGB 文件
    osg::ref_ptr<osg::Node> node = osgDB::readNodeFile(filepath);
    if (!node) {
        return strdup("{\"error\": \"Failed to load OSGB file\"}");
    }

    std::ostringstream json;
    json << "{";
    
    // 收集基本信息
    json << "\"filename\":\"" << filepath << "\",";
    json << "\"nodeType\":\"" << node->className() << "\",";
    
    // 收集几何信息
    osg::Geode* geode = node->asGeode();
    if (geode) {
        unsigned int numDrawables = geode->getNumDrawables();
        json << "\"numDrawables\":" << numDrawables << ",";
        
        // 可以添加更多几何信息收集逻辑
    }
    
    json << "\"success\": true}";
    
    return strdup(json.str().c_str());
}

void free_osgb_result(char* ptr) {
    free(ptr);
}

// OSGT转换实现
int convert_osgb_to_osgt(const char* input_path, const char* output_path) {
    // 确保插件路径已设置（根据实际路径调整）
    std::string plugin_path = "D:/studytrain/3d/3dtiles-master/3dtiles/vcpkg/installed/x64-windows-release/plugins/osgPlugins-3.6.5";
    osgDB::Registry::instance()->setLibraryFilePathList(plugin_path);
    
    osg::ref_ptr<osg::Node> node = osgDB::readNodeFile(input_path);
    if (!node) {
        return 1; // 文件读取失败
    }

    if (!osgDB::writeNodeFile(*node, output_path)) {
        return 2; // 文件写入失败
    }

    return 0;
}