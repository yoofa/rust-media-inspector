mod analyzer;
mod console;
mod egui_gui;
mod error;
mod reader;

use analyzer::{DefaultAnalyzer, MediaAnalyzer};
use clap::Command;

fn main() {
    let matches = Command::new("Media Analyzer")
        .version("1.0")
        .author("Your Name")
        .about("Analyzes media file formats")
        .arg(
            clap::Arg::new("gui")
                .short('g')
                .long("gui")
                .help("Launch GUI mode"),
        )
        .arg(
            clap::Arg::new("strategy")
                .short('s')
                .long("strategy")
                .value_parser(["auto", "extension", "content"])
                .default_value("auto")
                .help("File format detection strategy"),
        )
        .arg(
            clap::Arg::new("FILE")
                .help("Input file to analyze")
                .required_unless_present("gui"),
        )
        .get_matches();

    if matches.get_flag("gui") {
        std::process::exit(egui_gui::run_gui());
    } else {
        let file_path = matches.get_one::<String>("FILE").unwrap();
        let strategy = match matches.get_one::<String>("strategy").unwrap().as_str() {
            "auto" => analyzer::detector::DetectionStrategy::Auto,
            "extension" => analyzer::detector::DetectionStrategy::Extension,
            "content" => analyzer::detector::DetectionStrategy::Content,
            _ => analyzer::detector::DetectionStrategy::Auto,
        };

        match console::analyze_file(file_path, strategy) {
            Ok(info) => console::print_tree(&info),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
