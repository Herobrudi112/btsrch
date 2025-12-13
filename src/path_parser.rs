use std::path::Path;

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::query_manager::{ListEntry, QueryParser};

#[derive(Clone)]
pub struct PathParser {}
impl Default for PathParser {
    fn default() -> Self {
        Self {}
    }
}
#[async_trait]
impl QueryParser for PathParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) ->Option<()>{
        if Path::new(&query).exists() {
            let q2=query.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        ui.label(format!("open {}", &query));
                    }),
                    execute: Some(Box::new(move || {
                        open::that_in_background(&q2)
                            .join()
                            .unwrap()
                            .unwrap();
                        std::process::exit(0);
                    })),
                    priority: 10.0,
                })
                .await
                .ok()?;
        }
        None
    }
}
