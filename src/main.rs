use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (path, replacer) = handle_args(args).unwrap();
    let replaced = ["hyprland", "niri", "sway", "dwl", "mango"];

    let file_content = fs::read_to_string(&path).expect("file should exist");

    let result = find_and_replace(&replaced, replacer.as_str(), &file_content);
    fs::write(path, result).unwrap();
}

fn find_and_replace(replaced: &[&str], replacer: &str, string: &String) -> String {
    let mut contents = string.clone();

    for imposter in replaced {
        if let Some(_) = contents.find(imposter) {
            contents = contents.replace(imposter, replacer);
        }
    }

    return contents;
}

fn handle_args(args: Vec<String>) -> Option<(String, String)> {
    let usage = "USAGE: wcc [FULL PATH] [REPLACER]";

    if args.len() != 3 {
        eprintln!("{}", usage);
        return None;
    }

    let mut it = args.into_iter();

    let _ = it.next().unwrap();
    let path = it.next().unwrap();
    let replacer = it.next().unwrap();

    return Some((path, replacer));
}
