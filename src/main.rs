mod file_tree;
mod print_tree;
mod size;

use clap::{Parser, Subcommand};
use file_tree::FileTree;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Sort the tree lexicographically by path
    #[arg(short, long)]
    lexicographic_sort: bool,

    /// Filter nodes by extension
    #[arg(short, long)]
    filter: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the disk usage tree for the given path
    Usage {
        /// (default '.')
        path: Option<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Usage { path } => {
            let path = path.as_deref().unwrap_or(Path::new("."));
            FileTree::new(path)
                .unwrap()
                .show(cli.lexicographic_sort, cli.filter.as_deref());
        }
    }
    Ok(())
}
