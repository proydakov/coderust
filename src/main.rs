use std::fs;
use std::env;
use std::process::Command;

use std::fs::File;
use std::io::Write;

fn compile_cpp(extra: &str) {
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
    compile_cpp("");
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

fn round(contest: &String) {
    let url = format!("https://codeforces.com/contest/{:?}", contest);
    let body = ureq::get(&url).call().into_string().unwrap();
    let pat = format!("href=\"/contest/{}\">", contest);
    println!("search: {} in {} bytes", pat, body.len());
    let index_begin = body.find(&pat).unwrap();
    let offset = body[index_begin..].find(" (").unwrap();
    let path = &body[index_begin + pat.len() .. index_begin + offset].replace(" ", "_");
    println!("from: {} to: {} data: {:?}", index_begin, index_begin + offset, path);
    let _ = fs::create_dir(path);
    println!("{}", path)
}

fn pre_content(slice: &str) -> String {
    let pre_begin = slice.find("<pre>").unwrap();
    let pre_end = slice.find("</pre>").unwrap();
    let raw_text = &slice[pre_begin + 5..pre_end];
    let text = raw_text
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .trim().to_string();
    return text;
}

fn problem(contest: &String, problem: &String) {
    let url = format!("https://codeforces.com/problemset/problem/{:?}/{:?}", contest, problem);
    let body = ureq::get(&url).call().into_string().unwrap();
    let pat_sample = "<div class=\"sample-test\">";
    let pat_input = "<div class=\"input\">";
    let pat_output = "<div class=\"output\">";
   let sample_index = body.find(&pat_sample).unwrap();
    let mut slice = &body[sample_index..];

    let upproblem = &problem.to_uppercase();
    let _ = fs::create_dir(upproblem);

    for i in 1..1000000 {
        let input_index = slice.find(&pat_input);
        if input_index == None {
            break;
        }
        slice = &slice[input_index.unwrap()..];
        let input = pre_content(slice);

        let output_index = slice.find(&pat_output);
        if output_index == None {
            break;
        }
        slice = &slice[output_index.unwrap()..];
        let output = pre_content(slice);

        println!("input len: {} data: \n{}", input.len(), input);
        println!("output len: {} data: \n{}", output.len(), output);

        fs::write(format!("{}/input_{}.txt", upproblem, i), input).unwrap();
        fs::write(format!("{}/output_{}.txt", upproblem, i), output).unwrap();
    }
}

fn help() {
    println!("Usage cf [command] <args>

        r, round - create round folder
            <contest> is mandatory

        p, problem - download test data
            <contest> and <problem> are mandatory

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
            if args.len() == 2 {
                compile();
            }
            else {
                help();
            }
        }
        else if command == "g" || command == "git" {
            if args.len() == 2 {
                git();
            }
            else {
                help();
            }
        }
        else if command == "r" || command == "round" {
            if args.len() == 3 {
                round(&args[2]);
            }
            else {
                help();
            }
        }
        else if command == "p" || command == "problem" {
            if args.len() == 4 {
                problem(&args[2], &args[3]);
            }
            else {
                help();
            }
        }
        else {
            help();
        }
    }
}
