use clap::Parser;
use ob_plugin::{data::Plugin, error, extract::extract, success, util};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// Feat: support file name filter, like *.imgs, .gitignore
// Feat: support file content filter, like #tag
// Feat: reserve direcotry content file if it exits other than content
fn main() {
    let plugin = Plugin::parse();

    plugin
        .file_paths
        .par_iter()
        .for_each(|p| match extract(&plugin, p) {
            Ok(_) => success!("\"{}\" successed", util::reprot_path(p)),
            Err(e) => error!("\"{}\" failed, due to: {}", util::reprot_path(p), e),
        });
}
