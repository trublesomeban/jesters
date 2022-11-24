#![allow(unused, unreachable_code)]
macro_rules! str {
    () => {
        String::new()
    };
    ($s: expr) => {
        String::from($s)
    };
}

use std::{collections::HashMap, error::Error, fs::read_to_string, hash::Hash, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let reader = stdin();
    println!("Błazeńsko szybki terminal!");
    let mut varmap: HashMap<String, String> = HashMap::new();
    loop {
        let mut input = str!();
        reader.read_line(&mut input)?;
        let args = parse_args(input.trim().to_string());
        // println!("{:?}", args);
        match args[0].as_str() {
            "echo" => {
                if args.len() < 2 {
                    println!("Nie podano argumentu")
                } else {
                    if args[1] == "--inicjały" {
                        println!("{}", read_to_string("inicjały.txt").unwrap())
                    } else {
                        for i in 1..args.len() {
                            let var = lookup_var(&args[0], &varmap);
                            match var {
                                Ok(val) => print!("{}", val),
                                Err(e) => {
                                    println!("{e} this is an erfror");
                                    break;
                                }
                            }
                        }
                        print!("\n")
                    }
                }
            }
            "exit" => {
                if args.len() < 2 {
                    println!("Nie podano argumentu")
                } else {
                    let var = lookup_var(&args[0], &varmap);
                    match var {
                        Ok(val) => println!("Exited with code {}", val),
                        Err(e) => println!("{e}"),
                    }
                }
                break;
            }
            "var" => {
                if args.len() < 2 {
                    println!("Nie podano argumentu")
                } else {
                    let var = lookup_var(&args[2], &varmap);

                    varmap.insert(
                        args[1].clone(),
                        match var {
                            Ok(val) => val.to_owned(),
                            Err(e) => String::from("undefined"),
                        },
                    );
                }
            }
            "cmp" => {
                if args.len() < 3 {
                    println!("Porównanie wymaga przynajmniej dwóch wartości.")
                } else {
                    let mut ops: Vec<i64> = vec![];
                    for i in 1..args.len() {
                        let var = lookup_var(&args[i], &varmap);
                        match var {
                            Ok(val) => ops.push(val.parse::<i64>().unwrap()),
                            Err(e) => {
                                println!("{e} this is an error");
                                break;
                            }
                        }
                    }
                    ops.sort();
                    for i in &ops[0..ops.len() - 1] {
                        print!("{i} < ")
                    }
                    println!("{}", ops[ops.len() - 1])
                }
            }
            _ => println!("Nieznana komenda"),
        }
    }
    Ok(())
}

fn parse_args(args: String) -> Vec<String> {
    let mut parsed = vec![];
    let mut args = args.chars();
    let mut mode = 0;
    let mut arg = str!();
    while let Some(ch) = args.next() {
        match mode {
            // normal reading mode
            0 => match ch {
                // end of keyword
                ' ' => {
                    parsed.push(arg.clone());
                    arg = str!();
                }
                // start of string
                '"' => {
                    mode = 1;
                }
                _ => arg += ch.to_string().as_str(),
            },
            // string mode
            1 => match ch {
                // end of string
                '"' => {
                    parsed.push(arg.clone());
                    arg = str!();
                    // end of string mode
                    mode = 2
                }
                _ => arg += ch.to_string().as_str(),
            },
            // prevents empty strings from being pushed by normal mode due to a whitespace
            2 => mode = 0,
            _ => {}
        }
    }
    parsed.push(arg.clone());
    parsed
}

fn lookup_var<'a>(
    str: &'a String,
    varmap: &'a HashMap<String, String>,
) -> Result<&'a String, String> {
    if !str.contains("$") {
        Ok(str)
    } else {
        let lookup = str.clone();
        match varmap.get(&lookup[1..lookup.len()]) {
            None => Err(String::from(format!("Zmienna '{lookup}' nie istnieje"))),
            Some(val) => Ok(val),
        }
    }
}
