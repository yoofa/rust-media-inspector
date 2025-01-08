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
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(20.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(16.0, egui::FontFamily::Monospace),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            cc.egui_ctx.set_style(style);
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
    selected_element: Option<String>,
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
            selected_element: None,
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

        // 添加顶部菜单栏
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
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
                        if let Some(info) = &self.media_info {
                            let mut paths = std::collections::HashSet::new();
                            for element in &info.structure {
                                Self::collect_paths(element, "", &mut paths);
                            }
                            self.expanded_nodes = paths;
                        }
                        ui.close_menu();
                    }
                    if ui.button("Collapse All").clicked() {
                        self.expanded_nodes.clear();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // TODO: 显示关于对话框
                        ui.close_menu();
                    }
                });
            });
        });

        // 使用 SidePanel 和 CentralPanel 创建双栏布局
        egui::SidePanel::left("tree_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.search_text);
                });

                if let Some(info) = &self.media_info {
                    let search_text = self.search_text.clone();
                    let mut expanded_nodes = self.expanded_nodes.clone();
                    let mut selected_element = self.selected_element.clone();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading(
                            RichText::new(format!("Format: {}", info.format))
                                .color(Color32::LIGHT_BLUE),
                        );
                        ui.separator();
                        for element in &info.structure {
                            Self::show_element_tree(
                                ui,
                                element,
                                0,
                                "",
                                &search_text,
                                &mut expanded_nodes,
                                &mut selected_element,
                            );
                        }
                    });

                    // 更新状态
                    self.expanded_nodes = expanded_nodes;
                    self.selected_element = selected_element;
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.label("Drag and drop a media file here");
                        ui.label("or");
                        ui.label("use File -> Open to select a file");
                    });
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error_message {
                ui.colored_label(Color32::RED, err);
            } else if let Some(selected_path) = &self.selected_element {
                if let Some(info) = &self.media_info {
                    if let Some(element) = self.find_element(info, selected_path) {
                        self.show_element_details(ui, element);
                    }
                }
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.label("Select an element to view its details");
                });
            }
        });
    }
}

impl MediaInspectorApp {
    // 显示元素树（左侧面板）
    fn show_element_tree(
        ui: &mut egui::Ui,
        element: &ElementInfo,
        depth: usize,
        parent_path: &str,
        search_text: &str,
        expanded_nodes: &mut std::collections::HashSet<String>,
        selected_element: &mut Option<String>,
    ) {
        let path = format!("{}/{}", parent_path, element.name);
        let matches_search = search_text.is_empty()
            || element
                .name
                .to_lowercase()
                .contains(&search_text.to_lowercase());

        if !matches_search && element.children.is_empty() {
            return;
        }

        let indent = "    ".repeat(depth);
        let is_selected = Some(&path) == selected_element.as_ref();
        let header_text = if is_selected {
            RichText::new(format!("{}{}", indent, element.name))
                .color(Color32::YELLOW)
                .size(16.0)
        } else {
            RichText::new(format!("{}{}", indent, element.name)).size(16.0)
        };

        let is_expanded = expanded_nodes.contains(&path);

        // 使用完整路径作为基础 ID
        let base_id = ui.make_persistent_id(path.as_str());

        // 创建折叠头部
        let header = egui::CollapsingHeader::new(header_text)
            .id_source(base_id)
            .default_open(false) // 默认关闭
            .open(Some(is_expanded)); // 只有在展开集合中的才展开

        // 显示折叠头部和内容
        let header_response = header.show(ui, |ui| {
            // 显示子元素
            for (i, child) in element.children.iter().enumerate() {
                ui.push_id(i, |ui| {
                    Self::show_element_tree(
                        ui,
                        child,
                        depth + 1,
                        &path,
                        search_text,
                        expanded_nodes,
                        selected_element,
                    );
                });
            }
        });

        // 处理点击事件
        let header_rect = header_response.header_response.rect;
        let arrow_rect = egui::Rect::from_min_size(
            header_rect.min,
            egui::vec2(20.0, header_rect.height()), // 箭头区域宽度
        );

        // 点击箭头区域时处理展开/折叠
        if header_response.header_response.clicked() {
            let mouse_pos = ui.input(|i| i.pointer.hover_pos());
            if let Some(pos) = mouse_pos {
                if arrow_rect.contains(pos) {
                    if is_expanded {
                        expanded_nodes.remove(&path);
                    } else {
                        expanded_nodes.insert(path.clone());
                    }
                } else {
                    // 点击非箭头区域时更新选中元素
                    *selected_element = Some(path.clone());
                }
            }
        }
    }

    // 显示元素详情（右侧面板）
    fn show_element_details(&self, ui: &mut egui::Ui, element: &ElementInfo) {
        ui.push_id("details", |ui| {
            ui.heading(
                RichText::new(&element.name)
                    .color(Color32::LIGHT_BLUE)
                    .size(20.0),
            );
            ui.separator();

            ui.push_id("basic_info", |ui| {
                ui.group(|ui| {
                    ui.heading(RichText::new("Basic Information").size(18.0));
                    ui.label(
                        RichText::new(format!("Offset: {}", element.offset))
                            .color(Color32::LIGHT_GRAY)
                            .size(16.0),
                    );
                    ui.label(
                        RichText::new(format!("Size: {}", element.size))
                            .color(Color32::LIGHT_GRAY)
                            .size(16.0),
                    );
                });
            });

            if !element.properties.is_empty() {
                ui.push_id("properties", |ui| {
                    ui.group(|ui| {
                        ui.heading(RichText::new("Properties").size(18.0));
                        egui::Grid::new(ui.make_persistent_id("properties_grid"))
                            .striped(true)
                            .spacing([40.0, 8.0]) // 增加行间距
                            .show(ui, |ui| {
                                for (i, (key, value)) in element.properties.iter().enumerate() {
                                    ui.push_id(i, |ui| {
                                        ui.label(
                                            RichText::new(key)
                                                .color(Color32::LIGHT_GREEN)
                                                .size(16.0),
                                        );
                                        ui.label(RichText::new(value).size(16.0));
                                        ui.end_row();
                                    });
                                }
                            });
                    });
                });
            }

            if !element.children.is_empty() {
                ui.push_id("children", |ui| {
                    ui.group(|ui| {
                        ui.heading(RichText::new("Children").size(18.0));
                        for (i, child) in element.children.iter().enumerate() {
                            ui.push_id(i, |ui| {
                                ui.label(RichText::new(&child.name).size(16.0));
                            });
                        }
                    });
                });
            }
        });
    }

    // 查找指定路径的元素
    fn find_element<'a>(&self, info: &'a MediaInfo, path: &str) -> Option<&'a ElementInfo> {
        fn find_recursive<'a>(
            element: &'a ElementInfo,
            path_parts: &[&str],
            current_depth: usize,
        ) -> Option<&'a ElementInfo> {
            if current_depth >= path_parts.len() {
                return None;
            }

            if element.name == path_parts[current_depth] {
                if current_depth == path_parts.len() - 1 {
                    return Some(element);
                }
                for child in &element.children {
                    if let Some(found) = find_recursive(child, path_parts, current_depth + 1) {
                        return Some(found);
                    }
                }
            }
            None
        }

        let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        for element in &info.structure {
            if let Some(found) = find_recursive(element, &path_parts, 0) {
                return Some(found);
            }
        }
        None
    }

    // 添加辅助方法用于展开所有节点
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
