use tracing::warn;
use yazi_shared::env_exists;

use crate::Mux;

#[derive(Clone, Copy, Debug)]
pub enum Brand {
	Kitty,
	Konsole,
	Iterm2,
	WezTerm,
	Foot,
	Ghostty,
	Microsoft,
	Rio,
	BlackBox,
	VSCode,
	Tabby,
	Hyper,
	Mintty,
	Neovim,
	Apple,
	Urxvt,
}

impl Brand {
	pub fn from_env() -> Option<Self> {
		use Brand as B;

		if env_exists("NVIM_LOG_FILE") && env_exists("NVIM") {
			return Some(Self::Neovim);
		}

		let vars = [
			("KITTY_WINDOW_ID", B::Kitty),
			("KONSOLE_VERSION", B::Konsole),
			("ITERM_SESSION_ID", B::Iterm2),
			("WEZTERM_EXECUTABLE", B::WezTerm),
			("GHOSTTY_RESOURCES_DIR", B::Ghostty),
			("WT_Session", B::Microsoft),
			("VSCODE_INJECTION", B::VSCode),
			("TABBY_CONFIG_DIRECTORY", B::Tabby),
		];
		match vars.into_iter().find(|&(s, _)| env_exists(s)) {
			Some((_, brand)) => return Some(brand),
			None => warn!("[Adapter] No special environment variables detected"),
		}

		let (term, program) = B::env();
		match program.as_str() {
			"iTerm.app" => return Some(B::Iterm2),
			"WezTerm" => return Some(B::WezTerm),
			"ghostty" => return Some(B::Ghostty),
			"rio" => return Some(B::Rio),
			"BlackBox" => return Some(B::BlackBox),
			"vscode" => return Some(B::VSCode),
			"Tabby" => return Some(B::Tabby),
			"Hyper" => return Some(B::Hyper),
			"mintty" => return Some(B::Mintty),
			"Apple_Terminal" => return Some(B::Apple),
			_ => warn!("[Adapter] Unknown TERM_PROGRAM: {program}"),
		}
		match term.as_str() {
			"xterm-kitty" => return Some(B::Kitty),
			"foot" => return Some(B::Foot),
			"foot-extra" => return Some(B::Foot),
			"xterm-ghostty" => return Some(B::Ghostty),
			"rio" => return Some(B::Rio),
			"rxvt-unicode-256color" => return Some(B::Urxvt),
			_ => warn!("[Adapter] Unknown TERM: {term}"),
		}
		None
	}

	pub(super) fn from_csi(resp: &str) -> Option<Self> {
		let names = [
			("kitty", Self::Kitty),
			("Konsole", Self::Konsole),
			("iTerm2", Self::Iterm2),
			("WezTerm", Self::WezTerm),
			("foot", Self::Foot),
			("ghostty", Self::Ghostty),
		];
		names.into_iter().find(|&(n, _)| resp.contains(n)).map(|(_, b)| b)
	}

	pub(super) fn adapters(self) -> &'static [crate::Adapter] {
		use Brand as B;

		use crate::Adapter as A;

		match self {
			B::Kitty => &[A::Kgp],
			B::Konsole => &[A::KgpOld],
			B::Iterm2 => &[A::Iip, A::Sixel],
			B::WezTerm => &[A::Iip, A::Sixel],
			B::Foot => &[A::Sixel],
			B::Ghostty => &[A::Kgp],
			B::Microsoft => &[A::Sixel],
			B::Rio => &[A::Iip, A::Sixel],
			B::BlackBox => &[A::Sixel],
			B::VSCode => &[A::Iip, A::Sixel],
			B::Tabby => &[A::Iip, A::Sixel],
			B::Hyper => &[A::Iip, A::Sixel],
			B::Mintty => &[A::Iip],
			B::Neovim => &[],
			B::Apple => &[],
			B::Urxvt => &[],
		}
	}

	fn env() -> (String, String) {
		let (term, program) = Mux::term_program();
		(
			term.unwrap_or(std::env::var("TERM").unwrap_or_default()),
			program.unwrap_or(std::env::var("TERM_PROGRAM").unwrap_or_default()),
		)
	}
}
