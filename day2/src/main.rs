use std::fs;
use std::env;
use std::error::Error;
use std::path::Path;

fn read_file(filename: &Path) -> Result<String, Box<dyn Error>> {
    let message: String = fs::read_to_string(filename)?;
    Ok(message)
}

fn get_factors(n: i32) -> Vec<i32> {
    // return a list of factors for input n
    let m = (n as f32).sqrt() as i32;
    (1..=m).filter(|x| n % x == 0)
        .flat_map(|x| {
            let y = n / x;
            if x == y { vec![x] } else { vec![x, y] }})
        .collect()
}

fn is_invalid(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    get_factors(n as i32)
        .into_iter()
        .map(|x| x as usize)
        .filter(|&x| x > 1)
        .any(|x| {
            let block = n / x;
            c.chunks(block).all(|chunk| chunk == &c[..block])
        })
} 

#[allow(dead_code)]
fn is_invalid_part1(s: &str) -> bool {
    // determine if a string is invalid by comparing the left and right sides
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    (n % 2 == 0) && c.iter()
        .take(n / 2)
        .eq(c.iter().skip(n / 2))
}

fn range(s: &str) -> std::ops::RangeInclusive<u64> {
    let (a, b) = s.split_once('-').unwrap();
    a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
}

fn check_range(s: &str) -> u64 {
    // for a given range sum all elements together that meet criteria
    range(s)
        .filter(|x| is_invalid(&x.to_string()))
        // .inspect(|x| println!("  {} is valid", x))
        .sum()
}

fn main() {
    println!("{}",read_file(
        Path::new(&env::args().last().unwrap()))
        .unwrap()
        .split(',')
        .map(check_range)
        .sum::<u64>());
}
