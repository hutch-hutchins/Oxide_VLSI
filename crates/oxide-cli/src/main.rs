mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Oxide_VLSI", about = "Oxide VLSI command-line tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run DRC on a project file
    Drc { project: std::path::PathBuf },
    /// Extract connectivity from a layout
    Extract { project: std::path::PathBuf },
    /// Export a project to GDS, SVG, or PNG
    Export {
        project: std::path::PathBuf,
        #[arg(long, value_delimiter = ',')]
        format: Vec<String>,
    },
    /// Generate a report for a project
    Report { project: std::path::PathBuf },
    /// Batch-grade student submissions against a lab spec
    Grade {
        submissions: std::path::PathBuf,
        #[arg(long)]
        lab: std::path::PathBuf,
    },
    /// Create a new project from a lab template
    New {
        name: String,
        #[arg(long)]
        template: Option<String>,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Drc { project } => commands::drc::run(&project),
        Commands::Extract { project } => commands::extract::run(&project),
        Commands::Export { project, format } => commands::export::run(&project, &format),
        Commands::Report { project } => commands::report::run(&project),
        Commands::Grade { submissions, lab } => commands::grade::run(&submissions, &lab),
        Commands::New { name, template } => commands::new::run(&name, template.as_deref()),
    }
}
