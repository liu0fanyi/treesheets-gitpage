use std::{env, fs, path::Path};

fn main() {
    // 当 doc 目录变化时重新运行构建脚本
    println!("cargo:rerun-if-changed=public/doc");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    
    // 生成文件名列表
    let files_dest_path = Path::new(&out_dir).join("doc_file_list.rs");
    
    // 生成文件名和内容列表
    let contents_dest_path = Path::new(&out_dir).join("doc_files.rs");

    let mut files = Vec::new();
    let mut file_contents = Vec::new();

    // 读取 public/doc 目录
    if let Ok(entries) = fs::read_dir("public/doc") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".html") {
                    files.push(name.to_string());
                    
                    // 读取文件内容
                    let file_path = entry.path();
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        // 使用 Rust 的字符串字面量转义
                        let content_escaped = content
                            .replace('\\', "\\\\")  // 先转义反斜杠
                            .replace('"', "\\\"")  // 再转义双引号
                            .replace('\n', "\\n")   // 转义换行符
                            .replace('\r', "\\r")   // 转义回车符
                            .replace('\t', "\\t");  // 转义制表符
                        file_contents.push((name.to_string(), content_escaped));
                    }
                }
            }
        }
    }

    files.sort(); // 保持顺序一致

    // 生成包含文件列表的 Rust 代码（只包含文件名）
    let files_code = if files.is_empty() {
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

    fs::write(files_dest_path, files_code).unwrap();

    // 生成包含文件内容的 Rust 代码（包含文件名和内容）
    let contents_code = if file_contents.is_empty() {
        "pub const DOC_CONTENTS: &[(&str, &str)] = &[];".to_string()
    } else {
        format!(
            "pub const DOC_CONTENTS: &[(&str, &str)] = &[{}];",
            file_contents
                .iter()
                .map(|(name, content)| format!("(\"{}\", \"{}\")", name, content))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    fs::write(contents_dest_path, contents_code).unwrap();
}