use std::collections::HashMap;
use std::env::args;

pub fn parse_args() -> HashMap<String, String> {
    let mut toret = HashMap::new();
    let mut index = String::new();
    for i in args() {
        if i.starts_with("-") {
            index = index_tweak(&i);
            toret.insert(index.clone(), String::from(""));
        } else {
            toret.insert(index.clone(), i);
        }
    }
    toret
}

fn index_tweak<'a>(i: &String) -> String {
    let mut toret = i.replace("-", "");
    if toret.starts_with("-") {
        toret = toret.replace("-", "");
    }

    match toret.as_str() {
        "dir" | "expr" => {}
        "d" => toret = String::from("dir"),
        "n" => toret = String::from("name"),
        "i" => toret = String::from("ignore-extensions"),
        _ => eprint!("Unkown parameter: {}", toret),
    }

    toret
}

pub fn print_args(args: &HashMap<String, String>) {
    for i in args {
        println!("{}: {}", i.0, i.1);
    }
}
