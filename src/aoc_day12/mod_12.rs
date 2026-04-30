// Advent of Code 2024, Day 12: Packing Presents
// You are given a set of 2D Shapes (wxh)
//for each region you must place a specific number if each shape
// All shape must be placed aligned to the grid (no partial cells)
//GOAL : count how many regions are feasible (can fit all shapes)


use std::collections::HashSet;


pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day12/input_day12.txt")
        .expect("Cannot read input_day12.txt");
    println!("Day 12 p1 = {}", p1(&input));
}

type Cells = Vec<(i32, i32)>;

// ---------- Shape transforms ----------

fn normalize(cells: &[(i32, i32)]) -> Cells {
    let min_r = cells.iter().map(|&(r, _)| r).min().unwrap();
    let min_c = cells.iter().map(|&(_, c)| c).min().unwrap();
    let mut out: Cells = cells.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();
    out.sort();
    out
}

fn rotate90(cells: &[(i32, i32)]) -> Cells {
    cells.iter().map(|&(r, c)| (c, -r)).collect()
}

fn flip(cells: &[(i32, i32)]) -> Cells {
    cells.iter().map(|&(r, c)| (r, -c)).collect()
}

/// Up to 8 orientations: 4 rotations of the original, 4 of the mirror.
/// Symmetric shapes return fewer unique ones.
fn all_orientations(cells: &[(i32, i32)]) -> Vec<Cells> {
    let mut seen: HashSet<Cells> = HashSet::new();
    let mut result: Vec<Cells> = Vec::new();

    let mut cur = cells.to_vec();
    for _ in 0..4 {
        let n = normalize(&cur);
        if seen.insert(n.clone()) {
            result.push(n);
        }
        cur = rotate90(&cur);
    }
    let mut cur = flip(cells);
    for _ in 0..4 {
        let n = normalize(&cur);
        if seen.insert(n.clone()) {
            result.push(n);
        }
        cur = rotate90(&cur);
    }
    result
}

// ---------- Parsing ----------

struct Region {
    w: usize,
    h: usize,
    presents: Vec<usize>, // shape indices, with multiplicity
}

fn parse_input(input: &str) -> (Vec<Vec<Cells>>, Vec<Region>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut shapes: Vec<Cells> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // Region line: contains 'x' and ':'  (e.g. "12x5: 1 0 1 0 2 2")
        if trimmed.contains('x') && trimmed.contains(':') {
            let colon = trimmed.find(':').unwrap();
            let (dims, rest) = trimmed.split_at(colon);
            let mut dims_iter = dims.split('x');
            let w: usize = dims_iter.next().unwrap().trim().parse().unwrap();
            let h: usize = dims_iter.next().unwrap().trim().parse().unwrap();
            let counts: Vec<usize> = rest[1..]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let mut presents = Vec::new();
            for (idx, &n) in counts.iter().enumerate() {
                for _ in 0..n {
                    presents.push(idx);
                }
            }
            regions.push(Region { w, h, presents });
            i += 1;
        } else if trimmed.ends_with(':') {
            // Shape header "N:"
            let idx: usize = trimmed.trim_end_matches(':').parse().unwrap();
            i += 1;
            let mut cells: Cells = Vec::new();
            let mut row = 0i32;
            while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].contains(':') {
                for (c, ch) in lines[i].chars().enumerate() {
                    if ch == '#' {
                        cells.push((row, c as i32));
                    }
                }
                row += 1;
                i += 1;
            }
            while shapes.len() <= idx {
                shapes.push(Vec::new());
            }
            shapes[idx] = cells;
        } else {
            i += 1;
        }
    }

    let shapes_orient: Vec<Vec<Cells>> = shapes.iter().map(|s| all_orientations(s)).collect();
    (shapes_orient, regions)
}

// ---------- Packing ----------

#[inline]
fn fits(grid: &[bool], w: usize, h: usize, cells: &[(i32, i32)], dr: i32, dc: i32) -> bool {
    for &(r, c) in cells {
        let nr = r + dr;
        let nc = c + dc;
        if nr < 0 || nr >= h as i32 || nc < 0 || nc >= w as i32 {
            return false;
        }
        if grid[(nr as usize) * w + (nc as usize)] {
            return false;
        }
    }
    true
}

#[inline]
fn paint(grid: &mut [bool], w: usize, cells: &[(i32, i32)], dr: i32, dc: i32, val: bool) {
    for &(r, c) in cells {
        let nr = (r + dr) as usize;
        let nc = (c + dc) as usize;
        grid[nr * w + nc] = val;
    }
}

fn recurse(
    grid: &mut [bool],
    w: usize,
    h: usize,
    shapes: &[Vec<Cells>],
    presents: &[usize],
    idx: usize,
    prev_anchor: (i32, i32),
) -> bool {
    if idx == presents.len() {
        return true;
    }
    let shape_idx = presents[idx];
    let same_as_prev = idx > 0 && presents[idx - 1] == shape_idx;

    for orient in &shapes[shape_idx] {
        let max_r = orient.iter().map(|&(r, _)| r).max().unwrap();
        let max_c = orient.iter().map(|&(_, c)| c).max().unwrap();
        let dr_max = h as i32 - 1 - max_r;
        let dc_max = w as i32 - 1 - max_c;
        if dr_max < 0 || dc_max < 0 {
            continue;
        }

        for dr in 0..=dr_max {
            for dc in 0..=dc_max {
                // Symmetry break: identical consecutive pieces must be placed
                // in strictly increasing (dr, dc) order to kill permutations.
                if same_as_prev && (dr, dc) <= prev_anchor {
                    continue;
                }
                if fits(grid, w, h, orient, dr, dc) {
                    paint(grid, w, orient, dr, dc, true);
                    if recurse(grid, w, h, shapes, presents, idx + 1, (dr, dc)) {
                        return true;
                    }
                    paint(grid, w, orient, dr, dc, false);
                }
            }
        }
    }
    false
}

fn solve_region(region: &Region, shapes: &[Vec<Cells>]) -> bool {
    // Quick area check
    let total: usize = region
        .presents
        .iter()
        .map(|&i| shapes[i][0].len())
        .sum();
    if total > region.w * region.h {
        return false;
    }
    // Place biggest pieces first (better pruning), keeping identical pieces grouped.
    let mut presents = region.presents.clone();
    presents.sort_by_key(|&i| (std::cmp::Reverse(shapes[i][0].len()), i));

    let mut grid = vec![false; region.w * region.h];
    recurse(&mut grid, region.w, region.h, shapes, &presents, 0, (-1, -1))
}

pub fn p1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    let count = regions.iter()
        .filter(|r| solve_region(r, &shapes))
        .count();

    println!("Valid regions: {}", count);

    count
}

#[cfg(test)]
mod tests {
    use super::p1;

    const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn p1_example() {
        assert_eq!(p1(EXAMPLE), 2);
    }
}
