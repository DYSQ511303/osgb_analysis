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
           .compiler(cl_path)
           .file("cpp/hello.cpp")
           .flag("/EHsc")
           .compile("main");
    } else {
        // 对于非 Windows 系统，可以添加其他编译器的配置，这里简单报错
        panic!("This build script is only configured for Windows with MSVC.");
    }
}