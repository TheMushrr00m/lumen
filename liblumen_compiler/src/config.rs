use std::collections::VecDeque;
use std::convert::Into;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use super::errors::{self, CompilerError};

use libeir_diagnostics::{CodeMap, ColorChoice};
use libeir_syntax_erl::ParseConfig;

/// Determines which type of compilation to perform,
/// either parsing modules from BEAM files, or by
/// parsing modules from Erlang source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum FileType {
    Erlang,
}
impl FromStr for FileType {
    type Err = errors::CompilerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "erl" => Ok(FileType::Erlang),
            _ => Err(CompilerError::FileType(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Debug,
    Info,
    Warning,
    Error,
    Silent,
}
impl Verbosity {
    pub fn from_level(level: isize) -> Self {
        if level < 0 {
            return Verbosity::Silent;
        }

        match level {
            0 => Verbosity::Warning,
            1 => Verbosity::Info,
            _ => Verbosity::Debug,
        }
    }
}

/// This structure holds all top-level compiler options
/// and configuration; it is passed through all phases
/// of compilation
#[derive(Debug, Clone)]
pub struct CompilerSettings {
    pub file_type: FileType,
    pub color: ColorChoice,
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    //pub defines: HashMap<Symbol, MacroDef>,
    pub warnings_as_errors: bool,
    pub no_warn: bool,
    pub verbosity: Verbosity,
    pub code_path: Vec<PathBuf>,
    pub include_path: VecDeque<PathBuf>,
    pub codemap: Arc<Mutex<CodeMap>>,
}
impl Into<ParseConfig> for CompilerSettings {
    fn into(self) -> ParseConfig {
        ParseConfig {
            codemap: self.codemap.clone(),
            warnings_as_errors: self.warnings_as_errors,
            no_warn: self.no_warn,
            code_paths: self.code_path.clone().into(),
            include_paths: self.include_path.clone(),
            macros: None,
        }
    }
}
