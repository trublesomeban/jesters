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

#[derive(Debug)]
struct Exit(u64);

fn main() -> Result<(), Box<dyn Error>> {
    let reader = stdin();
    println!("Błazeńsko szybki terminal!");
    let mut varmap: HashMap<String, String> = HashMap::new();
    varmap.insert(str!("undefined"), str!("undefined"));
    let mut mem: Vec<String> = vec![];
    loop {
        let mut input = str!();
        reader.read_line(&mut input)?;
        let args = parse_args(input.trim().to_string());
        // println!("{:?}", args);
        match match_command(&args, &mut varmap, &mem) {
            Ok(new_mem) => mem = new_mem,
            Err(Exit(code)) => break println!("Exited with code {code}"),
        }
    }
    Ok(())
}

fn match_command(
    args: &Vec<String>,
    varmap: &mut HashMap<String, String>,
    mem: &Vec<String>,
) -> Result<Vec<String>, Exit> {
    let mut new_mem: Vec<String> = vec![];
    match args[0].as_str() {
        "echo" => {
            if args.len() < 2 {
                println!("Nie podano argumentu")
            } else {
                if args[1] == "--inicjały" {
                    println!("{}", read_to_string("inicjały.txt").unwrap())
                } else {
                    let mut args = args.to_owned();
                    let delim = if args[1] == "--delim" {
                        let _v = lookup_var(&args[2], &varmap, &mem).to_owned();
                        args.remove(1);
                        args.remove(1);
                        _v
                    } else {
                        str!(" ")
                    };
                    let echo_args = collect_args::<String>(&args, &varmap, &mem);
                    let out = echo_args.join(delim.as_str());
                    println!("{out}");
                    new_mem = mem.to_owned();
                }
            }
        }
        "exit" => {
            if args.len() < 2 {
                println!("Nie podano argumentu")
            } else {
                let val = lookup_var(&args[1], &varmap, &mem);
                return Err(Exit(val.parse::<u64>().unwrap()));
            }
        }
        "var" => {
            let var = lookup_var(&args[2], &varmap, &mem);
            varmap.insert(args[1].clone(), var.to_owned());
            new_mem = mem.to_owned();
        }
        "mset" => {
            let vals = collect_args::<String>(&args, &varmap, &mem);
            new_mem.clear();
            new_mem = vals;
        }
        "mpush" => {
            let vals = collect_args::<String>(&args, &varmap, &mem);
            new_mem = mem.to_owned();
            new_mem.extend(vals);
        }
        "mpop" => {
            let count = if args.len() == 2 {
                lookup_var(&args[1], &varmap, &mem)
                    .to_owned()
                    .parse::<usize>()
                    .unwrap()
            } else {
                0
            };
            new_mem = mem.to_owned();
            for _ in 0..count {
                new_mem.pop();
            }
        }
        "mreset" => {}
        "len" => {
            let mut args = args.to_owned();
            let var = lookup_var(&args[1], &varmap, &mem).to_owned();
            args.remove(1);
            let vals = collect_args::<i64>(&args, &varmap, &mem);
            varmap.insert(var, vals.len().to_string());
            new_mem = mem.to_owned();
        }
        "map" => {
            let mut args = args.to_owned();
            let eoa = args.iter().position(|sep| sep == "/").unwrap();
            let map_args = &args[0..eoa].to_vec();
            let mut vals = collect_args::<String>(map_args, &varmap, &mem);
            for _ in 0..(eoa - 1) {
                args.remove(0);
            }
            // println!("{:?}", args);
            args[0] = args[2].to_owned();
            args[1] = str!("tmp");
            // println!("{:?}", args);
            for i in 0..vals.len() {
                args[2] = vals[i].to_owned();
                // println!("{:?}", args);
                match_command(&args, varmap, mem).unwrap();
                vals[i] = varmap.get("tmp").unwrap().to_owned();
            }
            // println!("{:?}", vals);
            new_mem = vals;
        }
        "sort" => {
            let mut vals = collect_args::<i64>(&args, &varmap, &mem);
            vals.sort();
            // for i in &vals[0..vals.len() - 1] {
            //     // iterate through all except last element
            //     print!("{i} ≤ ")
            // }
            // println!("{}", vals[vals.len() - 1]); // print the last element with a newline
            new_mem.clear();
            new_mem = vals.iter().map(|v| v.to_string()).collect::<Vec<String>>();
        }
        "sum" => {
            let mut args = args.to_owned();
            let var = lookup_var(&args[1], &varmap, &mem).to_owned();
            args.remove(1);
            let vals = collect_args(&args, &varmap, &mem);
            let sum: i64 = vals.iter().sum();
            varmap.insert(var, sum.to_string());
            new_mem = mem.to_owned();
        }
        "sub" => {
            let mut args = args.to_owned();
            let var = lookup_var(&args[1], &varmap, &mem).to_owned();
            args.remove(1);
            let mut vals = collect_args::<i64>(&args, &varmap, &mem);
            vals[0] = -vals[0];
            vals = vals.iter().map(|x| -x).collect::<Vec<i64>>();
            let sum: i64 = vals.iter().sum();
            varmap.insert(var, sum.to_string());
            new_mem = mem.to_owned();
        }
        "mul" => {
            let mut args = args.to_owned();
            let var = lookup_var(&args[1], &varmap, &mem).to_owned();
            args.remove(1);
            let vals = collect_args(&args, &varmap, &mem);
            let prod: i64 = vals.iter().product();
            varmap.insert(var, prod.to_string());
            new_mem = mem.to_owned();
        }
        "fac" => {
            let mut args = args.to_owned();
            let var = lookup_var(&args[1], &varmap, &mem).to_owned();
            args.remove(1);
            let base = lookup_var(&args[1], &varmap, &mem)
                .parse::<i64>()
                .unwrap_or(args[1].len() as i64);
            let res = if base == 0 {
                0
            } else {
                (1..=base).product::<i64>()
            };
            varmap.insert(var, res.to_string());
            new_mem = mem.to_owned();
        }
        "pow" => {
            let mut args = args.to_owned();
            let var = lookup_var(&args[1], &varmap, &mem).to_owned();
            args.remove(1);
            let base = lookup_var(&args[1], &varmap, &mem)
                .parse::<i64>()
                .unwrap_or(args[1].len() as i64);
            let power = lookup_var(&args[2], &varmap, &mem)
                .parse::<i64>()
                .unwrap_or(args[1].len() as i64);
            let res = base.pow(power as u32);
            // println!("{}", res);
            varmap.insert(var, res.to_string());
            new_mem = mem.to_owned();
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
    // println!("mem values: {:?}", new_mem);
    Ok(new_mem)
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
    mem: &Vec<String>,
) -> Vec<T> {
    if args[1] == "|" {
        mem.iter()
            .map(|val| val.parse::<T>().unwrap_or(T::default()))
            .collect::<Vec<T>>()
    } else if &args[1][0..1] == "&" {
        let range = &args[1][1..args[1].len()].split(":").collect::<Vec<&str>>();
        let start = range[0].parse::<usize>().unwrap();
        let end = range[1].parse::<usize>().unwrap();
        mem[start..=end]
            .iter()
            .map(|val| val.parse::<T>().unwrap_or(T::default()))
            .collect::<Vec<T>>()
    } else {
        args[1..args.len()]
            .iter()
            .map(|val| {
                lookup_var(&val, &varmap, mem)
                    .parse::<T>()
                    .unwrap_or(T::default())
            })
            .collect::<Vec<T>>()
    }
}

fn lookup_var<'a>(
    str: &'a String,
    varmap: &'a HashMap<String, String>,
    mem: &'a Vec<String>,
) -> &'a String {
    if str.contains("$") {
        let lookup = str.clone();
        match varmap.get(&lookup[1..lookup.len()]) {
            None => varmap.get("undefined").unwrap(),
            Some(val) => val,
        }
    } else if str.contains("|") {
        let idx = str[1..str.len()].parse::<usize>().unwrap();
        &mem[idx]
    } else if str.contains("&") {
        let idx = str[1..str.len()].parse::<usize>().unwrap();
        &mem[idx]
    } else {
        str
    }
}
