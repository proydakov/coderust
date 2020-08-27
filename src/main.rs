use std::env;
use std::process::Command;

fn compile_cpp(extra: String) {
    let mut command = Command::new("clang++");

    command
        .arg("-std=c++17")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-O2")
        .arg("solution.cpp")
        .arg("-o")
        .arg("solution");

    if !extra.is_empty() {
        command
            .arg(extra);
    }

    command
        .status()
        .expect("can't start clang++");
}

#[cfg(target_os = "macos")]
fn compile() {
    compile_cpp("".to_string());
}

#[cfg(not(target_os = "macos"))]
fn compile() {
    compile_cpp("-m32");
}

fn git() {
    let path = env::current_dir().unwrap().as_path().display().to_string();
    let commit_message = "Added ".to_string() + &path;

    Command::new("git")
        .arg("add")
        .arg("*.cpp")
        .arg("*.txt")
        .status()
        .expect("can't start git");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .status()
        .expect("can't start git");

    Command::new("git")
        .arg("pull")
        .arg("-r")
        .status()
        .expect("can't start git");

    Command::new("git")
        .arg("push")
        .status()
        .expect("can't start git");
}

fn help() {
    println!("Usage cf [command] <args>
    
        c, compile - build solutuon binary
        g, git - commit & push solution sources
    ");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
    }
    else {
        let command = &args[1];
        if command == "c" || command == "compile" {
            compile();
        }
        else if command == "g" || command == "git" {
            git();
        }
        else {
            help();
        }
    }
}
