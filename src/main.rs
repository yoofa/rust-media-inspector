mod analyzer;
mod console;
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
    file: PathBuf,

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
}

fn main() {
    let cli = Cli::parse();

    let analyzer = DefaultAnalyzer::new(!cli.quiet);
    match analyzer.analyze(cli.file.to_str().unwrap()) {
        Ok(info) => match cli.format {
            OutputFormat::Tree => console::print_tree(&info),
            OutputFormat::Json => todo!(),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
