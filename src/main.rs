use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs::{read_dir, canonicalize};
use std::collections::BTreeMap;


fn main() {
    let arg1 = env::args().nth(1);
    let counter : BTreeMap<String, usize>;
    match arg1 {
        None => counter = count_dir_lines(Path::new(".")),
        Some(v) => counter = count_dir_lines(clone_repo(&v).unwrap().as_path())
    }
    for (extension, num_lines) in counter.iter() {
        println!("{} : {}", extension, num_lines);
    }
}

fn clone_repo(repo: &str) -> std::io::Result<PathBuf> {
    let mut git = Command::new("git")
        .args(&["clone", &format!("https://github.com/{}", repo)])
        .spawn()
        .expect("Failed to clone the repository... is git installed?");
    git.wait().expect("Something went wrong.");
    let mut iter = repo.split('/');
    return canonicalize(Path::new(iter.next_back().unwrap()));
}

fn count_dir_lines(dir: &Path) -> BTreeMap<String, usize> {
    let mut counter = BTreeMap::new();
    for e in read_dir(dir).unwrap() {
        let entry = e.unwrap();
        let ft = entry.file_type().unwrap();
        if ft.is_file() {
            counter = merge_counters(counter, count_file_lines(entry.path().as_path()));
        } else if ft.is_dir() {
            counter = merge_counters(counter, count_dir_lines(entry.path().as_path()));
        }
    }
    return counter;
}

fn count_file_lines(file: &Path) -> BTreeMap<String, usize> {
    let mut res = BTreeMap::new();
    let whole_file_string = std::fs::read_to_string(file);
    match whole_file_string {
        Err(_) => return res,
        Ok(v) => match file.extension() {
            None => return res,
            Some(ext) => res.insert(ext.to_string_lossy().to_mut().to_string(), v.lines().count())
        } 
    };
    return res;
}

fn merge_counters(a: BTreeMap<String, usize>, b: BTreeMap<String, usize>) -> BTreeMap<String, usize> {
    let mut res = BTreeMap::new();
    for (key, val) in a.iter() {
        match b.get(key) {
            None => res.insert(key.to_string(), *val),
            Some(v) => res.insert(key.to_string(), val + v)
        };
    }
    return res;
}