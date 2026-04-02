## How to run this project:

# from stdin
echo "5 6 1 4\n2 3\n1 2\n2 5\n4 2\n5 5" | cargo run

# tests
cargo test


## Problem description

Note: Used AI assistance for Rust syntax, all design decisions and logic are mine.

problem space: grid of size [r, c] where each cell can have 3 states [empty, left_mirror, right_mirror]
input is :
    [r, c, m, and n] r is the number of rows, c is the number of columns
    [m] lines of left mirror positions [x, y]
    [n] lines of right mirror positions [x, y]

input from stdin

output: 
0  - safe is open without inserting a mirror
k, r, c - if the safe does not open without inserting a mirror, there are
          exactly k positions where inserting a mirror opens the safe, and (r, c)
          is the lexicographically smallest such row, column position. A position
          where both a / and a \mirror open the safe counts just once.
-1 - impossible if the safe cannot be opened with or without inserting a
mirror.

## solution:
  option A: brute force - run spanning tree on greed - check all possible positions for mirror
  option B: 
  check for solution:  run from start to end and see if we have reach end point, regarding loops (not possible as mirror not transparent), find next mirror in O(log(n)) sort the mirrors by row, col.
  check inserting mirror: run on grid from both ends, each cell we will indicate entry and exit points
  if we have found a cell that have different (90 degrees)entry and exit points, 
  we have found a solution. otherwise, we have no solution.


## Design Decisions

I went with option B. The grid can be up to 1,000,000 x 1,000,000 — we can never allocate it.
Instead, mirrors are stored in two sorted maps: one indexed by row, one by column.
This lets us jump to the next mirror in any direction in O(log n), skipping all empty cells.

The core idea: trace the beam forward from the laser entry, and backward from the detector.
Each trace produces a set of horizontal and vertical segments.
A mirror insertion works when a forward-horizontal segment crosses a backward-vertical segment
(or forward-vertical crosses backward-horizontal) at an empty cell — the mirror at that cell
deflects one path onto the other, completing the circuit.

To find all such crossings efficiently I use a sweep line over columns, which avoids the
O(H*V) brute-force comparison of every segment pair.

## Complexity

Let M = total number of mirrors (m + n), S = number of segments each trace produces (at most M+1).

- Building the mirror map: O(M log M)
- Tracing the beam (forward + backward): O(M log M) — each step is one BTreeMap range query
- Sweep line intersection: O(S log S + K) where K = number of valid insertion cells found
- Overall: O(M log M + K)

Memory: O(M) for the mirror maps, O(S) for segments — no grid allocation.
