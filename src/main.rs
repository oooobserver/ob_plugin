use clap::Parser;
use ob_plugin::{data::Plugin, dispatch, error, success, util};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// Feat: print recent update file or write file and while print remove the suffix specified by path(add base path)
// Feat: support file name filter, like *.imgs, .gitignore
// Feat: reserve direcotry content file if it exits other than content
fn main() {
    let mut plugin = Plugin::parse();
    plugin.file_paths = util::normalize_paths(plugin.file_paths);

    plugin
        .file_paths
        .par_iter()
        .for_each(|p| match dispatch(&plugin, p) {
            Ok(_) => success!("\"{}\" successed", util::reprot_path(p)),
            Err(e) => error!("\"{}\" failed, due to: {}", util::reprot_path(p), e),
        });
}
