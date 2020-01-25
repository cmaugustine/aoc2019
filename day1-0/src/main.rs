use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {

    let mut tot = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(read) = line {
                let mass: i32 = read.trim().parse().unwrap();
                tot += mass / 3 - 2;
            }
        }
    };
    print!("total = {}", tot);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
