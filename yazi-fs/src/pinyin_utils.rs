use std::path::{Path, PathBuf};
use pinyin::ToPinyin;

/// 将文件名中的中文字符转换为拼音首字母(使用专业拼音库)
pub fn to_pinyin_initials(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    
    // 使用pinyin库处理每个字符
    for c in text.chars() {
        if c as u32 >= 0x4E00 && c as u32 <= 0x9FFF {
            // 对中文字符，获取其拼音首字母
            if let Some(py) = c.to_pinyin() {
                let first_letter = py.plain().chars().next().unwrap_or('x');
                result.push(first_letter);
            } else {
                // 如果无法找到拼音，使用默认字母'x'
                result.push('x');
            }
        } else {
            // 非中文字符保持不变
            result.push(c);
        }
    }
    
    result
}

/// 转换路径中包含中文的文件名为拼音格式(只取首字母)
pub fn convert_path_to_pinyin(path: &Path) -> PathBuf {
    let parent = path.parent();
    
    if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
        let pinyin_name = to_pinyin_initials(file_name);
        
        if let Some(parent_path) = parent {
            parent_path.join(pinyin_name)
        } else {
            PathBuf::from(pinyin_name)
        }
    } else {
        path.to_path_buf()
    }
}

/// 转换目录中所有文件的名称为拼音格式，生成映射关系
pub fn convert_dir_to_pinyin(dir_path: &Path) -> anyhow::Result<Vec<(PathBuf, PathBuf)>> {
    let mut result = Vec::new();
    
    // 需要读取目录下的所有文件
    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            let pinyin_path = convert_path_to_pinyin(&path);
            
            // 如果转换后的路径与原路径不同，则添加到结果中
            if pinyin_path != path {
                // 一定要clone原始path，因为后面还会使用
                result.push((path.clone(), pinyin_path));
            }
            
            // 如果是目录，则递归处理
            if path.is_dir() {
                result.extend(convert_dir_to_pinyin(&path)?);
            }
        }
    }
    
    Ok(result)
}

/// 将转换结果保存到文件
pub fn save_pinyin_mappings(mappings: &[(PathBuf, PathBuf)], output: &Path) -> anyhow::Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::File::create(output)?;
    
    for (original, pinyin) in mappings {
        writeln!(file, "{} -> {}", original.display(), pinyin.display())?;
    }
    
    Ok(())
} 