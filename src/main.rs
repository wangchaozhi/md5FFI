use std::ffi::{CStr, CString};
use std::time::Instant;
use libc::{c_char, c_int};

// 声明C函数
#[link(name = "md5utils")] // 不带扩展名，如 .dll 或 .lib
extern "C" {
    fn calculate_md5_from_array(filenames: *const *const c_char, count: c_int, result_buffer: *mut c_char) -> c_int;
}

fn main() {
    // 记录开始时间
    let start = Instant::now();
    // println!("cargo:rustc-link-lib=dylib=md5utils"); // 不带前缀和扩展名
    // println!("cargo:rustc-link-search=native=E:/RustroverProjects/md5FFI"); // DLL 文件所在路径

    // 要计算MD5的文件路径
    let file_paths = vec!["E:\\OneDrive\\图片\\Untitled_1.1.1.png"];
    let count = file_paths.len() as c_int;

    // 将Rust字符串转换为C字符串
    let c_file_paths: Vec<CString> = file_paths
        .iter()
        .map(|&path| CString::new(path).expect("CString::new failed"))
        .collect();

    // 获取C字符串指针数组
    let c_file_paths_ptrs: Vec<*const c_char> = c_file_paths
        .iter()
        .map(|cstr| cstr.as_ptr())
        .collect();

    // 为结果分配缓冲区
    let result_buffer_size = 32 * count; // 假设每个MD5哈希为32个字符（16字节的十六进制）
    let mut result_buffer = vec![0 as c_char; (result_buffer_size + 1) as usize]; // +1 是为了确保有结尾的 `\0`

    // 调用C函数
    let result = unsafe {
        calculate_md5_from_array(c_file_paths_ptrs.as_ptr(), count, result_buffer.as_mut_ptr())
    };

    // 检查结果
    if result == 0 {
        // 输出结果
        let result_cstr = unsafe { CStr::from_ptr(result_buffer.as_ptr()) };
        println!("MD5 Calculation Successful: {}", result_cstr.to_str().unwrap());
    } else {
        println!("MD5 Calculation Failed");
    }
    // 记录结束时间并计算时间差
    let duration = start.elapsed();
    println!("计算 MD5 所需时间: {} 毫秒", duration.as_millis());
}
