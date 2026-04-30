use clap::Parser as _;
use dtk_pal::cli::{self, Cli};
use dtk_pal::config::Config;
use dtk_pal::splits::{File, Split, Splits};
use dtk_pal::symbols::ObjSymbol;
use eyre::{Context, ContextCompat, Result, bail};
use relative_path::RelativePath;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Splits(cli) => main_splits(cli),
    }
}

fn main_splits(cli: cli::Splits) -> Result<()> {
    match cli {
        cli::Splits::Add(cli) => main_splits_add(cli),
    }
}

fn main_splits_add(cli: cli::SplitsAdd) -> Result<()> {
    let config_dir = &cli.config_dir;
    let config = Config::load_from_dir(config_dir)?;
    let symbols = config.load_symbols(config_dir)?;
    let mut splits = config.load_splits(config_dir)?;

    let start_symbol = find_symbol(&symbols, &cli.start_symbol)?;
    let end_symbol = find_symbol(&symbols, &cli.end_symbol)?;

    if start_symbol.addr > end_symbol.addr {
        bail!("start symbol is located after end symbol")
    }

    let Some(start_section) = &start_symbol.section else {
        bail!("section of start symbol is unknown");
    };

    let Some(end_section) = &end_symbol.section else {
        bail!("section of end symbol is unknown");
    };

    if start_section != end_section {
        bail!("the symbols are not in the same section");
    }

    let section = start_section;
    let mut file = remove_splits_file(&mut splits, &cli.file_path) //
        .unwrap_or_else(|| File::new(cli.file_path));
    let mut split = remove_file_split(&mut file, section) //
        .unwrap_or_else(|| Split {
            section: section.clone(),
            start: 0,
            end: 0,
        });

    split.start = start_symbol.addr;
    split.end = end_symbol.addr;

    add_file_split(&mut file, split);
    add_splits_file(&mut splits, file);

    let splits_path = config.splits_path(config_dir)?;

    splits
        .save(&splits_path)
        .with_context(|| format!("failed to save splits to {splits_path:?}"))?;

    Ok(())
}

fn find_symbol<'a>(symbols: &'a [ObjSymbol], name: &str) -> Result<&'a ObjSymbol> {
    symbols
        .iter()
        .find(|symbol| symbol.name == name)
        .with_context(|| format!("symbol not found: {name:?}"))
}

fn remove_splits_file(splits: &mut Splits, file_path: &RelativePath) -> Option<File> {
    let position = splits
        .files
        .iter()
        .position(|file| file.path == file_path)?;

    Some(splits.files.remove(position))
}

fn remove_file_split(file: &mut File, section: &str) -> Option<Split> {
    let position = file
        .splits
        .iter()
        .position(|split| split.section == section)?;

    Some(file.splits.remove(position))
}

fn add_file_split(file: &mut File, split: Split) {
    file.splits.push(split);
    // file.splits.sort_by_key(|split| split.end);
}

fn add_splits_file(splits: &mut Splits, file: File) {
    splits.files.push(file);
    // splits.files.sort_by_key(|file| file.splits.first().map(|split| split.end));
}
