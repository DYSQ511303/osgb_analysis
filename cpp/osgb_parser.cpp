#include "osgb_parser.h"
#include <osgDB/ReadFile>
#include <osgDB/WriteFile>
#include <osg/Node>
#include <osg/Geode>
#include <osg/Geometry>
#include <sstream>
#include <string>
#include <vector>
#include <osg/PagedLOD>
#include <osg/NodeVisitor>

// 自定义Visitor统计三角面片
class TriangleCountVisitor : public osg::NodeVisitor {
public:
    unsigned int numTriangles = 0;

    TriangleCountVisitor() : osg::NodeVisitor(osg::NodeVisitor::TRAVERSE_ALL_CHILDREN) {}

    void apply(osg::Geode& geode) override {
        for (unsigned int i = 0; i < geode.getNumDrawables(); ++i) {
            osg::Geometry* geom = geode.getDrawable(i)->asGeometry();
            if (geom) {
                for (unsigned int j = 0; j < geom->getNumPrimitiveSets(); ++j) {
                    osg::PrimitiveSet* ps = geom->getPrimitiveSet(j);
                    if (ps->getMode() == osg::PrimitiveSet::TRIANGLES) {
                        numTriangles += ps->getNumIndices() / 3;
                    }
                }
            }
        }
        traverse(geode);
    }
};

const char* parse_osgb2(const char* filepath) {
    osg::ref_ptr<osg::Node> node = osgDB::readNodeFile(filepath);
    if (!node) {
        return strdup("{\"error\": \"Failed to load OSGB file\"}");
    }

    TriangleCountVisitor visitor;
    node->accept(visitor); // 递归统计三角面片

    std::ostringstream json;
    json << "{"
         
         << "\"nodeType\":\"" << node->className() << "\","
         << "\"filename\":\"" << filepath << "\","
         << "\"numTriangles\":" << visitor.numTriangles << ","
         << "\"success\": true}";

    return strdup(json.str().c_str());
}

// const char* parse_osgb(const char* filepath) {
//     // 读取 OSGB 文件
//     osg::ref_ptr<osg::Node> node = osgDB::readNodeFile(filepath);
//     if (!node) {
//         return strdup("{\"error\": \"Failed to load OSGB file\"}");
//     }

//     std::ostringstream json;
//     json << "{";
    
//     // 收集基本信息
//     json << "\"filename\":\"" << filepath << "\",";
//     json << "\"nodeType\":\"" << node->className() << "\",";
    
//     // 收集几何信息
//     osg::Geode* geode = node->asGeode();
//     if (geode) {
//         unsigned int numTriangles = 0;
//     for (unsigned int i = 0; i < geode->getNumDrawables(); ++i) {
//         osg::Geometry* geom = geode->getDrawable(i)->asGeometry();
//         if (geom) numTriangles += geom->getNumPrimitiveSets();
//     }
//     json << "\"numTriangles\":" << numTriangles << ",";
        
//         // 可以添加更多几何信息收集逻辑
//     }
    
//     json << "\"success\": true}";
    
//     return strdup(json.str().c_str());
// }

