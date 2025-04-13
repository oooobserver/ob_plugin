use std::{
    fs::{self, DirEntry, ReadDir},
    time::SystemTime,
};

use crate::{data::SortOrder, error, RE};

const CONTENT_FILE_NAME: &str = "Content.md";

/// Check if the file's extention is md
///
/// # Examples
///
/// ```
/// use ob_plugin::util::check_path_extention;
///
/// let path = "/path/to/this.md";
/// assert!(check_path_extention(path));
///
/// let path = "/path/to/this.me";
/// assert!(!check_path_extention(path));
/// ```
pub fn check_path_extention(path: &str) -> bool {
    let ext = path
        .split('/')
        .collect::<Vec<_>>()
        .into_iter()
        .last()
        .unwrap_or(path);

    ext.ends_with(".md")
}

/// Parse the raw title
///
/// # Examples
///
/// ```
/// use ob_plugin::util::parse_title;
///
/// let title = "### Concurrent ";
/// assert_eq!(parse_title(title), (3 as _, "Concurrent".to_owned()));
///
/// let title = "#### Memory layout in go";
/// assert_eq!(parse_title(title), (4 as _, "Memory layout in go".to_owned()));
/// ```
pub fn parse_title(title: &str) -> (usize, String) {
    let arr = title.split_ascii_whitespace().collect::<Vec<_>>();
    (arr[0].len(), arr[1..arr.len()].join(" "))
}

/// Trim the useless part of the path
///
/// # Examples
///
/// ```
/// use ob_plugin::util::reprot_path;
///
/// let path = "../../path/to/t.md";
/// assert_eq!(reprot_path(path), "path/to/t.md".to_owned());
///
/// let path = "path/to/t.md";
/// assert_eq!(reprot_path(path), "path/to/t.md".to_owned());
/// ```
pub fn reprot_path(path: &str) -> String {
    path.split('/')
        .filter(|&p| p != "..")
        .collect::<Vec<_>>()
        .join("/")
}

/// Check if the file is already extracted, if so delete previous content
///
/// # Examples
///
/// ```
/// use ob_plugin::util::check_delete_previous_content;
///
/// let mut file_content = "## Content\n***\n## Prelude";
/// file_content = check_delete_previous_content(file_content);
/// assert_eq!(file_content, "## Prelude");
///
/// let mut file_content = "## Prelude";
/// file_content = check_delete_previous_content(file_content);
/// assert_eq!(file_content, "## Prelude");
/// ```
pub fn check_delete_previous_content(file_content: &str) -> &str {
    let first_title = RE.captures(file_content);
    if let Some(t) = first_title {
        if t.get(0).map(|s| s.as_str()) == Some("## Content") {
            // Find the next title at offset 15
            // Because "## Content\n***" is 14 length
            let second = RE.find_at(file_content, 15).map(|m| m.as_str());
            if let Some(t) = second {
                let offset = file_content.find(t).expect("previous has matched");
                return &file_content[offset..];
            }
        }
    }

    file_content
}

// sort_filter_entries will add filter function later, for now it default filter `content.md` file
pub fn sort_filter_entries(entries: ReadDir, order: SortOrder) -> Vec<DirEntry> {
    match order {
        SortOrder::Alphabet => sort_entries_by_name(entries),
        SortOrder::UpdateTime => sort_entries_by_update_time(entries),
    }
}

fn sort_entries_by_name(entries: ReadDir) -> Vec<DirEntry> {
    let mut entries: Vec<DirEntry> = entries
        .filter_map(Result::ok)
        .filter(|e| e.file_name() != CONTENT_FILE_NAME)
        .collect();
    entries.sort_by_key(|a| a.file_name());
    entries
}

fn sort_entries_by_update_time(entries: ReadDir) -> Vec<DirEntry> {
    let mut entries: Vec<DirEntry> = entries
        .filter_map(Result::ok)
        .filter(|e| e.file_name() != CONTENT_FILE_NAME)
        .collect();

    entries.sort_by(|a, b| {
        let a_time = a
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let b_time = b
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        a_time.cmp(&b_time)
    });

    entries
}

pub fn normalize_paths(paths: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for original_path in paths {
        match fs::canonicalize(&original_path) {
            Ok(path_buf) => match path_buf.to_str() {
                Some(s) => result.push(s.to_string()),
                None => error!("Cannot convert to string: {:?}", path_buf),
            },
            Err(e) => error!("Failed to canonicalize '{}': {}", original_path, e),
        }
    }

    result
}
