use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use arrayvec::ArrayVec;

fn mapper(res: Result<String, std::io::Error>) -> String {
    if let Ok(res_line) = res {
        return res_line;
    }
    return " ".to_string();
}

fn main() {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        // map lines iterator to an array of integers
        let input: ArrayVec<_, 1000> = lines.map(mapper).collect();

        for i in 0..input.len() {
            let command: Vec<&str> = input[i].split(" ").collect();

            let direction = command[0];
            let value = command[1].parse::<i32>().unwrap();
            
            if direction == "forward" {
                depth = depth + (aim * value);
                pos = pos + value;
            } else if direction == "up" {
                aim = aim - value;
            } else if direction == "down" {
                aim = aim + value;
            }
        }
        println!("Depth: {}, Position: {}, Multiplied: {}", depth, pos, depth * pos);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
