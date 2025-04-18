use std::fs;
use std::collections::HashMap;
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering};

static LINE: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
enum VariableValue {
    Int(i128),
    Float(f64),
    Str(String),
    Bool(bool),
}

static mut VARS: Option<HashMap<String, VariableValue>> = None;

fn main() {
    unsafe {
        VARS = Some(HashMap::new());
    }
    let parsed_contents = parse(r"C:\Users\busin\OneDrive\Documents\GitHub\Mocha-Rust\src\main.mocha");
    // println!("{:?}", parsed_contents);
    loop {
        let line = LINE.load(Ordering::SeqCst);
        run(&line, &parsed_contents);
    }
}

fn parse(filepath: &str) -> Vec<Vec<String>> {
    //read the file
    let rawfile = fs::read_to_string(filepath).expect("woopsies");
    let rawfile = rawfile.replace("\\\\", "\0");
    //fixed spaces
    let re = Regex::new(r#""([^"]*)""#).unwrap();
    let replaced_quotes = re.replace_all(&rawfile, |caps: &regex::Captures| {
        caps[0].replace('\0', "\\")
    });
    let processed_file = re.replace_all(&replaced_quotes, |caps: &regex::Captures| {
        caps[0].replace(' ', "\0")
    });
    //split to tokens
    let mut replaced_lines = Vec::new();
    for line in processed_file.lines() {
        let split_line: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        replaced_lines.push(split_line);
    }
    //replace the \0 with spaces
    for i in 0..replaced_lines.len() {
        for j in 0..replaced_lines[i].len() {
            replaced_lines[i][j] = replaced_lines[i][j].replace('\0', " ");
            replaced_lines[i][j] = replaced_lines[i][j].replace("\\n", "\n");
        }
    }

    replaced_lines
}

fn run(line: &usize, lines: &Vec<Vec<String>>) {
    match lines[*line][0].as_str() {
        "out" => {
            for i in 1..lines[*line].len() {
                let output = &lines[*line][i];
                if output.starts_with('"') && output.ends_with('"') {
                    print!("{}", &output[1..output.len() - 1]);
                } else {
                    //variablez n stuff
                    print!("{}", output);
                }
            }
            LINE.store(LINE.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        }
        "var" => {
            unsafe { //just making it a bit easier so i dont have to keep writing unsafe{}
                match lines[*line][1].as_str() {
                    "int" => {
                        match lines[*line][3].as_str() {
                            "set" => {
                                // set the lines[line][2] key in VARS to the value of lines[line][4] IF lines[line][4] is an int
                                if lines[*line][4].parse::<i128>().is_ok() {
                                    // &VARS.unwrap().insert(lines[*line][2].clone(), VariableValue::Int(lines[*line][4].parse::<i128>().unwrap()));
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            LINE.store(LINE.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        }
        "end" => {
            std::process::exit(0)        
        }
        _ => {}
    }
}