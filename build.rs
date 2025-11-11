use std::{env, fs, path::Path};

fn main() {
    // 当 doc 目录变化时重新运行构建脚本
    println!("cargo:rerun-if-changed=public/doc");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("doc_files.rs");

    let mut files = Vec::new();

    // 读取 public/doc 目录
    if let Ok(entries) = fs::read_dir("public/doc") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".html") {
                    files.push(name.to_string());
                }
            }
        }
    }

    files.sort(); // 保持顺序一致

    // 生成包含文件列表的 Rust 代码
    let code = if files.is_empty() {
        "pub const DOC_FILES: &[&str] = &[];".to_string()
    } else {
        format!(
            "pub const DOC_FILES: &[&str] = &[{}];",
            files
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    fs::write(dest_path, code).unwrap();
}
