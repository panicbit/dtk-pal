use enumset::{EnumSet, EnumSetType};
use eyre::{Result, bail};
use serde::Serialize;

use crate::util::serde::ser_enumset;

mod parser;

pub use parser::symbols as parser;

#[derive(Debug, Serialize)]
pub struct ObjSymbol<'a> {
    name: &'a str,
    section: Option<&'a str>,
    addr: &'a str,
    attrs: Attrs,
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct Attrs {
    r#type: ObjSymbolKind,
    #[serde(serialize_with = "ser_enumset")]
    flags: EnumSet<ObjSymbolFlag>,
    size: Option<u32>,
    data: ObjDataKind,
    hidden: bool,
    align: u32,
}

#[derive(Debug, Copy, Clone, Serialize, Default)]
pub enum ObjSymbolKind {
    #[default]
    Unknown,
    Function,
    Object,
    Section,
}

impl ObjSymbolKind {
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "label" => Self::Unknown,
            "function" => Self::Function,
            "object" => Self::Object,
            "section" => Self::Section,
            _ => bail!("unknown symbol kind: {s:?}"),
        })
    }
}

#[derive(Debug, Serialize, EnumSetType)]
pub enum ObjSymbolFlag {
    Common,
    Weak,
    Global,
    Local,
}

impl ObjSymbolFlag {
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "common" => ObjSymbolFlag::Common,
            "weak" => ObjSymbolFlag::Weak,
            "global" => ObjSymbolFlag::Global,
            "local" => ObjSymbolFlag::Local,
            _ => bail!("unknown symbol scope: {s:?}"),
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize, Default)]
pub enum ObjDataKind {
    #[default]
    Unknown,
    Byte,
    Byte2,
    Byte4,
    Byte8,
    Float,
    Double,
    String,
    ShiftJIS,
    String16,
    StringTable,
    ShiftJISTable,
    String16Table,
    Int,
    Short,
}

impl ObjDataKind {
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "byte" => Self::Byte,
            "2byte" => Self::Byte2,
            "4byte" => Self::Byte4,
            "8byte" => Self::Byte8,
            "float" => Self::Float,
            "double" => Self::Double,
            "string" => Self::String,
            "sjis" => Self::ShiftJIS,
            "wstring" => Self::String16,
            "string_table" => Self::StringTable,
            "sjis_table" => Self::ShiftJISTable,
            "wstring_table" => Self::String16Table,
            "int" => Self::Int,
            "short" => Self::Short,
            _ => bail!("unknown obj data type: {s:?}"),
        })
    }
}
