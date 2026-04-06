# Mirrors and Lasers

> Note: Used AI assistance for Rust syntax; all design decisions and logic are my own.

## Running

```bash
# from stdin
echo "5 6 1 4\n2 3\n1 2\n2 5\n4 2\n5 5" | cargo run

# tests
cargo test
```

## Problem

A laser enters a grid at the top-left and must exit at the bottom-right, deflected by `/` and `\` mirrors. Given an existing mirror layout, find how many empty cells, if any, could receive a new mirror to open the safe.

**Input** (stdin):
- Line 1: `r c m n` — grid dimensions, count of `/` mirrors, count of `\` mirrors
- Next `m` lines: `row col` positions of `/` mirrors
- Next `n` lines: `row col` positions of `\` mirrors

**Output**:
- `0` — beam already reaches the exit without any insertion
- `k r c` — `k` valid insertion positions exist; `(r, c)` is the lexicographically smallest
- `impossible` — no single mirror insertion can open the safe

## Approach

The grid can be up to 1,000,000 × 1,000,000, so allocating it is not viable. Mirrors are stored in two sorted maps (by row, by column), allowing O(log M) jumps to the next mirror in any direction.

**Beam tracing**: trace forward from the laser entry and backward from the detector. Each trace produces a list of horizontal and vertical segments.

**Finding valid insertions**: a mirror placed at cell `(r, c)` works if a forward segment passes through it horizontally and a backward segment passes through it vertically (or vice versa). Finding all such crossings uses a sweep line over columns, avoiding the O(H·V) brute-force comparison.

## Complexity

Let M = total mirrors, S = segments per trace (at most 2M+1, since each mirror can be hit from two sides), K = valid insertion cells found.

| Phase | Time |
|---|---|
| Build mirror map | O(M log M) |
| Trace forward + backward | O(M log M) |
| Sweep line intersection | O(S log S + K log K) |
| **Overall** | **O(M log M + K log M)** |

Memory: O(M + K) — no grid allocation.
