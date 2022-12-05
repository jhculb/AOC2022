use std::fs;


fn main() {
    input1();
}

fn trial() {
    let debug = false;
    let file_path = "resource/data.txt";
    if debug {println!("In file {}", file_path);}
    let data = read_text_at_once(file_path);
    let answer = find_maximum(data, debug);
    println!("Maximum calories = {}", answer);
}

fn input1() {
    let debug = false;
    let file_path = "resource/input1.txt";
    if debug {println!("In file {}", file_path);}
    let data = read_text_at_once(file_path);
    let answer = find_maximum(data, debug);
    println!("Maximum calories = {}", answer);
}

fn challenge2() {
    
}

fn find_maximum(data:String, debug:bool) -> i32 {
    let mut running_total = 0;
    let mut max = 0;
    for line in data.lines() {
        if debug{println!("{}", line);}
        if line.eq("") {
            if debug{println!("Current Maximum = {}", max);}
            
            if running_total > max {
                max = running_total;
            }

            if debug{println!("New Maximum = {}", max);}
            
            running_total = 0;
        }
        else {
            if debug {println!("rt = {}", running_total);}

            let converted_string = line.parse::<i32>().unwrap();
            running_total = running_total + converted_string;

            if debug {println!("Running total = {}\n", running_total);}
        }
    }
    max
}


fn read_text_at_once(file_path:&str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}