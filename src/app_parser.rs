use std::{process::Command, sync::Arc};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use async_trait::async_trait;
use serde::Deserialize;
use tokio::sync::{RwLock, mpsc};

use crate::query_manager::{ListEntry, QueryParser};

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
                let app_dirs = [
                    "/usr/share/applications",
                    &format!(
                        "{}/.local/share/applications",
                        std::env::var("HOME").unwrap()
                    ),
                ];
                let mut apps = Vec::new();
                for dir in app_dirs {
                    use std::path::Path;

                    if Path::new(dir).exists() {
                        use std::fs;

                        if let Ok(entries) = fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.extension().map_or(false, |ext| ext == "desktop") {
                                    use tokio::{fs::File, io::AsyncReadExt};

                                    let name =
                                        path.file_stem().unwrap().to_str().unwrap().to_string();
                                    let mut content = String::new();
                                    File::open(path)
                                        .await
                                        .unwrap()
                                        .read_to_string(&mut content)
                                        .await
                                        .unwrap();
                                    if let Some(pos)=content.find("\nExec="){
                                        let ec = content[(pos + 6)..]
                                            .to_string();
                                        let exec = ec[..(ec.find("\n").unwrap())].to_string();
                                        apps.push(AppInfo {
                                            name,
                                            app_i_d: exec,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                let mut app_list = app_list_clone.write().await;
                *app_list = apps;
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
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>)->Option<()> {
        let mut apps = self.apps.read().await;
        while apps.len() == 0 {
            drop(apps);
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            apps = self.apps.read().await;
        }
        for s in apps.iter() {
            let priority;
            if s.name.to_lowercase().starts_with(&query.to_lowercase()) {
                priority = /* prob = (1/26)^priority */(query.len() as f32) + (apps.len() as f32).log(1.0/26.0);
            } else if s.name.to_lowercase().contains(&query.to_lowercase()) {
                priority = (query.len() as f32)
                    + (apps.len() as f32).log(1.0 / 26.0)
                    + ((s.name.len() - query.len()) as f32).log(1.0 / 26.0);
            } else {
                continue;
            }
            let s2 = s.clone();
            let s3 = s.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        ui.label(format!("{}", &s2.name));
                    }),
                    execute: Some(Box::new(move || {
                        #[cfg(target_os = "windows")]
                        {
                            let app_id = format!("shell:AppsFolder\\{}", s3.app_i_d);
                            Command::new("explorer").arg(app_id).spawn().unwrap();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            let _ = Command::new("bash")
                                .arg("-c")
                                .arg(s3.app_i_d.clone())
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
