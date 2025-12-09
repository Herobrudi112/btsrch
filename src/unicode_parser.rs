use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine;
use egui::{
    Align, Color32, ColorImage, FontFamily, FontSelection, Image, RichText, Style, TextureHandle, TextureOptions, Vec2, text::LayoutJob
};
use image::ImageFormat;
use serde::Deserialize;
use tokio::sync::{RwLock, mpsc};

use crate::query_manager::{ListEntry, QueryParser};

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
            let filec = include_str!("../unicode.json");// tokio::fs::read_to_string("unicode.json").await.unwrap();
            let mut chars: Vec<UnicodeChar> = serde_json::from_str::<Vec<UnicodeCharRaw>>(&filec)
                .unwrap()
                .iter()
                .map(|c| UnicodeChar {
                    name: c.name.clone(),
                    key: c.key.clone(),
                    picture: None,
                })
                .collect();
            let filee = include_str!("../list.with.images.with.modifiers.json");//tokio::fs::read_to_string("list.with.images.with.modifiers.json").await.unwrap();
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
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) {
        let mut characters = self.unicode.read().await;
        while characters.len() == 0 {
            drop(characters);
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            characters = self.unicode.read().await;
        }
        for s in characters.iter() {
            let priority;
            if s.name
                .to_lowercase()
                .split(" ")
                .collect::<Vec<&str>>()
                .contains(&query.to_lowercase().as_str())
            {
                priority = /* prob = (1/26)^priority */(query.len() as f32) + (characters.len() as f32).log(1.0/26.0);
            } else if s.name.to_lowercase().contains(&query.to_lowercase()) {
                priority = (query.len() as f32)
                    + (characters.len() as f32).log(1.0 / 26.0)
                    + ((s.name.len() - query.len()) as f32).log(1.0 / 26.0);
            } else {
                continue;
            }
            let s2 = s.clone();
            let s3 = s.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
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
                        ui.label(format!("{} {}", &s2.key, &s2.name));
                        // let style = Style::default();
                        // let mut text = LayoutJob::default();
                        // RichText::new("non-bold text ").color(Color32::from_rgb(255, 255, 255)).append_to(
                        //     &mut text,
                        //     &style,
                        //     FontSelection::Default,
                        //     Align::Center,
                        // );
                        // RichText::new("bold text").color(Color32::from_rgb(0, 255, 255)).underline().append_to(
                        //     &mut text,
                        //     &style,
                        //     FontSelection::Default,
                        //     Align::Center,
                        // );
                        // RichText::new(" non-bold text").color(Color32::from_rgb(255, 255, 255)).append_to(
                        //     &mut text,
                        //     &style,
                        //     FontSelection::Default,
                        //     Align::Center,
                        // );
                        // ui.label(text);
                    }),
                    execute: Some(Box::new(move || {
                        let mut clipboard = arboard::Clipboard::new().unwrap();
                        clipboard.set_text(s3.key.clone()).unwrap();
                        std::process::exit(0);
                    })),
                    priority,
                })
                .await
                .unwrap();
        }
    }
}
