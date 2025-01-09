mod analyzer;
mod console;
mod egui_gui;
mod error;
mod reader;

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
        std::process::exit(console::run_console(&matches));
    }
}
