use std::{env, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{RwLock, mpsc};

use crate::query_manager::{ListEntry, QueryParser};

#[derive(Clone)]
pub struct CustomCommandsParser {
    scripts: Arc<RwLock<Vec<ScriptInfo>>>,
}
#[derive(Clone)]
struct ScriptInfo {
    path: String,
    name: String,
    _stem: String,
    _extension: String,
}
impl Default for CustomCommandsParser {
    fn default() -> Self {
        let scripts = Arc::new(RwLock::new(Vec::new()));
        let scripts_clone = scripts.clone();
        tokio::spawn(async move {
            let mut s = tokio::fs::read_dir(
                env::current_exe()
                    .unwrap()
                    .ancestors()
                    .nth(3)
                    .unwrap()
                    .join("scripts"),
            )
            .await
            .unwrap();
            let mut s2 = Vec::new();
            while let Some(sc) = s.next_entry().await.unwrap() {
                if sc.file_type().await.unwrap().is_file() {
                    let mut skip = false;
                    #[cfg(target_os = "windows")]
                    {
                        skip |=
                            vec!["sh"].contains(&sc.path().extension().unwrap().to_str().unwrap());
                    }
                    #[cfg(target_os = "linux")]
                    {
                        skip |= vec!["bat", "exe", "ps1", "lnk"]
                            .contains(&sc.path().extension().unwrap().to_str().unwrap());
                    }
                    if !skip {
                        s2.push(ScriptInfo {
                            _extension: sc
                                .path()
                                .extension()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                            _stem: sc.path().file_stem().unwrap().to_str().unwrap().to_string(),
                            name: sc.path().file_name().unwrap().to_str().unwrap().to_string(),
                            path: sc.path().to_str().unwrap().to_string(),
                        });
                    }
                }
            }
            let mut scripts = scripts_clone.write().await;
            *scripts = s2;
        });
        Self { scripts }
    }
}
#[async_trait]
impl QueryParser for CustomCommandsParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>)->Option<()> {
        let mut scripts = self.scripts.read().await;
        while scripts.len() == 0 {
            drop(scripts);
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            scripts = self.scripts.read().await;
        }
        for s in scripts.iter() {
            let priority;
            if s.name.to_lowercase().starts_with(&query.to_lowercase()) {
                priority = /* prob = (1/26)^priority */(query.len() as f32) + (scripts.len() as f32).log(1.0/26.0);
            } else if s.name.contains(&query) {
                priority = (query.len() as f32)
                    + (scripts.len() as f32).log(1.0 / 26.0)
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
                            open::that_in_background(&s3.path).join().unwrap().unwrap();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            use std::{ffi::OsStr, os::unix::ffi::OsStrExt, path::Path, process::Command};

                            match Path::new(&s3.path)
                                .extension()
                                .unwrap_or(&OsStr::from_bytes(b""))
                                .to_str()
                                .unwrap()
                            {
                                "" | "sh" => {
                                    let _ = Command::new("bash")
                                        .arg("-c")
                                        .arg(s3.path.clone())
                                        .stdin(std::process::Stdio::null())
                                        .stdout(std::process::Stdio::null())
                                        .stderr(std::process::Stdio::null())
                                        .spawn()
                                        .unwrap();
                                }
                                "url"=>{
                                    use std::{fs::File, io::Read};

                                    let mut content=String::new();
                                    File::open(&s3.path).unwrap().read_to_string(&mut content).unwrap();
                                    let arg = content[(content.find("=").unwrap()+1)..].to_string().trim().to_string();
                                    println!("{}",arg);
                                    let _ = Command::new("xdg-open")
                                        .arg(arg)
                                        .spawn()
                                        .unwrap();
                                }
                                _ => {
                                    open::that_in_background(&s3.path).join().unwrap().unwrap();
                                }
                            }
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
