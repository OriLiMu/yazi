use std::{ffi::OsStr, fmt::Display, ops::Range};

use anyhow::Result;
use regex::bytes::{Regex, RegexBuilder};
use yazi_shared::event::Cmd;

use crate::pinyin_utils::to_pinyin_initials;

pub struct Filter {
	raw:   String,
	regex: Regex,
	has_chinese: bool,
}

impl Filter {
	pub fn new(s: &str, case: FilterCase) -> Result<Self> {
		// 检查搜索字符串中是否包含中文字符
		let has_chinese = s.chars().any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF);

		let regex = match case {
			FilterCase::Smart => {
				let uppercase = s.chars().any(|c| c.is_uppercase());
				RegexBuilder::new(s).case_insensitive(!uppercase).build()?
			}
			FilterCase::Sensitive => Regex::new(s)?,
			FilterCase::Insensitive => RegexBuilder::new(s).case_insensitive(true).build()?,
		};
		Ok(Self { raw: s.to_owned(), regex, has_chinese })
	}

	#[inline]
	pub fn matches(&self, name: &OsStr) -> bool {
		// 首先尝试普通匹配
		if self.regex.is_match(name.as_encoded_bytes()) {
			return true;
		}
		
		// 处理拼音匹配 - 所有模式都支持
		if let Some(file_name) = name.to_str() {
			// 检查文件名是否包含中文
			let file_has_chinese = file_name.chars().any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF);
			
			// 只有当文件名包含中文时才进行拼音处理
			if file_has_chinese {
				// 转换为拼音格式
				let pinyin = to_pinyin_initials(file_name);
				let matched = self.regex.is_match(pinyin.as_bytes());
				return matched;
			}
		}
		
		false
	}

	#[inline]
	pub fn highlighted(&self, name: &OsStr) -> Option<Vec<Range<usize>>> {
		// 检查普通匹配
		if let Some(m) = self.regex.find(name.as_encoded_bytes()) {
			return Some(vec![m.range()]);
		}
		
		// 如果没有普通匹配，对包含中文的文件名进行拼音匹配
		if name.to_str().is_some_and(|s| s.chars().any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF)) {
			// 对于拼音模式匹配，返回整个名称范围
			return name.to_str().map(|s| vec![0..s.len()]);
		}
		
		None
	}
}

impl PartialEq for Filter {
	fn eq(&self, other: &Self) -> bool { 
		self.raw == other.raw && self.has_chinese == other.has_chinese
	}
}

impl Display for Filter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
		f.write_str(&self.raw)
	}
}

#[derive(Default, PartialEq, Eq)]
pub enum FilterCase {
	Smart,
	#[default]
	Sensitive,
	Insensitive,
}

impl From<&Cmd> for FilterCase {
	fn from(c: &Cmd) -> Self {
		match (c.bool("smart"), c.bool("insensitive")) {
			(true, _) => Self::Smart,
			(_, false) => Self::Sensitive,
			(_, true) => Self::Insensitive,
		}
	}
}
