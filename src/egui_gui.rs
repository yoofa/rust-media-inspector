use eframe::{egui, NativeOptions};
use egui::{Color32, RichText, ViewportBuilder};
use rfd::FileDialog;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::analyzer::{DefaultAnalyzer, ElementInfo, MediaAnalyzer, MediaInfo};

pub fn run_gui() -> i32 {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Media Inspector")
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "Media Inspector",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(MediaInspectorApp::default())
        }),
    )
    .map_or(-1, |_| 0)
}

struct MediaInspectorApp {
    media_info: Option<MediaInfo>,
    error_message: Option<String>,
    rx: Receiver<Result<MediaInfo, String>>,
    tx: Sender<Result<MediaInfo, String>>,
    search_text: String,
    expanded_nodes: std::collections::HashSet<String>,
    should_exit: bool,
}

impl Default for MediaInspectorApp {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            media_info: None,
            error_message: None,
            rx,
            tx,
            search_text: String::new(),
            expanded_nodes: std::collections::HashSet::new(),
            should_exit: false,
        }
    }
}

impl eframe::App for MediaInspectorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // 检查文件拖放
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            if let Some(file) = ctx.input(|i| i.raw.dropped_files.first().cloned()) {
                if let Some(path) = file.path {
                    let tx = self.tx.clone();
                    std::thread::spawn(move || {
                        let analyzer = DefaultAnalyzer::new(true);
                        match analyzer.analyze(path.to_str().unwrap()) {
                            Ok(info) => tx.send(Ok(info)),
                            Err(e) => tx.send(Err(e.to_string())),
                        }
                    });
                }
            }
        }

        // 检查后台任务结果
        if let Ok(result) = self.rx.try_recv() {
            match result {
                Ok(info) => {
                    self.media_info = Some(info);
                    self.error_message = None;
                }
                Err(err) => {
                    self.error_message = Some(err);
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        ui.close_menu();
                        let tx = self.tx.clone();
                        std::thread::spawn(move || {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Media Files", &["mp4", "mov", "m4a"])
                                .pick_file()
                            {
                                let analyzer = DefaultAnalyzer::new(true);
                                match analyzer.analyze(path.to_str().unwrap()) {
                                    Ok(info) => tx.send(Ok(info)),
                                    Err(e) => tx.send(Err(e.to_string())),
                                }
                            } else {
                                Ok(())
                            }
                        });
                    }
                    if ui.button("Exit").clicked() {
                        self.should_exit = true;
                        ui.close_menu();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Expand All").clicked() {
                        if let Some(info) = self.media_info.as_ref() {
                            let mut paths = std::collections::HashSet::new();
                            for element in &info.structure {
                                Self::collect_paths(element, "", &mut paths);
                            }
                            self.expanded_nodes = paths;
                        }
                    }
                    if ui.button("Collapse All").clicked() {
                        self.expanded_nodes.clear();
                    }
                });
            });

            // 搜索框
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_text);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error_message {
                ui.colored_label(Color32::RED, err);
            } else if let Some(info) = &self.media_info {
                let search_text = self.search_text.clone();
                let expanded_nodes = &mut self.expanded_nodes;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(
                        RichText::new(format!("Format: {}", info.format))
                            .color(Color32::LIGHT_BLUE),
                    );
                    ui.separator();
                    for element in &info.structure {
                        Self::show_element(ui, element, 0, "", &search_text, expanded_nodes);
                    }
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.label("Drag and drop a media file here");
                    ui.label("or");
                    ui.label("use File -> Open to select a file");
                });
            }
        });
    }
}

impl MediaInspectorApp {
    fn show_element(
        ui: &mut egui::Ui,
        element: &ElementInfo,
        depth: usize,
        parent_path: &str,
        search_text: &str,
        expanded_nodes: &mut std::collections::HashSet<String>,
    ) {
        let path = format!("{}/{}", parent_path, element.name);
        let matches_search = search_text.is_empty()
            || element
                .name
                .to_lowercase()
                .contains(&search_text.to_lowercase())
            || element.properties.iter().any(|(k, v)| {
                k.to_lowercase().contains(&search_text.to_lowercase())
                    || v.to_lowercase().contains(&search_text.to_lowercase())
            });

        if !matches_search && element.children.is_empty() {
            return;
        }

        let indent = "    ".repeat(depth);
        let header_text = if matches_search {
            RichText::new(format!("{}{}", indent, element.name)).color(Color32::YELLOW)
        } else {
            RichText::new(format!("{}{}", indent, element.name))
        };

        let is_expanded = expanded_nodes.contains(&path);
        if egui::CollapsingHeader::new(header_text)
            .default_open(is_expanded || matches_search)
            .show(ui, |ui| {
                if matches_search {
                    ui.label(
                        RichText::new(format!("{}offset: {}", indent, element.offset))
                            .color(Color32::LIGHT_GRAY),
                    );
                    ui.label(
                        RichText::new(format!("{}size: {}", indent, element.size))
                            .color(Color32::LIGHT_GRAY),
                    );

                    for (key, value) in &element.properties {
                        let text = format!("{}{}: {}", indent, key, value);
                        if search_text.is_empty()
                            || text.to_lowercase().contains(&search_text.to_lowercase())
                        {
                            ui.label(RichText::new(text).color(Color32::LIGHT_GREEN));
                        }
                    }
                }

                for child in &element.children {
                    Self::show_element(ui, child, depth + 1, &path, search_text, expanded_nodes);
                }
            })
            .header_response
            .clicked()
        {
            if is_expanded {
                expanded_nodes.remove(&path);
            } else {
                expanded_nodes.insert(path);
            }
        }
    }

    fn collect_paths(
        element: &ElementInfo,
        parent_path: &str,
        paths: &mut std::collections::HashSet<String>,
    ) {
        let path = format!("{}/{}", parent_path, element.name);
        paths.insert(path.clone());
        for child in &element.children {
            Self::collect_paths(child, &path, paths);
        }
    }
}
