use std::path::{Path, PathBuf};
use std::error::Error;
use std::{env, fs};
use std::process::{Command, Stdio};
use crate::errors::CompilationFailed;
use std::str::Lines;
use std::io::Write;

pub fn compile(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let compiler = env::var("SLIDE_CC").unwrap_or(String::from("g++"));
    let compiler_flags = env::var("SLIDE_CC_FLAGS").unwrap_or(String::from("-std=c++17 -Wall -Wextra -g3 -ggdb3 -D_GLIBCXX_DEBUG"));
    let compiler_add_flags = env::var("SLIDE_CC_ADD_FLAGS").unwrap_or(String::new());

    let compiler_flags = if compiler_add_flags.is_empty() {
        compiler_flags
    } else {
        format!("{} {}", compiler_flags, compiler_add_flags)
    };
    let mut executable = path.canonicalize()?;
    executable.set_extension("");

    let res = Command::new(compiler)
        .args(compiler_flags.split_ascii_whitespace())
        .arg("-o")
        .arg(&executable)
        .arg(path)
        .status()?;

    if res.success() {
       Ok(executable.into())
    } else {
        Err(CompilationFailed.into())
    }
}

#[derive(Debug)]
struct Test {
    name: String,
    input: String,
    output: String,
}

fn extract_test(idx: i32, lines: &mut Lines) -> Option<(Test, bool)> {
    let mut input = String::new();
    let mut output = String::new();
    let mut is_last = false;
    let mut found_sep = false;
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr == "---" {
            found_sep = true;
            break;
        } else if line_tr == "*/" {
            return None;
        }
        input.push_str(line);
    }
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr == "===" {
            break;
        } else if line_tr == "*/" {
            is_last = true;
            break;
        }
        output.push_str(line);
    }
    if !found_sep || output.is_empty() {
        return None;
    }
    Some((Test { name: format!("test.{}", idx), input, output }, is_last))
}

fn extract_stoml_test(lines: &mut Lines) -> Option<Test> {
    let name_line = loop {
        let line = lines.next()?;
        if !line.trim().is_empty() {
            break line;
        }
    };

    if name_line.trim() == "*/" || name_line.len() < 3 {
        return None;
    }
    let name = String::from(&name_line[1..name_line.len()-1]);
    let mut input = String::new();
    let mut output = String::new();
    lines.next();
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr == r#"""""# {
            break;
        }
        input.push_str(line);
    }
    lines.next();
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr == r#"""""# {
            break;
        }
        output.push_str(line);
    }
    Some(Test { name, input, output })
}

fn extract_tests(contents: &str) -> Vec<Test> {
    let mut lines = contents.lines();
    let mut res = Vec::new();
    let mut idx = 0;
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr.starts_with("/*!slide testdata") {
            loop {
                if let Some((test, is_last)) = extract_test(idx, &mut lines) {
                    res.push(test);
                    idx += 1;
                    if is_last {
                        break;
                    }
                } else {
                    break;
                }
            }
        } else if line_tr.starts_with("/*!slide stoml") {
            while let Some(test) = extract_stoml_test(&mut lines) {
                res.push(test);
            }
        }
    }
    res
}

fn compare_strings(str1: &str, str2: &str) -> bool {
    let mut lines1 = str1.lines();
    let mut lines2 = str2.lines();

    fn skip_empty<'a>(lines: &'a mut Lines) -> Option<&'a str> {
        while let Some(line) = lines.next() {
            if line.trim() != "" {
                return Some(line)
            }
        }
        None
    }

    loop {
        let line1 = skip_empty(&mut lines1);
        let line2 = skip_empty(&mut lines2);
        if line1.is_none() != line2.is_none() {
            return false;
        }
        match line1 {
            Some(line1) => {
                let line2 = line2.unwrap();
                if line1.trim() != line2.trim() {
                    return false;
                }
            }
            None => return true
        }
    }
}

fn run_test(executable: &Path, test: &Test, print_failures: bool) -> bool {
    let mut child = match Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(_) => return false,
    };
    let stdin = match child.stdin.as_mut() {
        Some(stdin) => stdin,
        None => return false,
    };
    match stdin.write_all(test.input.as_bytes()) {
        Ok(_) => {},
        Err(_) => return false,
    }
    let output = match child.wait_with_output() {
        Ok(output) => output,
        Err(_) => return false,
    };
    if !output.status.success() {
        println!("Program crashed on test {}", test.name);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stderr);
        return false;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);

    let res = compare_strings(&test.output, &stdout);
    if !res {
        println!("{} failed", test.name);
        if print_failures {
            if test.output.len() < 20 && stdout.len() < 20 {
                println!("{} != {}", stdout.trim(), test.output.trim());
            } else {
                println!("Received:");
                println!("{}", stdout.trim());
                println!("Expected:");
                println!("{}", test.output.trim());
            }
        }
    }
    res
}

pub fn run_tester(path: &Path, pattern: Option<&str>, print_failures: bool) -> Result<(), Box<dyn Error>> {
    let executable = compile(path)?;
    let contents = fs::read_to_string(path)?;
    let tests = extract_tests(&contents);
    let mut success = 0;
    let mut ran = 0;
    for test in &tests {
        if let Some(pattern) = pattern {
            if !test.name.starts_with(pattern) {
                continue;
            }
        }
        ran += 1;
        if run_test(&executable, test, print_failures) {
            success += 1;
        }
    }
    println!("{}/{} Tests ran successfully", success, ran);
    Ok(())
}
