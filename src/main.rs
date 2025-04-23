use std::fs;
use std::collections::HashMap;
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static LINE: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
enum VariableValue {
    Int(i128),
    Float(f64),
    Str(String),
    Bool(bool),
}

static VARS: Lazy<Mutex<HashMap<String, VariableValue>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

fn main() {
    let parsed_contents = parse(r"C:\Users\busin\OneDrive\Documents\GitHub\Mocha-Rust\src\main.mocha");
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
                    print!("{}", vars.get(output).as_str());
                }
            }
            LINE.store(LINE.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        }
        "var" => {
            let mut vars = VARS.lock().unwrap();
            match lines[*line][1].as_str() {
                "int" => {
                    match lines[*line][3].as_str() {
                        "set" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(num));
                            } 
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(*val));
                            }                       
                        }
                        "add" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) + VariableValue::Int(num));
                            }
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) + VariableValue::Int(*val));
                            }
                        }
                        "sub" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) - VariableValue::Int(num));
                            }
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) - VariableValue::Int(*val));
                            }
                        }
                        "mlt" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) * VariableValue::Int(num));
                            }
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) * VariableValue::Int(*val));
                            }
                        }
                        "div" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) / VariableValue::Int(num));
                            }
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())) / VariableValue::Int(*val));
                            }
                        }
                        "mod" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())).rem_euclid(VariableValue::Int(num)));
                            }                                 
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Int(vars.get(lines[*line][2].clone())).rem_euclid(VariableValue::Int(*val)));
                            }
                        }
                        "pow" => {
                            if let Ok(num) = lines[*line][4].parse::<i128>() {
                                vars.insert(lines[*line][2].clone(), pow(VariableValue::Int(vars.get(lines[*line][2].clone())), VariableValue::Int(num)));
                            }
                            else if let Some(VariableValue::Int(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), pow(VariableValue::Int(vars.get(lines[*line][2].clone())), VariableValue::Int(*val)));
                            }
                        }
                        _ => {}
                    }
                }
                "dbl" => {
                    match lines[*line][3].as_str() {
                        "set" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(num));
                            } 
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(*val));
                            }                       
                        }
                        "add" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) + VariableValue::Float(num));
                            }
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) + VariableValue::Float(*val));
                            }
                        }
                        "sub" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) - VariableValue::Float(num));
                            }
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) - VariableValue::Float(*val));
                            }
                        }
                        "mlt" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) * VariableValue::Float(num));
                            }
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) * VariableValue::Float(*val));
                            }
                        }
                        "div" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) / VariableValue::Float(num));
                            }
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())) / VariableValue::Float(*val));
                            }
                        }
                        "mod" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())).rem_euclid(VariableValue::Float(num)));
                            }                                 
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), VariableValue::Float(vars.get(lines[*line][2].clone())).rem_euclid(VariableValue::Float(*val)));
                            }
                        }
                        "pow" => {
                            if let Ok(num) = lines[*line][4].parse::<f128>() {
                                vars.insert(lines[*line][2].clone(), pow(VariableValue::Float(vars.get(lines[*line][2].clone())), VariableValue::Float(num)));
                            }
                            else if let Some(VariableValue::Float(val)) = vars.get(&lines[*line][4]) {
                                vars.insert(lines[*line][2].clone(), pow(VariableValue::Float(vars.get(lines[*line][2].clone())), VariableValue::Float(*val)));
                            }
                        }
                        _ => {}
                    }
                }
                "bln" => {
                    match lines[*line][3].as_str() {
                        "bin" => {
                            match lines[*line][4].as_str() {
                                "and" => {
                                    vars.insert(lines[*line][2].clone(), VariableValue::Boolean(vars.get(lines[*line][5].clone())) && VariableValue::Boolean(vars.get(lines[*line][6].clone())))
                                }
                                "orr" => {
                                    vars.insert(lines[*line][2].clone(), VariableValue::Boolean(vars.get(lines[*line][5].clone())) || VariableValue::Boolean(vars.get(lines[*line][6].clone())))
                                }
                                "xor" => {
                                    vars.insert(lines[*line][2].clone(), VariableValue::Boolean(vars.get(lines[*line][5].clone())) ^ VariableValue::Boolean(vars.get(lines[*line][6].clone())))
                                }
                                "not" => {
                                    vars.insert(lines[*line][2].clone(), VariableValue::Boolean(vars.get(lines[*line][5].clone())))
                                }
                            }
                        }
                        "str" => {
                            match lines[*line][4].as_str() {
                                "eql" => {
                                    
                                    vars.insert(lines[*line][2].clone(), assert_eq!(VariableValue::Str(vars.get(lines[*line][5].clone())), VariableValue::Boolean(vars.get(lines[*line][6].clone()))))
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            LINE.store(LINE.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        }
        "end" => {
            std::process::exit(0)        
        }
        _ => {}
    }
}