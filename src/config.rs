use std::fs;
use std::path::{Path, PathBuf};

use chumsky::Parser;
use eyre::{Context, ContextCompat, Result, bail};
use relative_path::RelativePathBuf;
use serde::Deserialize;

use crate::splits::{self, Splits};
use crate::symbols::{self, ObjSymbol};
use crate::util::parsing;

#[derive(Deserialize)]
pub struct Config {
    pub symbols: RelativePathBuf,
    pub splits: RelativePathBuf,
}

impl Config {
    pub fn load_from_dir(config_dir: impl AsRef<Path>) -> Result<Self> {
        let config_path = config_dir.as_ref().join("config.yml");
        let config = fs::read_to_string(&config_path)
            .with_context(|| format!("failed to read {config_path:?}"))?;
        let config = yaml_serde::from_str::<Self>(&config)
            .with_context(|| format!("failed to parse {config_path:?}"))?;

        Ok(config)
    }

    pub fn symbols_path(&self, config_dir: impl AsRef<Path>) -> Result<PathBuf> {
        let project_dir = project_dir_from_config_dir(&config_dir)?;

        Ok(self.symbols.to_path(project_dir))
    }

    pub fn splits_path(&self, config_dir: impl AsRef<Path>) -> Result<PathBuf> {
        let project_dir = project_dir_from_config_dir(&config_dir)?;

        Ok(self.splits.to_path(project_dir))
    }

    pub fn load_symbols(&self, config_dir: impl AsRef<Path>) -> Result<Vec<ObjSymbol>> {
        let symbols_path = self.symbols_path(config_dir)?;
        let symbols_raw = fs::read_to_string(&symbols_path)
            .with_context(|| format!("failed to read {symbols_path:?}"))?;

        let symbols = match symbols::parser().parse(&symbols_raw).into_result() {
            Ok(symbols) => symbols,
            Err(errs) => {
                parsing::print_errors(&errs, &symbols_raw)?;
                bail!("failed to parse symbols: {symbols_path:?}");
            }
        };

        Ok(symbols)
    }

    pub fn load_splits(&self, config_dir: impl AsRef<Path>) -> Result<Splits> {
        let splits_path = self.splits_path(config_dir)?;
        let splits_raw = fs::read_to_string(&splits_path)
            .with_context(|| format!("failed to read {splits_path:?}"))?;

        let splits = match splits::parser().parse(&splits_raw).into_result() {
            Ok(splits) => splits,
            Err(errs) => {
                parsing::print_errors(&errs, &splits_raw)?;
                bail!("failed to parse splits: {splits_path:?}");
            }
        };

        Ok(splits)
    }
}

fn project_dir_from_config_dir<P>(config_dir: &P) -> Result<&Path>
where
    P: AsRef<Path>,
{
    config_dir
        .as_ref()
        .parent()
        .and_then(Path::parent)
        .with_context(|| {
            format!(
                "failed to determine project directory for {:?}",
                config_dir.as_ref()
            )
        })
}
