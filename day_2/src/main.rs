use std::fs;


fn main() {
    challenge1();
    challenge2();
}


fn challenge1() {
    let debug = false;
    let file_path = "resource/input.txt";
    let data = read_text_at_once(file_path);
    let answer = solution(data, debug);
    println!("Solution 1 - Total Score = {}", answer);
}

fn challenge2() {
    let debug = false;
    let file_path = "resource/input.txt";
    let data = read_text_at_once(file_path);
    let answer = solution2(data, debug);
    println!("Solution 2 - Total Score = {}", answer);
}

fn score_play(their_play:&str, my_play:&str) -> i32 {
    let score: i32 = if their_play.eq("A") && my_play.eq("X") {return 1+3} else
    if their_play.eq("A") && my_play.eq("Y") {2+6} else
    if their_play.eq("A") && my_play.eq("Z") {3+0} else
    if their_play.eq("B") && my_play.eq("X") {1+0} else
    if their_play.eq("B") && my_play.eq("Y") {2+3} else
    if their_play.eq("B") && my_play.eq("Z") {3+6} else
    if their_play.eq("C") && my_play.eq("X") {1+6} else
    if their_play.eq("C") && my_play.eq("Y") {2+0} else
    {3+3};
    return score;

}

fn solution(data:String, debug:bool) -> i32 {
    let mut running_score = 0;
    for line in data.lines() {
        if debug{println!("{}", line);}
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();
        running_score = running_score + score_play(vec[0], vec[1]);
    }
    running_score
}


fn score_play2(their_play:&str, my_play:String) -> i32 {
    let score: i32 = if their_play.eq("A") && my_play.eq("X") {return 1+3} else
    if their_play.eq("A") && my_play.eq("Y") {2+6} else
    if their_play.eq("A") && my_play.eq("Z") {3+0} else
    if their_play.eq("B") && my_play.eq("X") {1+0} else
    if their_play.eq("B") && my_play.eq("Y") {2+3} else
    if their_play.eq("B") && my_play.eq("Z") {3+6} else
    if their_play.eq("C") && my_play.eq("X") {1+6} else
    if their_play.eq("C") && my_play.eq("Y") {2+0} else
    {3+3};
    return score;

}

fn compute_my_play(their_play:&str, my_code:&str) -> String {
    let selection: &str = if their_play.eq("A") && my_code.eq("X") {"Z"} else
    if their_play.eq("A") && my_code.eq("Y") {"X"} else
    if their_play.eq("A") && my_code.eq("Z") {"Y"} else
    if their_play.eq("B") && my_code.eq("X") {"X"} else
    if their_play.eq("B") && my_code.eq("Y") {"Y"} else
    if their_play.eq("B") && my_code.eq("Z") {"Z"} else
    if their_play.eq("C") && my_code.eq("X") {"Y"} else
    if their_play.eq("C") && my_code.eq("Y") {"Z"} else
    {"X"};
    return selection.to_owned();
}

fn solution2(data:String, debug:bool) -> i32 {
    let mut running_score = 0;
    for line in data.lines() {
        if debug{println!("{}", line);}
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();
        running_score = running_score + score_play2(vec[0], compute_my_play(vec[0],&vec[1]));
    }
    running_score
}

fn read_text_at_once(file_path:&str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}