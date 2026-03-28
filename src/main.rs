
// problem space: grid of size [r, c] where each cell can have 3 states [empty, left_mirror, right_mirror]
// input is :
//     [r, c, m, and n] r is the number of rows, c is the number of columns
//     [m] lines of left mirror positions [x, y]
//     [n] lines of right mirror positions [x, y]

// input from file or stdin as default

// output: 
// 0  - safe is open without inserting a mirror
// k, r, c - if the safe does not open without inserting a mirror, there are
//           exactly k positions where inserting a mirror opens the safe, and (r, c)
//           is the lexicographically smallest such row, column position. A position
//           where both a / and a \mirror open the safe counts just once.
// -1 - impossible if the safe cannot be opened with or without inserting a
// mirror.

// solution:
//     option A: brute force - run spanning tree on greed - check all possible positions for mirror
//     option B: 
//   check for solution:  run from start to end and see if we have reach end point, 
//                        be carfule from a loop ? (not possible as mirror not transparent), find next mirror in O(log(n)) sort the mirrors by row, col.
//   check inserting mirror: run on grid from both ends, each cell we will indicate entry and exit points
//                        if we have found a cell that have different (90 degrees)entry and exit points, 
//                        we have found a solution. otherwise, we have no solution.

use std::env;
use std::fs;
use std::io::Read;

mod solver;
use solver::solve;

// enum for cell value
#[derive(Clone)]
enum CellValue {
    Empty,
    LeftMirror,
    RightMirror,
}

#[derive(Clone)]
struct Cell {
    state: CellValue,
}

struct PuzzleInput {
    r: usize,
    c: usize,
    left_mirrors: Vec<(usize, usize)>,
    right_mirrors: Vec<(usize, usize)>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut lines = input.lines();
    let first: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (r, c, m, n) = (first[0], first[1], first[2], first[3]);

    let left_mirrors: Vec<(usize, usize)> = (0..m).map(|_| {
        let parts: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    }).collect();

    let right_mirrors: Vec<(usize, usize)> = (0..n).map(|_| {
        let parts: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    }).collect();

    PuzzleInput { r, c, left_mirrors, right_mirrors }
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
        let input = "5 6 1 4\n1 2\n2 5\n4 2\n5 5";
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
