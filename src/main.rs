use std::collections::HashMap;
use std::fs;
mod args_parser;
mod file_info;

fn main() {
    let args = args_parser::parse_args();

    /*for i in args {
        println!("{}: {}", i.0, i.1);
    }*/

    args_parser::print_args(&args);

    let dir = match args.get("dir") {
        Some(x) => x,
        None => ".",
    };

    let new_pattern = match args.get("name") {
        Some(x) => x,
        None => "",
    };

    let ignore_extensions = match args.get("ignore-extensions") {
        Some(x) => x.eq("true"),
        None => false,
    };

    let mut files: Vec<file_info::FileInfo> = Vec::new();

    for i in fs::read_dir(dir).unwrap() {
        let entry = i.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let file = path.file_name().unwrap();
        match file.to_str() {
            Some(x) => {
                let fname = String::from(x);
                let divided: Vec<&str> = fname.split(".").collect();
                let len = divided.len();
                let mut name: String = String::from(divided[0]);
                for i in &divided[1..len - 1] {
                    name = vec![name.as_str(), i].join(".");
                }
                files.push(file_info::FileInfo {
                    name,
                    extension: String::from(divided[len - 1]),
                });
            }
            None => eprintln!("Error with file in path: {}", path.display()),
        }
    }
    println!("----------");
    for i in &files {
        println!("{}", i);
    }

    if ignore_extensions {
        let mut index = 0;
        for i in &files {
            index += 1;
            rename(dir, &i, new_pattern, &index);
        }
    } else {
        let mut counters: HashMap<String, i32> = HashMap::new();
        for i in &files {
            let index = match counters.get_mut(&i.extension) {
                Some(x) => {
                    *x += 1;
                    *x
                }

                None => {
                    counters.insert(i.extension.clone(), 1);
                    1
                }
            };
            rename(dir, &i, new_pattern, &index);
        }
    }
}

fn rename(dir: &str, file: &file_info::FileInfo, new_pattern: &str, index: &i32) {
    let from_path = vec![dir, "/", &file.name, ".", &file.extension].join("");
    let to_path = vec![
        dir,
        "/",
        new_pattern,
        &index.to_string(),
        ".",
        &file.extension,
    ]
    .join("");
    match fs::rename(from_path.clone(), to_path.clone()) {
        Ok(_x) => {
            println!("{} renamed!", file.name)
        }
        Err(x) => {
            println!(
                "{} couldn't be renamed: {} || from:{} || to:{}",
                file.extension, x, from_path, to_path
            )
        }
    }
}
