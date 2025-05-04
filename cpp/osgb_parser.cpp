#include "osgb_parser.h"
#include <osgDB/ReadFile>
#include <osgDB/WriteFile>
#include <osg/Node>
#include <osg/Geode>
#include <osg/Geometry>
#include <sstream>
#include <string>
#include <vector>

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