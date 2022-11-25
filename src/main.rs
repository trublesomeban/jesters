macro_rules! str {
    () => {
        String::new()
    };
    ($s: expr) => {
        String::from($s)
    };
}

use std::{
    collections::HashMap, default::Default, error::Error, fs::read_to_string, io::stdin,
    str::FromStr,
};

struct Exit(u64);

fn main() -> Result<(), Box<dyn Error>> {
    let reader = stdin();
    println!("Błazeńsko szybki terminal!");
    let mut varmap: HashMap<String, String> = HashMap::new();
    varmap.insert(str!("undefined"), str!("undefined"));
    let mut pipe: Vec<String> = vec![];
    loop {
        let mut input = str!();
        reader.read_line(&mut input)?;
        let args = parse_args(input.trim().to_string());
        // println!("{:?}", args);
        match match_command(&args, &mut varmap, &pipe) {
            Ok(new_pipe) => pipe = new_pipe,
            Err(Exit(code)) => break println!("Exited with code {code}"),
        }
    }
    Ok(())
}

fn match_command(
    args: &Vec<String>,
    varmap: &mut HashMap<String, String>,
    pipe: &Vec<String>,
) -> Result<Vec<String>, Exit> {
    let mut new_pipe: Vec<String> = vec![];
    match args[0].as_str() {
        "echo" => {
            if args.len() < 2 {
                println!("Nie podano argumentu")
            } else {
                if args[1] == "--inicjały" {
                    println!("{}", read_to_string("inicjały.txt").unwrap())
                } else {
                    let mut args = args.to_owned();
                    let delim = if !args[1].contains("\"") && args[1].contains("--delim") {
                        let _v = args[2].to_owned();
                        args.remove(1);
                        args.remove(1);
                        _v
                    } else {
                        str!(" ")
                    };
                    let echo_args = collect_args::<String>(&args, &varmap, &pipe);
                    let out = echo_args.join(delim.as_str());
                    println!("{out}");
                    new_pipe.clear();
                    new_pipe = echo_args;
                }
            }
        }
        "exit" => {
            if args.len() < 2 {
                println!("Nie podano argumentu")
            } else {
                let val = lookup_var(&args[1], &varmap, &pipe);
                return Err(Exit(val.parse::<u64>().unwrap()));
            }
        }
        "var" => {
            if args.len() < 2 {
                println!("Nie podano argumentu")
            } else {
                let var = lookup_var(&args[2], &varmap, &pipe);
                varmap.insert(args[1].clone(), var.to_owned());
            }
        }
        "list" => {
            let vals = collect_args::<i64>(&args, &varmap, &pipe);
            new_pipe.clear();
            new_pipe = vals.iter().map(|v| v.to_string()).collect::<Vec<String>>();
        }
        "cmp" => {
            let mut vals = collect_args::<i64>(&args, &varmap, &pipe);
            vals.sort();
            // for i in &vals[0..vals.len() - 1] {
            //     // iterate through all except last element
            //     print!("{i} ≤ ")
            // }
            // println!("{}", vals[vals.len() - 1]); // print the last element with a newline
            new_pipe.clear();
            new_pipe = vals.iter().map(|v| v.to_string()).collect::<Vec<String>>();
        }
        "sum" => {
            let vals = collect_args(&args, &varmap, &pipe);
            let sum: i64 = vals.iter().sum();
            new_pipe.clear();
            new_pipe.push(sum.to_string());
        }
        "prod" => {
            let vals = collect_args(&args, &varmap, &pipe);
            let prod: i64 = vals.iter().product();
            new_pipe.clear();
            new_pipe.push(prod.to_string());
        }
        "fac" => {
            let base = lookup_var(&args[1], &varmap, &pipe)
                .parse::<i64>()
                .unwrap_or(args[1].len() as i64);
            if base == 0 {
                println!("0")
            } else {
                let res = (1..=base).product::<i64>();
                new_pipe.clear();
                new_pipe.push(res.to_string());
            }
        }
        "pow" => {
            if args.len() < 3 {
                println!("Nie podano wszystkich argumentów")
            } else {
                let base = lookup_var(&args[1], &varmap, &pipe)
                    .parse::<i64>()
                    .unwrap_or(args[1].len() as i64);
                let power = lookup_var(&args[2], &varmap, &pipe)
                    .parse::<i64>()
                    .unwrap_or(args[1].len() as i64);
                let res = base.pow(power as u32);
                // println!("{}", res);
                new_pipe.clear();
                new_pipe.push(res.to_string());
            }
        }
        "peek" => {
            if args.len() < 2 {
                println!("Nie podano nazwy pliku")
            } else {
                let path = args[1].to_owned();
                let name = path.split("/").last().expect("ItErrAtor");
                let contents =
                    read_to_string(format!("./{path}")).expect("Error  when reading file");
                println!("===========================================");
                println!("{}", name);
                println!("===========================================");
                for line in contents.split("\n") {
                    println!("~ {line}")
                }
                println!("===========================================");
            }
        }
        _ => println!("Nieznana komenda"),
    };
    Ok(new_pipe)
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

fn collect_args<'a, T: FromStr + Default>(
    args: &Vec<String>,
    varmap: &'a HashMap<String, String>,
    pipe: &Vec<String>,
) -> Vec<T> {
    if args[1] == "|" {
        pipe.iter()
            .map(|val| val.parse::<T>().unwrap_or(T::default()))
            .collect::<Vec<T>>()
    } else {
        args[1..args.len()]
            .iter()
            .map(|val| {
                lookup_var(&val, &varmap, pipe)
                    .parse::<T>()
                    .unwrap_or(T::default())
            })
            .collect::<Vec<T>>()
    }
}

fn lookup_var<'a>(
    str: &'a String,
    varmap: &'a HashMap<String, String>,
    pipe: &'a Vec<String>,
) -> &'a String {
    if str.contains("$") {
        let lookup = str.clone();
        match varmap.get(&lookup[1..lookup.len()]) {
            None => varmap.get("undefined").unwrap(),
            Some(val) => val,
        }
    } else if str.contains("|") {
        let idx = str[1..str.len()].parse::<usize>().unwrap();
        &pipe[idx]
    } else {
        str
    }
}
