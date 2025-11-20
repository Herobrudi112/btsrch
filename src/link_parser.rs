use async_trait::async_trait;
use regex::Regex;
use tokio::sync::mpsc;

use crate::query_manager::{ListEntry, QueryParser};

#[derive(Clone)]
pub struct LinkParser {}
impl Default for LinkParser {
    fn default() -> Self {
        Self {}
    }
}
#[async_trait]
impl QueryParser for LinkParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) {
        let top_level_domains = vec![
            "com", // Commercial
            "org", // Organization
            "net", // Network
            "edu", // Education
            "co",  // Company (used in countries like .co.uk)
            "io",  // Tech startups
            "us",  // United States
            "uk",  // United Kingdom
            "ca",  // Canada
            "de",  // Germany
            "rs",  // docs.rs
            "tv",
        ];
        let word = r"([A-Za-z0-9_\-]+)";
        let bword = r"([A-Za-z0-9_\-%\.]+)";
        let tlds = format!("({})", top_level_domains.join("|"));
        let byte = r"(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]?|0)";
        let regex = format!(
            r"^((https?)://)?((({word}\.)+{tlds})|({byte}\.){{3}}{byte}|localhost)(:[0-9]{{1,5}})?(/{bword})*(/(\?({bword}={bword}&)*({bword}={bword}))?)?$"
        );
        let specifies_protocoll = r"^https?://";
        let re = Regex::new(&regex).unwrap();
        let protocoll = Regex::new(&specifies_protocoll).unwrap();
        let q2 = query.clone();
        if re.is_match(&q2) {
            let final_link = match protocoll.is_match(&q2) {
                true => q2.clone(),
                false => format!("https://{q2}"),
            };
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        ui.label(format!("open {} in the browser", &q2));
                    }),
                    execute: Some(Box::new(move || {
                        open::that_in_background(&final_link)
                            .join()
                            .unwrap()
                            .unwrap();
                        std::process::exit(0);
                    })),
                    priority: 100.0,
                })
                .await
                .unwrap();
        }
    }
}
