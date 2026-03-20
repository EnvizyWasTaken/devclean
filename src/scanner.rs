use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};
use std::path::Path;
use crate::rules::{IGNORE_DIRS, JUNK_DIRS};
use rayon::prelude::*;


#[derive(Debug)]
pub struct JunkItem {
    pub path: PathBuf,
    pub size: u64,
    pub is_system: bool,
}

fn is_junk(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|name| JUNK_DIRS.contains(&name))
        .unwrap_or(false)
}

fn is_safe_path(path: &std::path::Path) -> bool {
    path.starts_with("/home")
}

fn is_ignored(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|n| IGNORE_DIRS.contains(&n))
        .unwrap_or(false)
}

fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.len() > 0)
        .map(|m| m.len())
        .sum()
}

pub fn scan(path: &str) -> Vec<JunkItem> {
    let mut results = Vec::new();

    let mut walker = WalkDir::new(path).into_iter();

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if is_ignored(&entry) {
            walker.skip_current_dir(); 
            continue;
        }

        if is_junk(&entry) {
            let size = dir_size(entry.path());
            let is_system = !is_safe_path(entry.path());

            results.push(JunkItem {
                path: entry.path().to_path_buf(),
                size,
                is_system,
            });

            walker.skip_current_dir();
        }
    }

    results
}