use std::{env, fs, path::PathBuf, sync::OnceLock};

#[derive(Clone, Default)]
pub struct Config {
    pub style: Option<String>,
    pub search: Option<String>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn config_path() -> Option<PathBuf> {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            return Some(dir.join("btsrch.conf"));
        }
    }
    None
}

fn load_config() -> Config {
    let mut cfg = Config::default();
    if let Some(path) = config_path() {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim().to_string();
                    if key.eq_ignore_ascii_case("style") {
                        cfg.style = Some(value);
                    } else if key.eq_ignore_ascii_case("search") {
                        cfg.search = Some(value);
                    }
                }
            }
        }
    }
    cfg
}

pub fn config() -> &'static Config {
    CONFIG.get_or_init(load_config)
}

pub fn is_launcher_style() -> bool {
    matches!(
        config().style.as_deref(),
        Some(s) if s.eq_ignore_ascii_case("launcher") || s.eq_ignore_ascii_case("gnome")
    )
}

pub fn is_sachsi_search() -> bool {
    matches!(
        config().search.as_deref(),
        Some(s) if s.eq_ignore_ascii_case("sachsi")
    )
}

