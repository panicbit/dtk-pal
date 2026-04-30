use std::fs;
use std::io::Write;
use std::path::Path;

use eyre::{Context, Result};
use relative_path::RelativePathBuf;

mod parser;

pub use parser::splits as parser;

#[derive(Debug)]
pub struct Splits {
    pub sections: Vec<Section>,
    pub files: Vec<File>,
}

impl Splits {
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut data = Vec::new();

        self.write(&mut data).context("failed to format splits")?;
        fs::write(path, data).context("failed to write splits")?;

        Ok(())
    }

    pub fn write<W>(&self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        let longest_section_name = self
            .sections
            .iter()
            .map(|section| section.name.len())
            .max()
            .unwrap_or_default();
        let colum_width = longest_section_name + 2;

        writeln!(w, "Sections:")?;

        for section in &self.sections {
            write!(w, "\t{}", section.name)?;

            let required_padding = colum_width - section.name.len();

            for _ in 0..required_padding {
                write!(w, " ")?;
            }

            writeln!(
                w,
                "type:{type} align:{align}",
                r#type = section.r#type,
                align = section.align,
            )?;
        }

        writeln!(w)?;

        for file in &self.files {
            writeln!(w, "{}:", file.path)?;

            for split in &file.splits {
                write!(w, "\t{}", split.section)?;

                let required_padding = colum_width - split.section.len();

                for _ in 0..required_padding {
                    write!(w, " ")?;
                }

                writeln!(
                    w,
                    "start:0x{start:08X} end:0x{end:08X}",
                    start = split.start,
                    end = split.end,
                )?;
            }

            writeln!(w)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: RelativePathBuf,
    pub splits: Vec<Split>,
}

impl File {
    pub fn new(path: RelativePathBuf) -> Self {
        Self {
            path,
            splits: Vec::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Split {
    pub section: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub r#type: String,
    pub align: u32,
}
