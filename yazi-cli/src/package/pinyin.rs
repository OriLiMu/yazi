use std::path::Path;
use anyhow::Result;
use yazi_fs::pinyin_utils::{convert_dir_to_pinyin, save_pinyin_mappings};

pub fn handle_pinyin_command(dir: Option<&str>, output: &str) -> Result<()> {
    // 获取目录路径
    let dir_path = match dir {
        Some(path) => Path::new(path).to_path_buf(),
        None => std::env::current_dir()?,
    };

    // 将目录中的文件名转换为拼音格式
    let mappings = convert_dir_to_pinyin(&dir_path)?;

    // 将转换结果保存到文件
    save_pinyin_mappings(&mappings, Path::new(output))?;

    // 输出转换结果摘要
    println!("已将{}个文件/目录名转换为拼音格式，并保存到 {}", mappings.len(), output);

    Ok(())
} 