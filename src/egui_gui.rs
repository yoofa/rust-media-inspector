use eframe::{egui, NativeOptions};
use egui::{Color32, RichText, ViewportBuilder};
use rfd::FileDialog;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::analyzer::detector::DetectionStrategy;
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

pub fn run_gui_with_options(strategy: &str) -> i32 {
    // 将 strategy 转换为 owned 类型
    let strategy = strategy.to_string();

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
        Box::new(move |_cc| {
            let mut app = MediaInspectorApp::default();
            app.detection_strategy = strategy.as_str().into();
            Box::new(app)
        }),
    )
    .map_or(-1, move |_| 0)
}

pub fn run_gui_with_file(file_path: &str, strategy: &str) -> i32 {
    // 将参数转换为 owned 类型
    let file_path = file_path.to_string();
    let strategy = strategy.to_string();

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
        Box::new(move |_cc| {
            let mut app = MediaInspectorApp::default();
            // 设置策略
            app.detection_strategy = strategy.as_str().into();
            // 立即开始分析文件
            let tx = app.tx.clone();
            let path_str = file_path.clone();
            let strategy = app.detection_strategy;
            std::thread::spawn(move || {
                let analyzer = DefaultAnalyzer::with_strategy(true, strategy);
                let result = analyzer.analyze(&path_str).map_err(|e| e.to_string());
                tx.send(result).ok();
            });
            Box::new(app)
        }),
    )
    .map_or(-1, move |_| 0)
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
    detection_strategy: DetectionStrategy,
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
            detection_strategy: DetectionStrategy::Auto,
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

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 文件操作按钮组
                ui.group(|ui| {
                    if ui.button(RichText::new("📂 Open").size(16.0)).clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            let tx = self.tx.clone();
                            let path_str = path.to_str().unwrap().to_string();
                            let strategy = self.detection_strategy;

                            // 在新线程中分析文件
                            std::thread::spawn(move || {
                                let analyzer = DefaultAnalyzer::with_strategy(true, strategy);
                                let result = analyzer.analyze(&path_str).map_err(|e| e.to_string());
                                tx.send(result).ok();
                            });
                        }
                    }
                    if ui.button(RichText::new("🔄 Reload").size(16.0)).clicked() {
                        // ... 重新加载当前文件 ...
                    }
                });

                ui.separator();

                // 视图控制按钮组
                ui.group(|ui| {
                    if ui
                        .button(RichText::new("🔍 Expand All").size(16.0))
                        .clicked()
                    {
                        if let Some(info) = &self.media_info {
                            let mut paths = std::collections::HashSet::new();
                            Self::collect_all_paths(&info.structure, &mut paths);
                            self.expanded_nodes = paths;
                        }
                    }
                    if ui
                        .button(RichText::new("🔽 Collapse All").size(16.0))
                        .clicked()
                    {
                        self.expanded_nodes.clear();
                    }
                });

                ui.separator();

                // 策略选择
                ui.group(|ui| {
                    ui.label(RichText::new("Detection:").size(16.0));
                    egui::ComboBox::from_id_source("strategy")
                        .selected_text(format!("{:?}", self.detection_strategy))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.detection_strategy,
                                DetectionStrategy::Auto,
                                RichText::new("Auto").size(16.0),
                            );
                            ui.selectable_value(
                                &mut self.detection_strategy,
                                DetectionStrategy::Extension,
                                RichText::new("Extension").size(16.0),
                            );
                            ui.selectable_value(
                                &mut self.detection_strategy,
                                DetectionStrategy::Content,
                                RichText::new("Content").size(16.0),
                            );
                        });
                });

                // 搜索框
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search_text)
                            .hint_text("🔍 Search...")
                            .desired_width(200.0)
                            .font(egui::TextStyle::Monospace),
                    );
                });
            });
        });

        // 检查分析结果
        if let Ok(result) = self.rx.try_recv() {
            match result {
                Ok(info) => {
                    self.media_info = Some(info);
                    self.error_message = None;
                }
                Err(err) => {
                    self.media_info = None;
                    self.error_message = Some(err);
                }
            }
        }

        // 显示错误信息
        if let Some(error) = &self.error_message {
            let error_message = error.clone();
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.colored_label(Color32::RED, &error_message);
                    if ui.button("Close").clicked() {
                        self.error_message = None;
                    }
                });
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
                        for (i, element) in info.structure.iter().enumerate() {
                            Self::show_element_tree(
                                ui,
                                element,
                                0,
                                "",
                                0,
                                i,
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
    // 添加颜色数组作为常量
    const LEVEL_COLORS: [Color32; 6] = [
        Color32::from_rgb(87, 204, 153),  // 浅绿
        Color32::from_rgb(92, 179, 255),  // 浅蓝
        Color32::from_rgb(255, 179, 71),  // 橙色
        Color32::from_rgb(255, 145, 164), // 粉红
        Color32::from_rgb(179, 157, 219), // 紫色
        Color32::from_rgb(255, 214, 102), // 金黄
    ];

    // 显示元素树（左侧面板）
    fn show_element_tree(
        ui: &mut egui::Ui,
        element: &ElementInfo,
        depth: usize,
        parent_path: &str,
        parent_index: usize,
        index: usize,
        search_text: &str,
        expanded_nodes: &mut std::collections::HashSet<String>,
        selected_element: &mut Option<String>,
    ) {
        let display_path = format!("{}/{}", parent_path, element.name);
        let unique_path = format!("{}#{}_{}", parent_path, parent_index, index);

        let matches_search = search_text.is_empty()
            || element
                .name
                .to_lowercase()
                .contains(&search_text.to_lowercase());

        if !matches_search && element.children.is_empty() {
            return;
        }

        let indent = "    ".repeat(depth);
        let is_selected = Some(&display_path) == selected_element.as_ref();

        // 根据深度选择颜色
        let color = if is_selected {
            Color32::YELLOW
        } else if !search_text.is_empty() && matches_search {
            Color32::from_rgb(255, 128, 0) // 搜索匹配项使用醒目的橙色
        } else {
            Self::LEVEL_COLORS[depth % Self::LEVEL_COLORS.len()]
        };

        // 简化显示文本，移除图标
        let display_text = if element.children.is_empty() {
            format!("{}{}", indent, element.name)
        } else {
            format!("{}{} ({})", indent, element.name, element.children.len())
        };

        let text = RichText::new(display_text).size(16.0).color(color);

        let base_id = ui.make_persistent_id(&unique_path);

        if element.children.is_empty() {
            let response = ui.add(egui::Label::new(text).sense(egui::Sense::click()));
            if response.clicked() {
                *selected_element = Some(display_path);
            }
        } else {
            let is_expanded = expanded_nodes.contains(&unique_path);

            let header = egui::CollapsingHeader::new(text)
                .id_source(base_id)
                .default_open(false)
                .open(Some(is_expanded));

            let header_response = header.show(ui, |ui| {
                for (child_index, child) in element.children.iter().enumerate() {
                    ui.push_id(child_index, |ui| {
                        Self::show_element_tree(
                            ui,
                            child,
                            depth + 1,
                            &display_path,
                            index,
                            child_index,
                            search_text,
                            expanded_nodes,
                            selected_element,
                        );
                    });
                }
            });

            let header_rect = header_response.header_response.rect;
            let is_clicked = header_response.header_response.clicked();

            let arrow_rect =
                egui::Rect::from_min_size(header_rect.min, egui::vec2(20.0, header_rect.height()));

            if is_clicked {
                let mouse_pos = ui.input(|i| i.pointer.hover_pos());
                if let Some(pos) = mouse_pos {
                    if arrow_rect.contains(pos) {
                        if is_expanded {
                            expanded_nodes.remove(&unique_path);
                        } else {
                            expanded_nodes.insert(unique_path);
                        }
                    } else {
                        *selected_element = Some(display_path);
                    }
                }
            }
        }
    }

    // 显示元素详情（右侧面板）
    fn show_element_details(&self, ui: &mut egui::Ui, element: &ElementInfo) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.push_id("details", |ui| {
                // 标题
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new(&element.name)
                            .color(Color32::LIGHT_BLUE)
                            .size(24.0),
                    );
                });
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(16.0);

                // 基本信息
                ui.push_id("basic_info", |ui| {
                    ui.group(|ui| {
                        ui.heading(RichText::new("Basic Information").size(20.0));
                        ui.add_space(8.0);

                        // 使用 Grid 来显示基本信息
                        egui::Grid::new("basic_info_grid")
                            .num_columns(2)
                            .spacing([40.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Offset").strong().size(16.0));
                                if let Ok(offset) = element.offset.parse::<u64>() {
                                    ui.label(
                                        RichText::new(format!("{:#x} ({} bytes)", offset, offset))
                                            .monospace()
                                            .size(16.0),
                                    );
                                } else {
                                    ui.label(RichText::new(&element.offset).monospace().size(16.0));
                                }
                                ui.end_row();

                                ui.label(RichText::new("Size").strong().size(16.0));
                                if let Ok(size) = element.size.parse::<u64>() {
                                    ui.label(
                                        RichText::new(format!("{:#x} ({} bytes)", size, size))
                                            .monospace()
                                            .size(16.0),
                                    );
                                } else {
                                    ui.label(RichText::new(&element.size).monospace().size(16.0));
                                }
                                ui.end_row();
                            });
                    });
                });
                ui.add_space(16.0);

                // 属性信息
                if !element.properties.is_empty() {
                    ui.push_id("properties", |ui| {
                        ui.group(|ui| {
                            ui.heading(RichText::new("Properties").size(20.0));
                            ui.add_space(8.0);

                            // 使用 Grid 来显示属性
                            egui::Grid::new("properties_grid")
                                .num_columns(2)
                                .spacing([40.0, 8.0])
                                .show(ui, |ui| {
                                    for prop in &element.properties {
                                        // 属性名（左列）
                                        ui.label(
                                            RichText::new(&prop.name)
                                                .color(Color32::LIGHT_GREEN)
                                                .strong()
                                                .size(16.0),
                                        );

                                        // 属性值（右列）
                                        if prop.value.contains('\n') {
                                            // 多行值使用代码块显示
                                            egui::Frame::none()
                                                .fill(Color32::from_rgb(30, 30, 30))
                                                .inner_margin(8.0)
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(&prop.value)
                                                            .monospace()
                                                            .size(16.0)
                                                            .color(Color32::LIGHT_GRAY),
                                                    );
                                                });
                                        } else {
                                            // 单行值直接显示
                                            ui.label(
                                                RichText::new(&prop.value)
                                                    .monospace()
                                                    .size(16.0)
                                                    .color(Color32::LIGHT_GRAY),
                                            );
                                        }
                                        ui.end_row();
                                    }
                                });
                        });
                    });
                    ui.add_space(16.0);
                }

                // 子元素信息
                if !element.children.is_empty() {
                    ui.push_id("children", |ui| {
                        ui.group(|ui| {
                            ui.heading(RichText::new("Children").size(20.0));
                            ui.add_space(8.0);

                            for child in &element.children {
                                ui.horizontal(|ui| {
                                    ui.label("•"); // 添加项目符号
                                    ui.label(
                                        RichText::new(&child.name)
                                            .size(16.0)
                                            .color(Color32::LIGHT_GRAY),
                                    );
                                    ui.label(
                                        RichText::new(format!("(size: {} bytes)", child.size))
                                            .size(14.0)
                                            .weak(),
                                    );
                                });
                            }
                        });
                    });
                }
            });
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

    fn collect_all_paths(elements: &[ElementInfo], paths: &mut std::collections::HashSet<String>) {
        for element in elements {
            Self::collect_paths(element, "", paths);
        }
    }

    fn display_structure(&mut self, ui: &mut egui::Ui, elements: &[ElementInfo]) {
        for element in elements {
            let id = format!("{}_{}", element.name, element.offset);
            let is_expanded = self.expanded_nodes.contains(&id);

            // 构建显示文本
            let mut text = element.name.clone();
            if !element.readable_value.is_empty() {
                text = format!("{} ({})", text, element.readable_value);
            }

            // 添加大小信息
            if let Ok(size) = element.size.parse::<u64>() {
                text = format!("{} [{} bytes]", text, size);
            }

            let header = egui::CollapsingHeader::new(RichText::new(text).size(16.0))
                .id_source(&id)
                .default_open(is_expanded)
                .show(ui, |ui| {
                    // 显示基本信息
                    ui.add_space(4.0);
                    egui::Grid::new(&format!("grid_{}", id))
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .show(ui, |ui| {
                            // 显示偏移量
                            ui.label(RichText::new("Offset:").strong());
                            if let Ok(offset) = element.offset.parse::<u64>() {
                                ui.label(
                                    RichText::new(format!("0x{:x} ({} bytes)", offset, offset))
                                        .monospace(),
                                );
                            } else {
                                ui.label(RichText::new(&element.offset).monospace());
                            }
                            ui.end_row();

                            // 显示大小
                            ui.label(RichText::new("Size:").strong());
                            if let Ok(size) = element.size.parse::<u64>() {
                                ui.label(
                                    RichText::new(format!("0x{:x} ({} bytes)", size, size))
                                        .monospace(),
                                );
                            } else {
                                ui.label(RichText::new(&element.size).monospace());
                            }
                            ui.end_row();

                            // 显示属性
                            for prop in &element.properties {
                                ui.label(RichText::new(&prop.name).strong());
                                if prop.value != prop.readable_value {
                                    ui.label(
                                        RichText::new(format!(
                                            "{} ({})",
                                            prop.value, prop.readable_value
                                        ))
                                        .monospace(),
                                    );
                                } else {
                                    ui.label(RichText::new(&prop.value).monospace());
                                }
                                ui.end_row();
                            }
                        });

                    ui.add_space(8.0);

                    // 递归显示子元素
                    if !element.children.is_empty() {
                        ui.group(|ui| {
                            self.display_structure(ui, &element.children);
                        });
                    }
                });

            // 更新展开状态
            if header.header_response.clicked() {
                if is_expanded {
                    self.expanded_nodes.remove(&id);
                } else {
                    self.expanded_nodes.insert(id);
                }
            }
        }
    }

    // 添加搜索过滤功能
    fn should_show_element(&self, element: &ElementInfo) -> bool {
        if self.search_text.is_empty() {
            return true;
        }

        let search_text = self.search_text.to_lowercase();

        // 检查名称
        if element.name.to_lowercase().contains(&search_text) {
            return true;
        }

        // 检查属性
        for prop in &element.properties {
            if prop.name.to_lowercase().contains(&search_text)
                || prop.value.to_lowercase().contains(&search_text)
                || prop.readable_value.to_lowercase().contains(&search_text)
            {
                return true;
            }
        }

        // 递归检查子元素
        element
            .children
            .iter()
            .any(|child| self.should_show_element(child))
    }
}
