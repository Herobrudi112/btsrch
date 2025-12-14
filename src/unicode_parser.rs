use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine;
use egui::{
    Align, Color32, ColorImage, FontSelection, Image, RichText, Style, TextureHandle, TextureOptions, Ui, Vec2, text::LayoutJob
};
use image::ImageFormat;
use serde::Deserialize;
use tokio::sync::{RwLock, mpsc};

use crate::{
    query_manager::{ListEntry, QueryParser},
    search_helper::search,
};

#[derive(Clone, Deserialize)]
pub struct EmojiList {
    pub emojis: Vec<Emoji>,
}
#[derive(Clone, Deserialize)]
pub struct Emoji {
    pub name: String,
    pub emoji: String,
    pub image: String,
}
#[derive(Clone, Deserialize)]
pub struct UnicodeCharRaw {
    pub name: String,
    pub key: String,
}
#[derive(Clone)]
pub struct UnicodeChar {
    pub name: String,
    pub key: String,
    pub picture: Option<Arc<std::sync::RwLock<(ColorImage, Option<TextureHandle>)>>>,
}
fn decode_base64_image(data_uri: &str) -> Option<ColorImage> {
    let base64_data = data_uri.split(',').nth(1)?; // strip "data:image/png;base64,"
    let bytes = base64::prelude::BASE64_STANDARD.decode(base64_data).ok()?;
    let img = image::load_from_memory_with_format(&bytes, ImageFormat::Png).ok()?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_vec();
    Some(ColorImage::from_rgba_unmultiplied(size, &pixels))
}
#[derive(Clone)]
pub struct UnicodeParser {
    unicode: Arc<RwLock<Vec<UnicodeChar>>>,
}
impl Default for UnicodeParser {
    fn default() -> Self {
        let unicode_list = Arc::new(RwLock::new(Vec::new()));
        let unicode_list_clone = unicode_list.clone();
        tokio::spawn(async move {
            let filee = include_str!("../list.with.images.with.modifiers.json");
            let emojis_raw: EmojiList = serde_json::from_str(&filee).unwrap();
            let emojis = emojis_raw
                .emojis
                .into_iter()
                .map(|e| UnicodeChar {
                    name: e.name,
                    key: e.emoji,
                    picture: Some(Arc::new(std::sync::RwLock::new((
                        decode_base64_image(&e.image).unwrap(),
                        None,
                    )))),
                })
                .collect::<Vec<UnicodeChar>>();
            let fileu = include_str!("../UnicodeData.txt");
            let mut chars = fileu
                .lines()
                .filter(|l| l.len() > 0)
                .map(|l| {
                    let mut semicolon_seperated =
                        l.split(';').map(|s| s.to_string()).collect::<Vec<String>>();
                    let character = char::from_u32(
                        u32::from_str_radix(&semicolon_seperated[0].to_lowercase(), 16).unwrap(),
                    )
                    .map(|c| c.to_string())
                    .unwrap_or(String::new());
                    let name = semicolon_seperated.remove(1);
                    UnicodeChar {
                        name: name.to_lowercase(),
                        key: character,
                        picture: None,
                    }
                })
                .filter(|uc| !emojis.iter().any(|e| e.key == uc.key))
                .collect::<Vec<UnicodeChar>>();
            chars.extend(emojis);
            let mut unicode_list = unicode_list_clone.write().await;
            *unicode_list = chars;
        });
        Self {
            unicode: unicode_list,
        }
    }
}
#[async_trait]
impl QueryParser for UnicodeParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        let mut characters = self.unicode.read().await;
        while characters.len() == 0 {
            drop(characters);
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            characters = self.unicode.read().await;
        }
        let texts = characters
            .iter()
            .map(|c| (format!("{} {}", &c.key, &c.name), c))
            .collect::<Vec<(String, &UnicodeChar)>>();
        let mut found = search(&query, texts);
        for (id, mark) in found.drain(..) {
            let s = &characters[id];
            let priority = 1.0;
            let s2 = s.clone();
            let s3 = s.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |mut ui| {
                        let handle = if let Some(picture) = s2.picture.as_ref() {
                            let read = picture.read().unwrap();
                            let handle = if let Some(handle) = &read.1 {
                                handle.clone()
                            } else {
                                let handle = ui.ctx().load_texture(
                                    s2.name.replace(" ", "_"),
                                    read.0.clone(),
                                    TextureOptions::default(),
                                );
                                drop(read);
                                picture.write().unwrap().1 = Some(handle.clone());
                                handle
                            };
                            Some(handle)
                        } else {
                            None
                        };
                        if let Some(handle) = handle {
                            ui.add(Image::new(&handle).fit_to_exact_size(Vec2::new(16.0, 16.0)));
                        }
                        let s = format!("{} {}", &s2.key, &s2.name);
                        mark_text(s, &mark, &mut ui);
                    }),
                    execute: Some(Box::new(move || {
                        let mut clipboard = arboard::Clipboard::new().unwrap();
                        clipboard.set_text(s3.key.clone()).unwrap();
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

fn mark_text(s:String, mark:&Vec<usize>, ui:&mut Ui) {
    let style = Style::default();
    let mut text = LayoutJob::default();
    let mut last = 0;
    let mut marked = false;
    for i in mark.iter().chain(std::iter::once(&s.len())) {
        let curtxt = s[last..*i].to_string();
        if !marked {
            RichText::new(curtxt)
                .color(Color32::from_rgb(255, 255, 255))
                .append_to(
                    &mut text,
                    &style,
                    FontSelection::Default,
                    Align::Center,
                );
        } else {
            RichText::new(curtxt)
                .color(Color32::from_rgb(0, 255, 255))
                .underline()
                .append_to(
                    &mut text,
                    &style,
                    FontSelection::Default,
                    Align::Center,
                );
        }
        last = *i;
        marked = !marked;
    }
    ui.label(text);
}
