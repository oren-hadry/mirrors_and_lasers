
// Note: Used AI assistance for Rust syntax, all design decisions and logic are mine.

use std::env;
use std::fs;
use std::io::Read;

mod solver;
use solver::solve;


struct PuzzleInput {
    r: usize,
    c: usize,
    right_mirrors: Vec<(usize, usize)>,
    left_mirrors: Vec<(usize, usize)>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut lines = input.lines();
    let first: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (r, c, m, n) = (first[0], first[1], first[2], first[3]);

    let right_mirrors: Vec<(usize, usize)> = (0..m).map(|_| {
        let parts: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    }).collect();

    let left_mirrors: Vec<(usize, usize)> = (0..n).map(|_| {
        let parts: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    }).collect();

    PuzzleInput { r, c, right_mirrors, left_mirrors }
}

fn main() {
    // read input from file or stdin
    let args: Vec<String> = env::args().collect();

    let input = if let Some(path) = args.get(1) {
        fs::read_to_string(path).expect("Failed to read file")
    } else {
        // read from stdin
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let puzzle = parse_input(&input);
    println!("{}", solve(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 6 1 4\n2 3\n1 2\n2 5\n4 2\n5 5";
        let puzzle = parse_input(input);
        assert_eq!(solve(&puzzle), "2 4 3");
    }

    #[test]
    fn test_case_2() {
        let input = "100 100 0 2\n1 77\n100 77";
        let puzzle = parse_input(input);
        assert_eq!(solve(&puzzle), "0");
    }

    #[test]
    fn test_case_3() {
        let input = "100 100 0 0";
        let puzzle = parse_input(input);
        assert_eq!(solve(&puzzle), "impossible");
    }
}
