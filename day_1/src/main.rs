use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

// fn diagnose_what_bloody_directory_relative_paths_are_from() {
//     let path = env::current_dir()?;
//     println!("The current directory is {}", path.display());
//     for entry in fs::read_dir(".").unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path();
//         if path.is_dir() {
//             println!("{:?} is a dir", path);
//         } else {
//             println!("{:?} is a file", path);
//         }
//     }
//     trial();
// }

fn main() {
    trial();
}

fn trial() {
    let file_path = "resource/data.txt";
    println!("In file {}", file_path);
    data = read_text_at_once(file_path)
    for line in data.lines() {
        println!("{}", line)
    }

}


fn read_text_at_once(file_path:String) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    contents
}