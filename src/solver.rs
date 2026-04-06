use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::PuzzleInput;

// '/' mirror: right→up, left→down, up→right, down→left
// '\' mirror: right→down, left→up, up→left, down→right
#[derive(Clone, Copy, PartialEq)]
enum Mirror { Fwd, Back }  // Fwd = '/', Back = '\'

#[derive(Clone, Copy, PartialEq)]
enum Dir { Right, Left, Up, Down }

// ---- Mirror lookup --------------------------------------------------------
// Stores mirrors sorted by position within each row and each column.
// This lets us find the "next mirror in this direction" in O(log n) using
// a BTreeMap range query — no need to scan the whole row/column.
struct MirrorMap {
    by_row: BTreeMap<usize, BTreeMap<usize, Mirror>>,  // row -> (col -> mirror)
    by_col: BTreeMap<usize, BTreeMap<usize, Mirror>>,  // col -> (row -> mirror)
}

impl MirrorMap {
    fn build(puzzle: &PuzzleInput) -> Self {
        let mut by_row: BTreeMap<usize, BTreeMap<usize, Mirror>> = BTreeMap::new();
        let mut by_col: BTreeMap<usize, BTreeMap<usize, Mirror>> = BTreeMap::new();

        // right_mirrors = '/' right-leaning (first m lines of input)
        for &(r, c) in &puzzle.right_mirrors {
            by_row.entry(r).or_default().insert(c, Mirror::Fwd);
            by_col.entry(c).or_default().insert(r, Mirror::Fwd);
        }
        // left_mirrors = '\' left-leaning (next n lines of input)
        for &(r, c) in &puzzle.left_mirrors {
            by_row.entry(r).or_default().insert(c, Mirror::Back);
            by_col.entry(c).or_default().insert(r, Mirror::Back);
        }

        MirrorMap { by_row, by_col }
    }

    fn next_right(&self, row: usize, from_col: usize) -> Option<(usize, Mirror)> {
        self.by_row.get(&row)?.range((from_col + 1)..).next().map(|(&c, &m)| (c, m))
    }

    fn next_left(&self, row: usize, from_col: usize) -> Option<(usize, Mirror)> {
        self.by_row.get(&row)?.range(..from_col).next_back().map(|(&c, &m)| (c, m))
    }

    fn next_down(&self, col: usize, from_row: usize) -> Option<(usize, Mirror)> {
        self.by_col.get(&col)?.range((from_row + 1)..).next().map(|(&r, &m)| (r, m))
    }

    fn next_up(&self, col: usize, from_row: usize) -> Option<(usize, Mirror)> {
        self.by_col.get(&col)?.range(..from_row).next_back().map(|(&r, &m)| (r, m))
    }
}

// ---- Reflection -----------------------------------------------------------
fn reflect(dir: Dir, mirror: Mirror) -> Dir {
    match (dir, mirror) {
        (Dir::Right, Mirror::Fwd)  => Dir::Up,
        (Dir::Right, Mirror::Back) => Dir::Down,
        (Dir::Left,  Mirror::Fwd)  => Dir::Down,
        (Dir::Left,  Mirror::Back) => Dir::Up,
        (Dir::Up,    Mirror::Fwd)  => Dir::Right,
        (Dir::Up,    Mirror::Back) => Dir::Left,
        (Dir::Down,  Mirror::Fwd)  => Dir::Left,
        (Dir::Down,  Mirror::Back) => Dir::Right,
    }
}

// ---- Segment --------------------------------------------------------------
// A straight stretch the beam travels before hitting the next mirror (or wall).
// For a horizontal segment: beam is in row `fixed`, passing through all empty
//   cells with column strictly between `from` and `to`.
// For a vertical segment: beam is in col `fixed`, rows strictly between from/to.
// (from and to are the mirror/edge positions, not the cells themselves.)
struct Segment {
    horizontal: bool,
    fixed: usize,   // row (horizontal) or col (vertical)
    from: usize,    // exclusive start  (always < to)
    to: usize,      // exclusive end
}

// ---- Beam tracing ---------------------------------------------------------
// Follows the beam step by step, recording each segment it travels.
// Returns (reached_goal, segments).
//
// Forward:  start at (row=1, col=0, dir=Right)  — beam enters top-left
// Backward: start at (row=r, col=c+1, dir=Left) — beam enters bottom-right (detector side)
fn trace(
    map: &MirrorMap,
    grid_r: usize,
    grid_c: usize,
    start_row: usize,
    start_col: usize,
    start_dir: Dir,
) -> (bool, Vec<Segment>) {
    let (mut row, mut col, mut dir) = (start_row, start_col, start_dir);
    let mut segs = Vec::new();

    loop {
        match dir {
            Dir::Right => match map.next_right(row, col) {
                Some((nc, mirror)) => {
                    segs.push(Segment { horizontal: true, fixed: row, from: col, to: nc });
                    col = nc;
                    dir = reflect(dir, mirror);
                }
                None => {
                    segs.push(Segment { horizontal: true, fixed: row, from: col, to: grid_c + 1 });
                    // success = beam exits the right side of the bottom row
                    return (row == grid_r, segs);
                }
            },
            Dir::Left => match map.next_left(row, col) {
                Some((nc, mirror)) => {
                    segs.push(Segment { horizontal: true, fixed: row, from: nc, to: col });
                    col = nc;
                    dir = reflect(dir, mirror);
                }
                None => {
                    segs.push(Segment { horizontal: true, fixed: row, from: 0, to: col });
                    // backward trace exits the left side of the top row
                    return (row == 1, segs);
                }
            },
            Dir::Down => match map.next_down(col, row) {
                Some((nr, mirror)) => {
                    segs.push(Segment { horizontal: false, fixed: col, from: row, to: nr });
                    row = nr;
                    dir = reflect(dir, mirror);
                }
                None => {
                    segs.push(Segment { horizontal: false, fixed: col, from: row, to: grid_r + 1 });
                    return (false, segs);
                }
            },
            Dir::Up => match map.next_up(col, row) {
                Some((nr, mirror)) => {
                    segs.push(Segment { horizontal: false, fixed: col, from: nr, to: row });
                    row = nr;
                    dir = reflect(dir, mirror);
                }
                None => {
                    segs.push(Segment { horizontal: false, fixed: col, from: 0, to: row });
                    return (false, segs);
                }
            },
        }
    }
}

// ---- Intersection finding -------------------------------------------------
// Given a set of horizontal segments and a set of vertical segments, find all
// cells (row, col) that lie strictly inside one of each.
//
// Uses a sweep line over columns:
//   - "open"  a horizontal segment when we pass its left boundary
//   - "query" when we reach a vertical segment's column
//   - "close" a horizontal segment when we reach its right boundary
//
// Event ordering at the same column:
//   close (0) < query (1) < open (2)
// This enforces strict interior: a segment with boundary at C is NOT active
// during a query at C.
//
// O((H + V) log(H + V) + K log K) where K = number of intersections found.
// The K log K term comes from inserting each result into the output BTreeSet.
fn intersect_h_v(
    h_segs: &[&Segment],
    v_segs: &[&Segment],
    mirrors: &HashSet<(usize, usize)>,
    result: &mut BTreeSet<(usize, usize)>,
) {
    // Events: (col, type, payload)
    // payload is a row for open/close events, an index into v_segs for query events
    let mut events: Vec<(usize, u8, usize)> = Vec::new();

    for seg in h_segs {
        events.push((seg.from, 2, seg.fixed));  // open: add this row after `from`
        events.push((seg.to,   0, seg.fixed));  // close: remove this row at `to`
    }
    for (i, seg) in v_segs.iter().enumerate() {
        events.push((seg.fixed, 1, i));         // query at this column
    }

    events.sort_unstable_by_key(|&(col, typ, _)| (col, typ));

    // Active rows: BTreeMap<row, count> so we handle a row appearing in multiple
    // segments (the beam can cross the same row in different column ranges).
    let mut active: BTreeMap<usize, usize> = BTreeMap::new();

    for (col, typ, data) in events {
        match typ {
            0 => {  // close: remove row
                let cnt = active.entry(data).or_insert(0);
                if *cnt <= 1 { active.remove(&data); } else { *cnt -= 1; }
            }
            1 => {  // query: find all active rows inside the vertical segment's row range
                let seg = v_segs[data];
                for (&row, _) in active.range((seg.from + 1)..seg.to) {
                    if !mirrors.contains(&(row, col)) {
                        result.insert((row, col));
                    }
                }
            }
            2 => {  // open: add row
                *active.entry(data).or_insert(0) += 1;
            }
            _ => unreachable!(),
        }
    }
}

// ---- Public entry point ---------------------------------------------------
pub fn solve(puzzle: &PuzzleInput) -> String {
    let map = MirrorMap::build(puzzle);

    // We need to know which cells already have a mirror (can't insert there)
    let mirrors: HashSet<(usize, usize)> = puzzle.right_mirrors.iter()
        .chain(puzzle.left_mirrors.iter())
        .copied()
        .collect();

    // Step 1: trace the beam forward (top-left → bottom-right)
    let (success, fwd_segs) = trace(&map, puzzle.r, puzzle.c, 1, 0, Dir::Right);
    if success {
        return "0".to_string();  // safe already opens without any mirror
    }

    // Step 2: trace backward from the detector (bottom-right → top-left)
    // This gives us the path the beam would need to follow from the exit side.
    let (_, bwd_segs) = trace(&map, puzzle.r, puzzle.c, puzzle.r, puzzle.c + 1, Dir::Left);

    // Step 3: find cells where we can bridge the two paths with a single mirror.
    // A valid insertion is a cell where the forward path travels horizontally
    // AND the backward path travels vertically (or vice versa). Inserting a
    // mirror there deflects one path onto the other, connecting them.
    let fwd_h: Vec<&Segment> = fwd_segs.iter().filter(|s|  s.horizontal).collect();
    let fwd_v: Vec<&Segment> = fwd_segs.iter().filter(|s| !s.horizontal).collect();
    let bwd_h: Vec<&Segment> = bwd_segs.iter().filter(|s|  s.horizontal).collect();
    let bwd_v: Vec<&Segment> = bwd_segs.iter().filter(|s| !s.horizontal).collect();

    let mut valid: BTreeSet<(usize, usize)> = BTreeSet::new();
    intersect_h_v(&fwd_h, &bwd_v, &mirrors, &mut valid);  // forward-horiz × backward-vert
    intersect_h_v(&bwd_h, &fwd_v, &mirrors, &mut valid);  // backward-horiz × forward-vert

    if valid.is_empty() {
        return "impossible".to_string();
    }

    // BTreeSet is sorted, so the first entry is lexicographically smallest (row, col)
    let &(r, c) = valid.iter().next().unwrap();
    format!("{} {} {}", valid.len(), r, c)
}
