



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

fn main() {
    println!("Hello, world!");
}
