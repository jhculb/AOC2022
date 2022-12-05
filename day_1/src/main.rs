use std::fs;


fn main() {
    challenge2();
}


fn challenge1() {
    let debug = false;
    let file_path = "resource/input.txt";
    if debug {println!("In file {}", file_path);}
    let data = read_text_at_once(file_path);
    let answer = find_maximum(data, debug);
    println!("Maximum calories = {}", answer);
}

fn challenge2() {
    let debug = false;
    let file_path = "resource/input.txt";
    if debug {println!("In file {}", file_path);}
    let data = read_text_at_once(file_path);
    let answer = find_maximum2(data, debug);
    println!("Top 3 calorie carrying gnomes = {}, {}, {}", answer[0], answer[1], answer[2]);
    let sum = answer[0] + answer[1] + answer[2];
    println!("Total of top = {}", sum)
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

fn find_maximum2(data:String, debug:bool) -> [i32;3] {
    let mut running_total = 0;
    let mut max = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for line in data.lines() {
        if debug{println!("{}", line);}
        if line.eq("") {
            if debug{println!("Current Top 3 = {}, {}, {}", max, max2, max3);}
            
            if running_total > max {
                max3 = max2;
                max2 = max;
                max = running_total;
            }
            else if running_total > max2 {
                max3 = max2;
                max2 = running_total;
            }
            else if running_total > max3 {
                max3 = running_total;
            }
            

            if debug{println!("New Top 3 = {}, {}, {}", max, max2, max3);}
            
            running_total = 0;
        }
        else {
            if debug {println!("rt = {}", running_total);}

            let converted_string = line.parse::<i32>().unwrap();
            running_total = running_total + converted_string;

            if debug {println!("Running total = {}\n", running_total);}
        }
    }
    [max, max2, max3]
}

fn read_text_at_once(file_path:&str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}