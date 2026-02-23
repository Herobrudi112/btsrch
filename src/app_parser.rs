use std::{process::Command, sync::Arc, time::Instant};

#[cfg(target_os = "windows")]
use serde::Deserialize;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use async_trait::async_trait;
use tokio::sync::{RwLock, mpsc};

use crate::{
    query_manager::{ListEntry, QueryParser},
    search_helper::search,
    unicode_parser::mark_text,
};

#[cfg(target_os = "linux")]
/*
damn, this looks complex.
quotes: \", \`, \$, \\
apparently Exec="\\\\" just becomes \.
This is specifically the case for only(?) \ and $ since they get parsed bevore the quotation marks.
damn it, string parsing is another complicated layer on top! I guess I'll search for a library before doing it myself. this is a lot.
Reserved characters are space (" "), tab, newline, double quote, single quote ("'"), backslash character ("\"), greater-than sign (">"), less-than sign ("<"), tilde ("~"), vertical bar ("|"), ampersand ("&"), semicolon (";"), dollar sign ("$"), asterisk ("*"), question mark ("?"), hash mark ("#"), parenthesis ("(") and (")") and backtick character ("`")
does that mean they all can be escaped with a backslash?
%% -> %
no recursive % parsing
if a field code contains a space. no new argument
field codes: %f, %F, %u, %U, %d, %D, %n, %N, (%i, %c, %k)(these should probably be handled instead of removed...), %v, %m
no field codes in quotes! (:
%F, %U, %i only valid as their own argument
*/
// fn parse_exec_string(s:String)->Vec<String>{

// }
#[cfg(target_os = "linux")]
fn system_language() -> Option<String> {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        use std::env;

        if let Ok(val) = env::var(key) {
            if !val.is_empty() && val != "C" && val != "POSIX" {
                return Some(val.split('.').next().unwrap().to_string());
            }
        }
    }
    None
}
#[cfg(target_os = "linux")]
#[derive(Clone)]
pub struct AppInfo {
    pub filename: String,
    pub name: String,
    pub exec: String,
    pub search_terms: Option<String>,
    pub icon: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppInfo {
    pub name: String,
    pub app_i_d: String,
}
#[derive(Clone)]
pub struct AppParser {
    apps: Arc<RwLock<Vec<AppInfo>>>,
}
impl Default for AppParser {
    fn default() -> Self {
        let app_list = Arc::new(RwLock::new(Vec::new()));
        let app_list_clone = app_list.clone();
        let t = tokio::task::spawn_blocking(|| async move {
            let start = Instant::now();
            #[cfg(target_os = "windows")]
            {
                use std::process::Stdio;
                let output = Command::new("powershell")
                    .arg("-Command")
                    .arg("Get-StartApps | ConvertTo-Json")
                    .stdout(Stdio::piped())
                    .creation_flags(0x08000000)
                    .output()
                    .unwrap();
                let json_str = String::from_utf8_lossy(&output.stdout);
                let apps: Vec<AppInfo> = serde_json::from_str(&json_str).unwrap();
                let mut app_list = app_list_clone.write().await;
                *app_list = apps;
            }
            #[cfg(target_os = "linux")]
            {
                use icon::Icons;

                let icons = Icons::new();
                println!("{:?}", start.elapsed());
                let lang = system_language().unwrap();
                let app_dirs = [
                    if let Ok(s) = std::env::var("XDG_DATA_HOME") {
                        s.split(":").map(|s| format!("{s}/applications")).collect()
                    } else {
                        vec![format!("{}/.local/share/applications", std::env::var("HOME").unwrap())]
                    },
                    if let Ok(s) = std::env::var("XDG_DATA_DIRS") {
                        s.split(":").map(|s| format!("{s}/applications")).collect()
                    } else {
                        vec![format!("/usr/share/applications")]
                    },
                ].into_iter().flatten().collect::<Vec<String>>();
                let mut apps = Vec::new();
                for dir in app_dirs {
                    use std::path::Path;

                    if Path::new(&dir).exists() {
                        use std::fs;

                        if let Ok(entries) = fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                let filename =
                                    path.file_name().unwrap().to_str().unwrap().to_string();
                                if path.extension().map_or(false, |ext| ext == "desktop")
                                    && !apps.iter().any(|a: &AppInfo| a.filename == filename)
                                {
                                    use tokio::fs::read_to_string;

                                    let mut name = Some(
                                        path.file_stem().unwrap().to_str().unwrap().to_string(),
                                    );
                                    let content = read_to_string(&path).await.unwrap();
                                    let lines = content.lines();
                                    let mut name_lang: Option<String> = None;
                                    let mut exec: Option<String> = None;
                                    let mut search_terms: Option<String> = None;
                                    let mut search_terms_lang: Option<String> = None;
                                    let mut icon: Option<String> = None;
                                    let mut display = true;
                                    for l in lines {
                                        if let Some((a, b)) = l.split_once('=') {
                                            match a {
                                                "Name" => {
                                                    name = Some(b.to_string());
                                                }
                                                a if a == format!("Name[{lang}]") => {
                                                    name_lang = Some(b.to_string());
                                                }
                                                "Exec" => {
                                                    exec = Some(b.to_string());
                                                }
                                                "Keywords" => {
                                                    search_terms = Some(b.to_string());
                                                }
                                                a if a == format!("Keywords[{lang}]") => {
                                                    search_terms_lang = Some(b.to_string());
                                                }
                                                "Icon" => {
                                                    icon = Some(b.to_string());
                                                }
                                                "NoDisplay" => {
                                                    display = match b {
                                                        "true" => false,
                                                        "false" => true,
                                                        _ => true,
                                                    }
                                                }
                                                _ => {}
                                            }
                                        } else if l.starts_with('[') && l != "[Desktop Entry]" {
                                            break; // ignore actions
                                        }
                                    }
                                    let name_comb = name_lang.or(name);
                                    if display && name_comb.is_some() && exec.is_some() {
                                        let icon = icon
                                            .map(|icon| {
                                                let find_default_icon = icons.find_icon(
                                                    icon.as_str(),
                                                    16,
                                                    1,
                                                    "breeze-dark",
                                                );
                                                println!("{icon}: {:?}", find_default_icon);
                                                find_default_icon
                                            })
                                            .flatten()
                                            .map(|icon_file| {
                                                let path = icon_file.path();
                                                if path.extension().unwrap().to_str().unwrap()
                                                    == "svg"
                                                {
                                                    let cache_path = format!(
                                                        "{}{}{}.png",
                                                        std::env::current_exe()
                                                            .unwrap()
                                                            .ancestors()
                                                            .nth(3)
                                                            .unwrap()
                                                            .join("btsrch_cache")
                                                            .to_str()
                                                            .unwrap(),
                                                        path.parent().unwrap().to_str().unwrap(),
                                                        path.file_stem().unwrap().to_str().unwrap()
                                                    );
                                                    if !Path::new(&cache_path).exists() {
                                                        use std::fs::{
                                                            create_dir_all, read_to_string,
                                                        };

                                                        use resvg::{
                                                            tiny_skia::Pixmap, usvg::Transform,
                                                        };

                                                        let tree = resvg::usvg::Tree::from_str(
                                                            read_to_string(path).unwrap().as_str(),
                                                            &resvg::usvg::Options::default(),
                                                        )
                                                        .unwrap();
                                                        let mut pixmap = Pixmap::new(
                                                            tree.size().width().ceil() as u32,
                                                            tree.size().height().ceil() as u32,
                                                        )
                                                        .unwrap();
                                                        resvg::render(
                                                            &tree,
                                                            Transform::default(),
                                                            &mut pixmap.as_mut(),
                                                        );
                                                        create_dir_all(
                                                            &Path::new(&cache_path)
                                                                .parent()
                                                                .unwrap(),
                                                        )
                                                        .unwrap();
                                                        pixmap.save_png(&cache_path).unwrap();
                                                    }
                                                    Some(cache_path)
                                                } else if path
                                                    .extension()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap()
                                                    == "xpm"
                                                {
                                                    let cache_path = format!(
                                                        "{}{}{}.png",
                                                        std::env::current_exe()
                                                            .unwrap()
                                                            .ancestors()
                                                            .nth(3)
                                                            .unwrap()
                                                            .join("btsrch_cache")
                                                            .to_str()
                                                            .unwrap(),
                                                        path.parent().unwrap().to_str().unwrap(),
                                                        path.file_stem().unwrap().to_str().unwrap()
                                                    );
                                                    if !Path::new(&cache_path).exists() {
                                                        image::open(&path)
                                                            .unwrap()
                                                            .save(&cache_path)
                                                            .unwrap();
                                                    }
                                                    Some(cache_path)
                                                } else {
                                                    path.to_str().map(|s| s.to_string())
                                                }
                                            })
                                            .flatten();
                                        apps.push(AppInfo {
                                            filename,
                                            name: name_comb.unwrap(),
                                            exec: exec.unwrap(),
                                            search_terms: search_terms.or(search_terms_lang),
                                            icon,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                let mut app_list = app_list_clone.write().await;
                *app_list = apps;
                println!("{:?}", start.elapsed());
            }
        });
        tokio::spawn(async move {
            t.await.unwrap().await;
        });
        Self { apps: app_list }
    }
}
#[async_trait]
impl QueryParser for AppParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        let mut apps = self.apps.read().await;
        while apps.len() == 0 {
            drop(apps);
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            apps = self.apps.read().await;
        }
        let mut results = search(
            &query,
            apps.iter()
                .map(|a| {
                    (
                        Some(a.name.clone())
                            .iter()
                            .chain(a.search_terms.iter())
                            .map(|s| s.clone())
                            .collect::<Vec<String>>()
                            .join(" "),
                        a,
                    )
                })
                .collect(),
        );
        for s in results.drain(..) {
            let priority = 1.0;
            let s2 = apps[s.0].clone();
            let s3 = apps[s.0].clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        #[cfg(target_os = "linux")]
                        {
                            if let Some(handle) = &s2.icon {
                                use eframe::egui::{Image, Vec2};

                                ui.add(
                                    Image::new(format!("file://{}", &handle))
                                        .fit_to_exact_size(Vec2::new(16.0, 16.0)),
                                );
                            }
                        }
                        mark_text(
                            Some(s2.name.clone())
                                .iter()
                                .chain(s2.search_terms.iter())
                                .map(|s| s.clone())
                                .collect::<Vec<String>>()
                                .join(" "),
                            &s.1,
                            ui,
                        );
                    }),
                    execute: Some(Box::new(move || {
                        #[cfg(target_os = "windows")]
                        {
                            let app_id = format!("shell:AppsFolder\\{}", s3.app_i_d);
                            Command::new("explorer").arg(app_id).spawn().unwrap();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            let mut args = s3
                                .exec
                                .split(' ')
                                .filter(|s| !vec!["%F", "%U"].contains(s))
                                .collect::<Vec<&str>>();
                            let _ = Command::new(args[0])
                                .args(&mut args[1..])
                                .stdin(std::process::Stdio::null())
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null())
                                .spawn()
                                .unwrap();
                        }
                        std::process::exit(0);
                    })),
                    priority,
                })
                .await
                .ok()?;
        }
        None
    }
}
