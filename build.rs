extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // 获取环境变量中的 CARGO_CFG_TARGET_OS，用于判断操作系统
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| {
        panic!("Failed to get the target operating system information.");
    });

    if target_os == "windows" {
        // 获取 Visual Studio 安装目录
        let vc_install_dir = env::var("VCINSTALLDIR").unwrap_or_else(|_| {
            panic!("VCINSTALLDIR environment variable is not set. Please make sure Visual Studio is installed and configured correctly.");
        });

        // 构建 cl.exe 的路径
        let cl_path = PathBuf::from(vc_install_dir)
           .join(r"Tools\MSVC")
           .read_dir()
           .expect("Failed to read MSVC directory.")
           .filter_map(|entry| entry.ok())
           .map(|entry| entry.path())
           .filter(|path| path.is_dir())
           .max()
           .expect("No valid MSVC version found.")
           .join(r"bin\Hostx64\x64\cl.exe");

        // 检查 cl.exe 是否存在
        if!cl_path.exists() {
            panic!("cl.exe not found at the expected path: {:?}", cl_path);
        }

        // 编译 C++ 代码
        cc::Build::new()
            .cpp(true)
            .flag("-Zi")
            .flag("-INCREMENTAL")
            .flag("-bigobj")
            .warnings(false)
            .define("WIN32", None)
            .define("_WINDOWS", None)
            .include("cpp")
            .include("./src")
            .include("./vcpkg/installed/x64-windows/include")
            .include("./vcpkg/installed/x64-windows-release/include")
           //include将项目的 src 目录添加到编译器的包含路径中，这样编译器在查找 头文件 时会搜索该目录
           //将 vcpkg 安装的库的头文件目录添加到编译器的包含路径中，确保编译器能够找到所需的头文件
           .compiler(cl_path)
           .file("cpp/osgb_parser.cpp")
           .flag("/EHsc")
           .compile("main");
        println!("cargo:rustc-link-search=native=./vcpkg/installed/x64-windows-release/lib");
        //告诉 Rust 编译器在 ./vcpkg/installed/x64-windows-release/lib 目录中搜索链接库文件
        println!("cargo:rustc-link-lib=gdal");
        println!("cargo:rustc-link-lib=osg");
        println!("cargo:rustc-link-lib=osgDB");
        println!("cargo:rustc-link-lib=osgUtil");
        println!("cargo:rustc-link-lib=osgViewer");
        println!("cargo:rustc-link-lib=OpenThreads");
        //告诉 Rust 编译器链接指定的库文件，这里链接了 gdal、osg、osgDB、osgUtil、osgViewer 和 OpenThreads 等库
        // 告诉 Rust 运行时需要包含插件目录
        println!("cargo:rustc-env=OSG_LIBRARY_PATH=D:/studytrain/3d/3dtiles-master/3dtiles/vcpkg/installed/x64-windows-release/plugins/osgPlugins-3.6.5");
        println!("cargo:rustc-env=PATH={};D:/studytrain/3d/3dtiles-master/3dtiles/vcpkg/installed/x64-windows-release/bin", env::var("PATH").unwrap());
    } else {
        // 对于非 Windows 系统，可以添加其他编译器的配置，这里简单报错
        panic!("This build script is only configured for Windows with MSVC.");
    }
} 