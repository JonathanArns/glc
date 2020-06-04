use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs::{read_dir, canonicalize};


fn main() {
    let arg1 = env::args().nth(1);
    let counter : usize;
    match arg1 {
        None => counter = count_dir_lines(Path::new(".")),
        Some(v) => counter = count_dir_lines(clone_repo(&v).unwrap().as_path())
    }
    println!("{} lines in your project.", counter);
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

fn count_dir_lines(dir: &Path) -> usize {
    let mut counter = 0;
    for e in read_dir(dir).unwrap() {
        let entry = e.unwrap();
        let ft = entry.file_type().unwrap();
        if ft.is_file() {
            counter += count_file_lines(entry.path().as_path());
        } else if ft.is_dir() {
            counter += count_dir_lines(entry.path().as_path());
        }
    }
    return counter;
}

fn count_file_lines(file: &Path) -> usize {
    let whole_file_string = std::fs::read_to_string(file);
    match whole_file_string {
        Err(_) => return 0,
        Ok(v) => return v.lines().count()
    }
}