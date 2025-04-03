use std::fs;
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering};

static LINE: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let parsed_contents = parse(r"C:\Users\busin\OneDrive\Documents\GitHub\Mocha-Rust\src\main.mocha");
    println!("{:?}", parsed_contents);
    loop {
        let line = LINE.load(Ordering::SeqCst);
        run(&line, &parsed_contents);
    }
}

fn parse(filepath: &str) -> Vec<Vec<String>> {
    //read the file
    let rawfile = fs::read_to_string(filepath).expect("woopsies");
    //fix spaces
    let re = Regex::new(r#""([^"]*)""#).unwrap();
    let processed_file = re.replace_all(&rawfile, |caps: &regex::Captures| {
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
        "end" => {
            std::process::exit(0)        
        }
        _ => {}
    }
}