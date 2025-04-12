use clap::{Parser, ValueEnum};
use std::fmt;

/*
   Depth:
   2: just match H2
   3: H3
   4: H4
   (So on and so forth)...
*/
#[derive(Parser)]
#[command(name = "ob")]
#[command(version = "1.0")]
#[command(about = "Obsidian plugin to extract md content", long_about = None)]
pub struct Plugin {
    #[arg(short, long, value_name = "depth", default_value = "7")]
    pub depth: usize,
    // When the path is the directory, if extract all file in that dir
    #[arg(short, long, value_name = "recursive", default_value = "false")]
    pub recursive: bool,

    #[arg(short, long, value_name = "sort", default_value_t = SortOrder::Alphabet)]
    pub sort: SortOrder,

    pub file_paths: Vec<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortOrder {
    Alphabet,
    UpdateTime,
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SortOrder::Alphabet => "alphabet",
            SortOrder::UpdateTime => "update time",
        };
        write!(f, "{}", s)
    }
}
