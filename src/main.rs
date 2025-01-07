mod analyzer;
mod console;
mod egui_gui;
mod error;
mod reader;

use analyzer::{DefaultAnalyzer, MediaAnalyzer};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file path
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Tree)]
    format: OutputFormat,

    /// Disable debug output (enabled by default)
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Tree view of the file structure
    Tree,
    /// JSON output
    Json,
    /// GUI mode
    Gui,
}

fn main() {
    let cli = Cli::parse();

    if cli.file.is_none() || cli.format == OutputFormat::Gui {
        std::process::exit(egui_gui::run_gui());
    }

    let analyzer = DefaultAnalyzer::new(!cli.quiet);
    match analyzer.analyze(cli.file.unwrap().to_str().unwrap()) {
        Ok(info) => match cli.format {
            OutputFormat::Tree => console::print_tree(&info),
            OutputFormat::Json => todo!(),
            OutputFormat::Gui => unreachable!(),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
