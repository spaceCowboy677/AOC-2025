use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Turn(i32, i32);

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines()
        .map_while(Result::ok)
        .collect())
}

fn parse_move(movement: &str) -> io::Result<i32> {
    let dir_str = movement.chars().next().unwrap();
    let dir = if dir_str == 'L' { -1 } else { 1 };
    let mag_str = &movement[1..];
    let mag = mag_str.parse::<i32>().unwrap();
    Ok(dir * mag)
}

#[allow(dead_code)]
fn execute_move_part_1(movement: i32, start: i32) -> i32 {
    let turn = (start + movement) % 100;
    if turn < 0 { 100 + turn } else { turn }
}

fn execute_move(movement: i32, start: i32) -> Turn {
    // calculates the number of times the difference crosses zero
    // during a move
    let mut crosses: i32 = movement.abs() / 100;
    let residual: i32 = movement % 100;
    let pos: i32 = start + residual;
    crosses += if pos > 99 { 1 } else { 0 };
    crosses += if pos <= 0 && start != 0 { 1 } else { 0 };
    let pos = if pos < 0 { 100 + pos % 100 } else { pos % 100 };
    println!("Moved {} spaces from {} to {} with {} crosses", 
        movement, start, pos, crosses);
    Turn(pos, crosses)
}

fn main() {
    let lines = read_lines("data/data.dat").unwrap();
    let (_pos, cnt): (i32, i32) = lines
                     .into_iter()
                     .filter_map(|s| parse_move(&s).ok())
                     .fold((50, 0i32), |(pos, count), m| {
                         let Turn(new_pos, crosses) = execute_move(m, pos);
                         let new_count = count + crosses;
                         (new_pos, new_count)
                     });
    println!("{}", cnt);
}
