#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app_parser;
pub mod custom_commands_parser;
pub mod link_parser;
pub mod path_parser;
pub mod query_manager;
pub mod test_parser;
pub mod unicode_parser;
pub mod unit_calc_parser;
pub mod search_helper;

use std::sync::Arc;

use eframe::egui;
use egui::{Align, CentralPanel, FontId, Key, Layout, Shadow};
use egui::{Frame, TextEdit};
use single_instance::SingleInstance;
use tokio::sync::mpsc;

use crate::app_parser::AppParser;
use crate::custom_commands_parser::CustomCommandsParser;
use crate::link_parser::LinkParser;
use crate::path_parser::PathParser;
use crate::query_manager::{ChangeInstruction, ListEntry, QueryManager};
use crate::unicode_parser::UnicodeParser;
use crate::unit_calc_parser::main::UnitCalcParser;

struct SearchApp {
    query: String,
    pub layout_receiver: mpsc::Receiver<ChangeInstruction>,
    layout: Vec<ListEntry>,
    pub query_sender: mpsc::Sender<String>,
    selected_id: usize,
    scroll_todo: bool,
    had_focus: bool,
    last_input:String,
}

impl SearchApp {
    fn new(tx: mpsc::Sender<String>, rx: mpsc::Receiver<ChangeInstruction>) -> Self {
        Self {
            query: String::new(),
            layout: Vec::new(),
            query_sender: tx,
            layout_receiver: rx,
            selected_id: usize::MAX,
            had_focus: false,
            scroll_todo: false,
            last_input:String::new(),
        }
    }
}

impl eframe::App for SearchApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                Frame::NONE
                    .fill(egui::Color32::from_rgba_unmultiplied(10 + 30, 10, 10, 200))
                    .corner_radius(10)
                    .outer_margin(5)
                    .inner_margin(5)
                    .shadow(Shadow {
                        offset: [0, 0],
                        blur: 0,
                        spread: 2,
                        color: egui::Color32::from_rgba_unmultiplied(0, 255, 255, 128),
                    })
                    .show(ui, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            let resp = ui.add(
                                TextEdit::singleline(&mut self.query)
                                    // .hint_text("Type to search...")
                                    .desired_width(f32::INFINITY)
                                    .lock_focus(true)
                                    .font(FontId::new(24.0, egui::FontFamily::Proportional))
                                    .frame(false),
                            );
                            resp.request_focus();
                            if resp.changed() {
                                let q = self.query.clone();
                                if self.last_input!=q{
                                    let sender = self.query_sender.clone();
                                    self.last_input=q.clone();
                                    tokio::spawn(async move {
                                        sender.send(q).await.unwrap();
                                    });
                                }
                            }
                        });
                    });
                egui::ScrollArea::vertical()
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down(Align::Center), |ui| {
                            while let Ok(l) = self.layout_receiver.try_recv() {
                                match l {
                                    ChangeInstruction::Add(la) => {
                                        if self.selected_id == usize::MAX && la.execute.is_some() {
                                            self.selected_id = self.layout.len();
                                        }
                                        self.layout.push(la);
                                    }
                                    ChangeInstruction::Empty => {
                                        self.layout.clear();
                                        self.selected_id = usize::MAX;
                                        self.scroll_todo = false;
                                    }
                                }
                            }
                            self.layout
                                .sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
                            if self.selected_id != usize::MAX {
                                while self.layout[self.selected_id].execute.is_none() {
                                    self.selected_id = (self.selected_id + 1) % self.layout.len();
                                }
                            }
                            for i in 0..self.layout.len() {
                                let l = &mut self.layout[i];
                                let mut brightness = 10;
                                if l.execute.is_some() {
                                    brightness = 20;
                                    if i == self.selected_id {
                                        brightness = 50;
                                    }
                                }
                                let frame = Frame::NONE
                                    .fill(egui::Color32::from_rgba_unmultiplied(
                                        brightness + 30,
                                        brightness,
                                        brightness,
                                        200,
                                    ))
                                    .corner_radius(10)
                                    .outer_margin(5)
                                    .inner_margin(5)
                                    .shadow(Shadow {
                                        offset: [0, 0],
                                        blur: 0,
                                        spread: 2,
                                        color: egui::Color32::from_rgba_unmultiplied(
                                            0, 255, 255, 128,
                                        ),
                                    })
                                    .show(ui, |ui| {
                                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                            (l.layout_fn)(ui);
                                        });
                                    });
                                if self.scroll_todo && self.selected_id == i {
                                    frame.response.scroll_to_me(None);
                                    self.scroll_todo = false;
                                }
                            }
                        });
                    });
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                    if self.selected_id != usize::MAX {
                        self.selected_id = (self.selected_id + 1) % self.layout.len();
                        while self.layout[self.selected_id].execute.is_none() {
                            self.selected_id = (self.selected_id + 1) % self.layout.len();
                        }
                        self.scroll_todo = true;
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                    if self.selected_id != usize::MAX {
                        self.selected_id =
                            (self.selected_id - 1 + self.layout.len()) % self.layout.len();
                        while self.layout[self.selected_id].execute.is_none() {
                            self.selected_id =
                                (self.selected_id - 1 + self.layout.len()) % self.layout.len();
                        }
                        self.scroll_todo = true;
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    if self.selected_id != usize::MAX {
                        (self.layout[self.selected_id].execute.as_mut().unwrap())();
                    }
                }
                if ctx.input(|i| i.focused) {
                    self.had_focus = true;
                }
                if ctx.input(|i| !i.focused) {
                    if self.had_focus {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
            ..egui::Visuals::dark()
        });
        let mut style = (*ctx.style()).clone();
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_style(style);
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                r"../NotoSansSymbols-Regular-Subsetted.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        ctx.set_fonts(fonts);
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let mut options = eframe::NativeOptions::default();
    options.run_and_return = false;
    #[cfg(target_os = "windows")]
    {
        options.centered = true;
        options.viewport = egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_inner_size(egui::vec2(500.0, 1000.0))
            .with_always_on_top()
            .with_active();
    }
    #[cfg(target_os = "linux")]
    {
        use x11rb::{connection::Connection, protocol::randr::ConnectionExt};
        
        const WIDTH: f32 = 500.0;
        const HEIGHT: f32 = 1000.0;
        options.centered=true;
        options.viewport = egui::ViewportBuilder::default()
        .with_decorations(false)
        .with_transparent(true)
            .with_inner_size(egui::vec2(WIDTH, HEIGHT))
            .with_always_on_top()
            .with_active(true);
        if let Ok((conn, screen_num))=x11rb::connect(None){
            let roots = &conn.setup().roots[screen_num];
            let screen = roots;
            let primary_id = conn
                .randr_get_output_primary(screen.root)
                .unwrap()
                .reply()
                .unwrap()
                .output;
            if let Ok(temp)=conn
                .randr_get_output_info(primary_id, 0)
                .unwrap()
                .reply(){
                    let primary_crtc=temp.crtc;
                    let primary_info = conn
                        .randr_get_crtc_info(primary_crtc, 0)
                        .unwrap()
                        .reply()
                        .unwrap();
                    let x = primary_info.x + ((primary_info.width / 2) as i16) - (WIDTH as i16) / 2;
                    let y = primary_info.y + ((primary_info.height / 2) as i16) - (HEIGHT as i16) / 2;
                    options.viewport=options.viewport.with_position((x as f32, y as f32));
                    options.centered=false;
                }
        }
    }
    let (atx, rx) = mpsc::channel::<String>(128);
    let (tx, arx) = mpsc::channel::<ChangeInstruction>(128);
    let app = SearchApp::new(atx, arx);
    let mut mgr = QueryManager::new(rx, tx);
    let a = tokio::task::spawn_blocking(|| async move {
        let instance = SingleInstance::new("very_short_btsrch_key").unwrap();
        if !instance.is_single() {
            println!("already open...");
            std::process::exit(0);
        }
        mgr.add_query_parser::<CustomCommandsParser>();
        mgr.add_query_parser::<LinkParser>();
        mgr.add_query_parser::<PathParser>();
        mgr.add_query_parser::<UnitCalcParser>();
        mgr.add_query_parser::<AppParser>();
        mgr.add_query_parser::<UnicodeParser>();
        mgr.start().await.unwrap();
    });
    tokio::spawn(async move {
        a.await.unwrap().await;
    });
    eframe::run_native("BTSRCH", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}
