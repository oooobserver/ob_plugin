use clap::Parser;
use ob_plugin::{error, extract::extract, success, util};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Parser)]
#[command(name = "ob")]
#[command(version = "1.0")]
#[command(about = "Obsidian plugin to extract md content", long_about = None)]
struct Plugin {
    #[arg(short, long, value_name = "depth", default_value = "7")]
    depth: usize,
    // When the path is the directory, if extract all file in that dir
    #[arg(short, long, value_name = "recursive", default_value = "false")]
    recursive: bool,

    file_paths: Vec<String>,
}

/*
   Depth:
   2: just match H2
   3: H3
   4: H4
   (So on and so forth)...
*/
//NOTE: Delete previous extracted content may cause un-expected damage, please check your file before use
fn main() {
    let plugin = Plugin::parse();

    plugin
        .file_paths
        .par_iter()
        .for_each(|p| match extract(p, plugin.depth, plugin.recursive) {
            Ok(_) => success!("\"{}\" successed", util::reprot_path(p)),
            Err(e) => error!("\"{}\" failed, due to: {}", util::reprot_path(p), e),
        });
}
