use std::fs;
use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;

fn main() {
    // challenge1();
    challenge2();
}

fn read_text_at_once(file_path:&str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}

fn challenge1() {
    let debug = true;
    let file_path = "src/resource/input.txt";
    let data = read_text_at_once(file_path);
    let answer = solution(data, debug);
    println!("Solution 1 - Total Score = {}", answer);
}

fn challenge2() {
    let debug = true;
    let file_path = "src/resource/input.txt";
    let data = read_text_at_once(file_path);
    let answer = solution2(data, debug);
    println!("Solution 2 - Total Score = {}", answer);
}

fn inplace_intersection<T>(a: &mut HashSet<T>, b: &mut HashSet<T>) -> HashSet<T>
where
    T: Hash,
    T: Eq,
{
    let x: HashSet<(T, bool)> = a
        .drain()
        .map(|v| {
            let intersects = b.contains(&v);
            (v, intersects)
        })
        .collect();

    let mut c = HashSet::new();
    for (v, is_inter) in x {
        if is_inter {
            c.insert(v);
        } else {
            a.insert(v);
        }
    }

    b.retain(|v| !c.contains(&v));

    c
}

fn solution(data:String, debug:bool) -> u32 {
    let mut running_score: u32 = 0;
    for line in data.lines() {
        let string_length = line.chars().count();
        if debug{println!("{}, length={}",line, string_length);}
        let (first_half, second_half) = line.split_at(string_length/2);
        if debug{println!("First Half: {}. Second Half: {}", first_half, second_half);}
        
        let mut first_set: HashSet<char> = first_half.chars().collect();
        let mut second_set: HashSet<char> = second_half.chars().collect();
        
        let shared_character_hash = inplace_intersection(&mut first_set, &mut second_set);
        let shared_character: String = shared_character_hash.into_iter().sorted().collect();
        if debug{println!("Shared Character: {}", shared_character);}
        let mut character = shared_character.as_bytes()[0];
        if debug{println!("Character U8: {}", character);}
        if character > 96 {
            character = character - 96;
        }
        else {
            character = character - 38;
        }
        if debug{println!("Character value: {}", character);}
        running_score = running_score + character as u32;
    }
    running_score
}

fn find_shared_characters(string1:String, string2: String, string3: String, debug:bool) -> u8 {
    let mut first_set: HashSet<char> = string1.chars().collect();
    let mut second_set: HashSet<char> = string2.chars().collect();
    let mut third_set: HashSet<char> = string3.chars().collect();

    let mut shared_character_12_hash = inplace_intersection(&mut first_set, &mut second_set);
    let shared_character_123_hash = inplace_intersection(&mut shared_character_12_hash, &mut third_set);
    let shared_character: String = shared_character_123_hash.into_iter().sorted().collect();
    if debug{println!("Shared Character: {}", shared_character);}
    shared_character.as_bytes()[0] as u8
}

fn translate_char_u8_to_value(mut character:u8, debug:bool) -> u32 {
    if debug{println!("Character U8: {}", character);}
    if character > 96 {
        character = character - 96;
    }
    else {
        character = character - 38;
    }
    if debug{println!("Character value: {}", character);}
    character as u32
}

fn solution2(data:String, debug:bool) -> u32 {
    let mut running_score: u32 = 0;
    for mut lines in &data.lines().chunks(3) {
        let string1 = lines.next().expect("Could not read first string, maybe end of file").to_string();
        let string2 = lines.next().expect("Could not read second string, likely broken").to_string();
        let string3:String = lines.next().expect("Could not read second string, likely broken").to_string();

        let character: u8 = find_shared_characters(string1, string2, string3, debug);
        running_score = running_score + translate_char_u8_to_value(character, debug);
    }
    running_score
}
