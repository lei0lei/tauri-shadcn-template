use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    tauri_build::build();
    
    // 获取当前路径，base_path 是 `src` 目录的路径
    let base_path = env::var("CARGO_MANIFEST_DIR").unwrap();

    // 设置头文件的相对路径
    let CF_ParameterDefine = PathBuf::from(&base_path).join("assets/external/include/CF_ParameterDefine.h");
    let CF_UserInterface = PathBuf::from(&base_path).join("assets/external/include/CF_UserInterface.h");
    let TypeDefine = PathBuf::from(&base_path).join("assets/external/include/TypeDefine.h");
    
    // 使用 bindgen 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .header(TypeDefine.to_str().unwrap())
        .header(CF_ParameterDefine.to_str().unwrap())
        .header(CF_UserInterface.to_str().unwrap())
        .clang_arg("-include")
        .clang_arg("stdbool.h")  // 添加这一行来包含 stdbool.h
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
   
    // 获取 OUT_DIR 环境变量（由 Cargo 自动设置）
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);  // 使用引用而不是移动
    println!("OUT_DIR: {}", out_dir);

    
    bindings
        .write_to_file(PathBuf::from(&base_path).join("src").join("sensors").join("cf3000_bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("BASE_PATH: {}", base_path);


    // 设置库文件的路径
    // println!("cargo:rustc-link-args=-Wl,-Bdynamic");  // 强制动态链接
    // println!("cargo:rustc-link-lib=static=hps_cfxxx_sdk");  // 如果是动态库（DLL），使用 dylib
    println!("cargo:rustc-link-search=native={}/assets/external/bin", base_path);  // 设置 .lib 和 .dll 的目录
    println!("cargo:rustc-link-lib=static=hps_cfxxxx_sdk");
    println!("cargo:rustc-link-lib=dylib=hps_cfxxxx_sdk");  // 如果是动态库（DLL），使用 dylib
    // println!("cargo:rustc-link-lib=dylib=cf_windows_3156557");  // 如果是动态库（DLL），使用 dylib


    // 设置库的查找路径（运行时动态库的查找路径）
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-env=PATH={}/assets/external/bin;{}", base_path, std::env::var("PATH").unwrap());
    
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-env=LD_LIBRARY_PATH={}/external/bin", base_path); // 对 Linux 设置动态库路径
    }

    // 重新编译时，当头文件发生变化时
    println!("cargo:rerun-if-changed={}", CF_ParameterDefine.display());
    println!("cargo:rerun-if-changed={}", CF_UserInterface.display());
    println!("cargo:rerun-if-changed={}", TypeDefine.display());


   }