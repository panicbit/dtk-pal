use relative_path::RelativePathBuf;

mod parser;

pub use parser::splits as parser;

#[derive(Debug)]
pub struct Splits {
    pub sections: Vec<Section>,
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub path: RelativePathBuf,
    pub splits: Vec<Split>,
}

#[derive(Debug)]
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
