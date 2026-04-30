use clap::{Args, Parser};
use relative_path::RelativePathBuf;
use std::path::PathBuf;

#[derive(Parser)]
pub enum Cli {
    #[clap(subcommand)]
    Splits(Splits),
}

#[derive(Parser)]
pub enum Splits {
    Add(SplitsAdd),
}

#[derive(Args)]
pub struct SplitsAdd {
    pub config_dir: PathBuf,
    pub file_path: RelativePathBuf,
    pub start_symbol: String,
    pub end_symbol: String,
}
