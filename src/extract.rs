use crate::{
    data::Plugin,
    util::{self, check_delete_previous_content, check_path_extention},
    RE,
};
use std::{
    error::Error,
    ffi::OsStr,
    fs::{self, DirEntry},
    path::Path,
};

const FILE_TEMPLATE: &str = "
## Content
---
";

const DIR_TEMPLATE: &str = "
---
";

pub fn extract(plugin: &Plugin, path: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        // Check the file extention
        if !util::check_path_extention(path) {
            return Err("The file must be the mark down file".into());
        }
        extract_file(plugin, path)?
    } else {
        extract_dir(plugin, path)?
    }
    Ok(())
}

fn extract_file(plugin: &Plugin, path: &str) -> Result<(), Box<dyn Error>> {
    let raw_content = fs::read_to_string(path)?;
    let content = check_delete_previous_content(&raw_content);

    let titles = extract_file_titles(plugin, path)?;
    let mut res = FILE_TEMPLATE.to_owned();
    for (l, n) in titles.iter() {
        let row = gen_content_row(None, l, n, true);
        res.push_str(&row);
    }

    res.push_str(content);

    fs::write(path, res)?;
    Ok(())
}

// If this is used for the file, add `#` char to indexing
fn gen_content_row(base_path: Option<&OsStr>, level: &usize, name: &str, if_file: bool) -> String {
    let mut row = String::new();
    for _ in 0..level - 2 {
        row.push('\t');
    }

    let res = if if_file {
        format!("* [[#{}]]\n", name)
    } else {
        // directory
        if let Some(path) = base_path {
            let path_name = path.to_str().expect("convert to string failed");
            format!("* [[{}|{}]]\n", path_name, name)
        } else {
            format!("* [[{}]]\n", name)
        }
    };

    row.push_str(&res);
    row
}

fn gen_dir_name(level: &usize, name: &str) -> String {
    let mut row = String::new();
    for _ in 0..level - 2 {
        row.push('\t');
    }

    let res = format!("* **{}:**\n", name);
    row.push_str(&res);
    row
}

fn extract_dir(plugin: &Plugin, path: &str) -> Result<(), Box<dyn Error>> {
    let mut res = DIR_TEMPLATE.to_owned();
    // Only get the last segment of the path
    let base_path = Path::new(path)
        .file_name()
        .expect("get directory name error");

    let entries = fs::read_dir(path).expect("read dir error, previous has validate");
    let entries = util::sort_filter_entries(entries, plugin.sort);
    for e in entries {
        extract_dir_helper(plugin, base_path, &e, 2, &mut res)?
    }

    let mut path = path.to_owned();
    path.push_str("/Content.md");

    fs::File::create(&path)?;
    fs::write(&path, res)?;
    Ok(())
}

fn extract_dir_helper(
    plugin: &Plugin,
    base_path: &OsStr,
    e: &DirEntry,
    level: usize,
    res: &mut String,
) -> Result<(), Box<dyn Error>> {
    let file_name = e
        .file_name()
        .into_string()
        .expect("convert to string failed");

    let mut new_base_path = Path::new(base_path).to_path_buf();
    new_base_path.push(&file_name);
    let new_base_path = new_base_path.as_os_str();

    if file_name == "imgs" {
        return Ok(());
    }

    if e.file_type()
        .unwrap_or_else(|_| panic!("can't get the file type: {:#?}", e.file_name()))
        .is_dir()
    {
        res.push_str(&gen_dir_name(&level, &file_name));

        let entries = fs::read_dir(e.path()).expect("read dir error, previous has validate");
        let entries = util::sort_filter_entries(entries, plugin.sort);
        for e in entries {
            extract_dir_helper(plugin, new_base_path, &e, level + 1, res)?
        }
    } else {
        // Check if this file is `.md` file
        if !check_path_extention(&file_name) {
            return Ok(());
        }

        // If recursive, generate each file's content
        if plugin.recursive {
            let path = e
                .path()
                .into_os_string()
                .into_string()
                .expect("convert to string failed");
            println!("{}", path);
            extract_file(plugin, &path)?;
        }

        let row = gen_content_row(
            Some(new_base_path),
            &level,
            file_name.trim_end_matches(".md"),
            false,
        );
        res.push_str(&row);
    }

    Ok(())
}

// Extract the depth and the title, when there is a depth gap, auto increment the depth
// example: 2,4,4,5 -> 2,3,3,4
fn extract_file_titles(
    plugin: &Plugin,
    path: &str,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let mut res = RE
        .find_iter(&content)
        .map(|s| util::parse_title(s.as_str()))
        // The title can't be the content
        .filter(|(l, n)| l <= &plugin.depth && n != "Content")
        // Get rid of the bold syntax like this: "**xxxx**"
        .map(|(l, s)| {
            (
                l,
                s.trim_start_matches('*').trim_end_matches('*').to_string(),
            )
        })
        .collect::<Vec<_>>();

    fill_title_gap(&mut res);
    Ok(res)
}

fn fill_title_gap(titles: &mut [(usize, String)]) {
    if titles.is_empty() {
        return;
    }

    let mut cur = titles[0].0;
    for (depth, _) in titles.iter_mut().skip(1) {
        if *depth > cur && (*depth - cur) > 1 {
            *depth = cur + 1;
        }

        cur = *depth;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let text = "#### This is a sample text";

        let bres = RE.captures(text);
        assert!(bres.is_some());
        let res = bres.unwrap();

        assert!(res.get(1).is_some());
        assert_eq!(res.get(1).unwrap().as_str(), "## This is a sample text");
    }
}
