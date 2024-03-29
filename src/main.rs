use std::fs;
use std::env;
use std::cmp;
use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use colored::*;

const BASEURL: &str = "https://codeforces.com";

fn compile_cpp(extra: &str) {
    let mut command = Command::new("clang++");

    command
        .arg("-std=c++20")
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

fn compile() {
    compile_cpp("");
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

fn binary_name_cpp() -> String {
    return "./solution".to_string();
}

fn verify() {
    compile();

    for i in 1..1000000 {
        let input = format!("in_{}.txt", i);
        let output = format!("out_{}.txt", i);

        if !Path::new(&input).exists() || !Path::new(&output).exists() {
            break;
        }

        let mut child = Command::new(binary_name_cpp())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        {
            let input_content = fs::read_to_string(&input).unwrap();
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(input_content.as_bytes()).unwrap();
        }

        let child_output = child.wait_with_output().expect("Failed to read stdout");
        let real_content = String::from_utf8_lossy(&child_output.stdout);
        let mut real_lines = real_content.lines();

        let etalon_content = fs::read_to_string(&output).unwrap();
        let mut etalon_lines = etalon_content.lines();

        let mut res = true;
        loop {
            let line_et = etalon_lines.next();
            let line_re = real_lines.next();
            if line_et == None && line_re == None {
                break;
            }

            if line_et == None || line_re == None {
                res = false;
                break;
            }

            if line_et.unwrap().trim() != line_re.unwrap().trim() {
                res = false;
                break;
            }
        }

        let test_str = format!("Check {}/{}", &input, &output);
        if res {
            println!("{}", test_str.green());
        }
        else {
            println!("{}", test_str.red());
        }
    }
}

fn pre_content(slice: &str) -> String {
    let pre_begin = slice.find("<pre>").unwrap();
    let pre_end = slice.find("</pre>").unwrap();
    let raw_text = &slice[pre_begin + 5..pre_end];
    let text = raw_text
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("</div>", "\n")
        .trim().to_string();
    let re1 = Regex::new("<div class=\"test-example-line test-example-line-even test-example-line-[0-9]\">").unwrap();
    let re2 = Regex::new("<div class=\"test-example-line test-example-line-odd test-example-line-[0-9]\">").unwrap();
    let t1 = re1.replace_all(&text, "");
    let t2 = re2.replace_all(&t1, "");
    return t2.to_string();
}

fn save_problem_content(body: &str, folder: &str, problem: &str) -> String {
    let pat_sample = "<div class=\"sample-test\">";
    let pat_input = "<div class=\"input\">";
    let pat_output = "<div class=\"output\">";
    let sample_index = body.find(&pat_sample).unwrap();
    let mut slice = &body[sample_index..];

    let upproblem = problem.to_uppercase();
    let _ = fs::create_dir(format!("{}/{}", folder, upproblem));

    let mut done = false;
    let mut index = 1;
    while !done {
        let input_index = slice.find(&pat_input);
        if input_index == None {
            done = true;
            continue;
        }
        slice = &slice[input_index.unwrap()..];
        let input = pre_content(slice);

        let output_index = slice.find(&pat_output);
        if output_index == None {
            done = true;
            continue;
        }
        slice = &slice[output_index.unwrap()..];
        let output = pre_content(slice);

        fs::write(format!("{}/{}/in_{}.txt", folder, upproblem, index), input).unwrap();
        fs::write(format!("{}/{}/out_{}.txt", folder, upproblem, index), output).unwrap();
        index = index + 1;
    }
    return upproblem;
}

fn calc_cookie() -> String {
    return String::from("JSESSIONID=AA1CA98F87F1795DCFB80B9A70C66A79-n1")
}

fn problem(contest: &str, problem: &str) -> String {
    let url = format!("{}/problemset/problem/{}/{}", BASEURL, contest, problem);
    
    let agent = ureq::agent();
    let body = agent.get(&url)
        .set("Cookie", &calc_cookie())
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let pname = save_problem_content(&body, ".", problem);
    template("2", "..", &pname, false);
    return pname;
}

fn template(fname: &str, srcdir: &str, dstdir: &str, trace: bool) {
    let from = format!("{}/templates/{}.cpp", srcdir, fname);
    let to = format!("{}/solution.cpp", dstdir);
    if trace {
        println!("copy: {} -> {}", &from, &to);
    }
    fs::copy(&from, &to).unwrap();
}

fn round(contest: &str, init_problems: bool) -> String {
    let url = format!("{}/contest/{}/problems?f0a28=1", BASEURL, contest);

    let agent = ureq::agent();

    let body = agent.get(&url)
        .set("Cookie", &calc_cookie())
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    let pat_caption = "class=\"caption\">";
    let pat_problem = "problemindex=\"";

    let caption_index = body.find(&pat_caption).unwrap();
    let mut slice = &body[caption_index..];
    let end_caption_index_symbol = slice.find(" (").unwrap();
    let end_caption_index_div = slice.find("</div>").unwrap();
    let end_caption_index = cmp::min(end_caption_index_symbol, end_caption_index_div);
 
    let rname = slice[pat_caption.len()..end_caption_index].replace(" ", "_");
    let _ = fs::create_dir(&rname);

    fs::write(format!("{}/.meta", rname), contest).unwrap();

    let mut done = false;
    while init_problems && !done {
        let problem_index = slice.find(pat_problem);
        if problem_index == None {
            done = true;
            continue;
        }
        slice = &slice[problem_index.unwrap() + pat_problem.len()..];
        let problem_end_index = slice.find("\"");
        let pname = &slice[..problem_end_index.unwrap()];
        let next_problem_index = slice.find(pat_problem).unwrap_or(slice.len());
        let problem_view = &slice[..next_problem_index];
        save_problem_content(problem_view, &rname, pname);
        template("2", "..", pname, false);
    }

    return rname;
}

fn help() {
    println!("Usage cf [command] <args...>

        f, folder - create round folder
            <contest> is mandatory

        p, problem - download test data
            <contest> and <problem> are mandatory

        r, round - create round folder and download all problems test data
            <contest> is mandatory

        t, template - create solution source from template
            <name> is mandatory

        v, verify - build & verify solution

        c, compile - build solutuon binary

        g, git - commit & push solution sources

More details here: https://github.com/proydakov/coderust");
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
        else if command == "v" || command == "verify" {
            if args.len() == 2 {
                verify();
            }
            else {
                help();
            }
        }
        else if command == "f" || command == "folder" {
            if args.len() == 3 {
                println!("{}", round(&args[2], false));
            }
            else {
                help();
            }
        }
        else if command == "r" || command == "round" {
            if args.len() == 3 {
                println!("{}", round(&args[2], true));
            }
            else {
                help();
            }
        }
        else if command == "p" || command == "problem" {
            if args.len() == 4 {
                println!("{}", problem(&args[2], &args[3]));
            }
            else {
                help();
            }
        }
        else if command == "t" || command == "template" {
            if args.len() == 3 {
                template(&args[2], "../..", ".", true);
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
