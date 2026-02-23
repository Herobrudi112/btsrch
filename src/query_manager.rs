use async_trait::async_trait;
use tokio::{sync::mpsc, task::JoinHandle};

pub type LayoutFn = Box<dyn FnMut(&mut eframe::egui::Ui) + Send + Sync>;
pub type ExecuteFn = Box<dyn FnMut() + Send + Sync>;
pub type ScrollToFn = Box<dyn FnMut() + Send + Sync>;

#[async_trait]
pub trait QueryParser: BoxClone + Send + Sync + 'static {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>)->Option<()>;
}

/// stupid dumb crazy mad workaround for dyn compatibility
pub trait BoxClone {
    fn clone_box(&self) -> Box<dyn QueryParser>;
}
impl<T> BoxClone for T
where
    T: Clone + QueryParser,
{
    fn clone_box(&self) -> Box<dyn QueryParser> {
        Box::new(self.clone())
    }
}

pub struct ListEntry {
    pub layout_fn: LayoutFn,
    pub execute: Option<ExecuteFn>,
    pub priority:f32,
}

pub enum ChangeInstruction {
    Add(ListEntry),
    Empty,
}

pub struct QueryManager {
    parsers: Vec<Box<dyn QueryParser>>,
    signal_receiver: mpsc::Receiver<String>,
    layout_sender: mpsc::Sender<ChangeInstruction>,
}

impl QueryManager {
    pub fn new(
        signal_receiver: mpsc::Receiver<String>,
        layout_sender: mpsc::Sender<ChangeInstruction>,
    ) -> Self {
        QueryManager {
            signal_receiver,
            layout_sender,
            parsers: Vec::new(),
        }
    }
    pub fn add_query_parser<T>(&mut self)
    where
        T: QueryParser + Default,
    {
        self.parsers.push(Box::new(T::default()));
    }
    pub fn add_custom_query_parser<T>(&mut self, parser: T)
    where
        T: QueryParser,
    {
        self.parsers.push(Box::new(parser));
    }
    pub fn start(self)-> JoinHandle<()> {
        let mut receiver = self.signal_receiver;
        tokio::spawn(async move {
            let sender = self.layout_sender;
            let mut handles: Vec<tokio::task::JoinHandle<Option<()>>>=Vec::new();
            while let Some(query) = receiver.recv().await {
                let mut parsers = Vec::new();
                for p in &self.parsers{
                    parsers.push(p.clone_box());
                }
                if receiver.len()>0{
                    continue;
                }
                for h in handles.iter(){
                    h.abort();
                }
                for h in handles.drain(..){
                    let _=h.await;
                }
                sender.send(ChangeInstruction::Empty).await.unwrap();
                let (tx, mut rx)=mpsc::channel(128);
                for p in parsers.drain(..) {
                    let q2=query.clone();
                    let tx2=tx.clone();
                    handles.push(tokio::spawn(async move{
                        p.parse(q2, tx2).await
                    }));
                }
                let s2=sender.clone();
                handles.push(tokio::spawn(async move{
                    while let Some(v)=rx.recv().await{
                        s2.send(ChangeInstruction::Add(v)).await.ok()?;
                    }
                    None
                }));
            }
        })
    }
}
