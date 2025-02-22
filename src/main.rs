mod analyzer;
mod console;
mod egui_gui;
mod error;
mod reader;

use clap::{Arg, Command};

#[derive(Debug)]
enum OutputMode {
    Console,
    Json,
    Gui,
}

impl From<&str> for OutputMode {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputMode::Json,
            "gui" => OutputMode::Gui,
            _ => OutputMode::Console,
        }
    }
}

fn main() {
    let matches = Command::new("Media Inspector")
        .version("1.0")
        .author("Your Name")
        .about("Analyzes media file formats")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_parser(["console", "json", "gui"])
                .default_value("gui")
                .help("Output mode (console/json/gui)"),
        )
        .arg(
            Arg::new("strategy")
                .short('s')
                .long("strategy")
                .value_parser(["auto", "extension", "content"])
                .default_value("auto")
                .help("File format detection strategy"),
        )
        .arg(
            Arg::new("FILE")
                .help("Input file to analyze")
                .required(false),
        )
        .get_matches();

    // 获取输出模式和策略
    let output_mode: OutputMode = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("gui")
        .into();

    let strategy = matches.get_one::<String>("strategy").unwrap();
    let file_path = matches.get_one::<String>("FILE");

    match (output_mode, file_path) {
        // 无文件参数，启动 GUI
        (_, None) => {
            std::process::exit(egui_gui::run_gui_with_options(strategy));
        }
        // GUI 模式且有文件
        (OutputMode::Gui, Some(path)) => {
            std::process::exit(egui_gui::run_gui_with_file(path, strategy));
        }
        // 控制台模式且有文件
        (OutputMode::Console, Some(path)) => {
            std::process::exit(console::run_console_with_file(path, strategy));
        }
        // JSON 模式且有文件
        (OutputMode::Json, Some(path)) => {
            std::process::exit(console::run_console_with_json(path, strategy));
        }
    }
}
