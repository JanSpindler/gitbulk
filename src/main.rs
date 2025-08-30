use std::{env, fs};
use std::process::Command;

fn help() {
    println!("gitbulk - Bulk Git Operations Tool");
    println!();
    println!("USAGE:");
    println!("    gitbulk <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("    list      List all git repositories in the current directory");
    println!("    status    Show git status for all repositories");
    println!("    fetch     Fetch updates from remote for all repositories");
    println!("    branch    Show current branch for all repositories");
    println!("    pull      Pull changes (only if working tree is clean)");
    println!("    help      Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    gitbulk performs git operations on all git repositories found");
    println!("    in subdirectories of the current working directory.");
    println!();
    println!("EXAMPLES:");
    println!("    gitbulk list     # List all git repos");
    println!("    gitbulk status   # Check status of all repos");
    println!("    gitbulk fetch    # Fetch from all remotes");
    println!("    gitbulk pull     # Pull only repos with clean working trees");
}

fn get_git_subdirs(base_dir: &str) -> Vec<fs::DirEntry> {
    fs::read_dir(base_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.as_ref().unwrap().path();
            if path.is_dir() {
                let mut git_path = path.clone();
                git_path.push(".git");
                if git_path.is_dir() {
                    return Some(entry.unwrap());
                }
            }
            None
        })
        .collect()
}

fn list() {
    let git_dirs = get_git_subdirs(".");
    for dir in git_dirs {
        println!("{}", dir.path().display());
    }
}

fn status() {
    let git_dirs = get_git_subdirs(".");
    for dir in git_dirs {
        let path = dir.path();
        let mut child = Command::new("git")
            .arg("status")
            .current_dir(&path)
            .spawn()
            .expect("Failed to start git status");

        println!("Directory: \x1b[33m{}\x1b[0m", path.display().to_string());
        child.wait().expect("Failed to wait on git status");
    }
}

fn fetch() {
    let git_dirs = get_git_subdirs(".");
    for dir in git_dirs {
        let path = dir.path();
        let mut child = Command::new("git")
            .arg("fetch")
            .current_dir(&path)
            .spawn()
            .expect("Failed to start git fetch");

        println!("Directory: \x1b[33m{}\x1b[0m", path.display().to_string());
        child.wait().expect("Failed to wait on git fetch");
    }
}

fn branch() {
    let git_dirs = get_git_subdirs(".");
    for dir in git_dirs {
        let path = dir.path();
        let mut child = Command::new("git")
            .arg("branch")
            .current_dir(&path)
            .spawn()
            .expect("Failed to start git branch");

        println!("Directory: \x1b[33m{}\x1b[0m", path.display().to_string());
        child.wait().expect("Failed to wait on git branch");
    }
}

fn pull() {
    let git_dirs = get_git_subdirs(".");
    for dir in git_dirs {
        let path = dir.path();

        let status_cmd = Command::new("git")
            .arg("status")
            .current_dir(&path)
            .output()
            .expect("Failed to execute git status");
        let status_str = String::from_utf8_lossy(&status_cmd.stdout);
        if !status_cmd.status.success() || 
                !status_str.ends_with("nothing to commit, working tree clean\n") {
            println!("Directory: \x1b[33m{}\x1b[0m ERROR", path.display().to_string());
            continue;
        }

        let mut child = Command::new("git")
            .arg("pull")
            .current_dir(&path)
            .spawn()
            .expect("Failed to start git pull");

        println!("Directory: \x1b[33m{}\x1b[0m", path.display().to_string());
        child.wait().expect("Failed to wait on git pull");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "help" => help(),
        "list" => list(),
        "status" => status(),
        "fetch" => fetch(),
        "branch" => branch(),
        "pull" => pull(),
        _ => help(),
    }
}
