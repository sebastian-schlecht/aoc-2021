use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use arrayvec::ArrayVec;

fn mapper(res: Result<String, std::io::Error>) -> i32 {
    if let Ok(res_line) = res {
        return res_line.parse::<i32>().unwrap();
    }
    return 0;
}

fn main() {
    let mut counter = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut previous_measurement = i32::MAX;

        // map lines iterator to an array of integers
        let input: ArrayVec<_, 2000> = lines.map(mapper).collect();

        for i in 0..1998 {
            let sum = input[i] + input[i+1] + input[i+2];
            let is_bigger = sum > previous_measurement;
            previous_measurement = sum;
            if is_bigger {
                counter = counter + 1;
            }
            println!("{}, {}, {} // {}, ({})",input[i], input[i+1], input[i+2], sum, is_bigger);
        }
        println!("Result: {}", counter);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
