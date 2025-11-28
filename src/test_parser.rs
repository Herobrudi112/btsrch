use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::query_manager::{ListEntry, QueryParser};

#[derive(Clone)]
pub struct TestParser{

}
impl Default for TestParser{
    fn default() -> Self {
        Self {  }
    }
}
#[async_trait]
impl QueryParser for TestParser{
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>){
        for c in query.chars(){
            resopnse.send(ListEntry { layout_fn: Box::new(move |ui|{
                ui.label(format!("{}",c));
            }), execute: Some(Box::new(move ||{
                println!("{}",c);
            })), priority: 0.0, }).await.unwrap();
        }
    }
}