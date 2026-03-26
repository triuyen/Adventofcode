use std::cmp::Reverse;
use std::collections::HashSet;

type Cell = (i32, i32);

#[derive(Clone, Debug)]
struct Shape {
    area: usize,
    variants: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug)]
struct RegionRequest {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Placement {
    bits: Vec<u64>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct StateKey {
    occupied: Vec<u64>,
    counts: Vec<usize>,
    starts: Vec<usize>,
}

fn normalize(cells: &[Cell]) -> Vec<Cell> {
    let min_r = cells.iter().map(|&(r, _)| r).min().unwrap_or(0);
    let min_c = cells.iter().map(|&(_, c)| c).min().unwrap_or(0);

    let mut out: Vec<Cell> = cells.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();
    out.sort_unstable();
    out
}

fn rotate90(cells: &[Cell]) -> Vec<Cell> {
    let rotated: Vec<Cell> = cells.iter().map(|&(r, c)| (c, -r)).collect();
    normalize(&rotated)
}

fn flip_horizontal(cells: &[Cell]) -> Vec<Cell> {
    let flipped: Vec<Cell> = cells.iter().map(|&(r, c)| (r, -c)).collect();
    normalize(&flipped)
}

fn all_variants(cells: &[Cell]) -> Vec<Vec<Cell>> {
    let base = normalize(cells);
    let mut seen = HashSet::<Vec<Cell>>::new();
    let mut variants = Vec::new();

    let mut current = base.clone();
    for _ in 0..4 {
        let v1 = normalize(&current);
        if seen.insert(v1.clone()) {
            variants.push(v1);
        }

        let v2 = flip_horizontal(&current);
        if seen.insert(v2.clone()) {
            variants.push(v2);
        }

        current = rotate90(&current);
    }

    variants
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<RegionRequest>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0usize;

    let mut raw_shapes: Vec<(usize, Vec<String>)> = Vec::new();

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.contains('x') && line.contains(':') {
            break;
        }

        if let Some(prefix) = line.strip_suffix(':') {
            let idx: usize = prefix
                .parse()
                .unwrap_or_else(|e| panic!("Index de forme invalide {:?}: {}", line, e));

            i += 1;
            let mut rows = Vec::new();

            while i < lines.len() {
                let row = lines[i].trim_end();

                if row.trim().is_empty() {
                    break;
                }
                if row.trim().ends_with(':') || (row.contains('x') && row.contains(':')) {
                    break;
                }

                rows.push(row.trim().to_string());
                i += 1;
            }

            raw_shapes.push((idx, rows));
        } else {
            panic!("Ligne inattendue dans la section formes: {:?}", line);
        }
    }

    raw_shapes.sort_unstable_by_key(|(idx, _)| *idx);

    let mut shapes = Vec::new();

    for (expected_idx, (idx, rows)) in raw_shapes.into_iter().enumerate() {
        assert_eq!(expected_idx, idx, "Les indices de formes doivent être continus");

        let mut cells = Vec::new();
        for (r, row) in rows.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                if ch == '#' {
                    cells.push((r as i32, c as i32));
                }
            }
        }

        let variants = all_variants(&cells);
        let area = variants[0].len();

        shapes.push(Shape { area, variants });
    }

    let mut regions = Vec::new();

    while i < lines.len() {
        let line = lines[i].trim();
        i += 1;

        if line.is_empty() {
            continue;
        }

        let (dims, counts_str) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("Ligne région invalide: {:?}", line));

        let (w, h) = dims
            .trim()
            .split_once('x')
            .unwrap_or_else(|| panic!("Dimensions invalides: {:?}", dims));

        let width: usize = w
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Largeur invalide {:?}: {}", w, e));
        let height: usize = h
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Hauteur invalide {:?}: {}", h, e));

        let counts: Vec<usize> = counts_str
            .split_whitespace()
            .map(|x| {
                x.parse::<usize>()
                    .unwrap_or_else(|e| panic!("Quantité invalide {:?}: {}", x, e))
            })
            .collect();

        regions.push(RegionRequest {
            width,
            height,
            counts,
        });
    }

    (shapes, regions)
}

fn bit_chunks(cell_count: usize) -> usize {
    cell_count.div_ceil(64)
}

fn bit_index(width: usize, r: usize, c: usize) -> usize {
    r * width + c
}

fn set_bit(bits: &mut [u64], idx: usize) {
    bits[idx / 64] |= 1u64 << (idx % 64);
}

fn overlaps(a: &[u64], b: &[u64]) -> bool {
    a.iter().zip(b).any(|(x, y)| (x & y) != 0)
}

fn apply_or_inplace(dst: &mut [u64], src: &[u64]) {
    for (d, s) in dst.iter_mut().zip(src) {
        *d |= *s;
    }
}

fn apply_xor_inplace(dst: &mut [u64], src: &[u64]) {
    for (d, s) in dst.iter_mut().zip(src) {
        *d ^= *s;
    }
}

fn occupied_count(bits: &[u64]) -> usize {
    bits.iter().map(|x| x.count_ones() as usize).sum()
}

fn build_placements_for_shape(shape: &Shape, width: usize, height: usize) -> Vec<Placement> {
    let cell_count = width * height;
    let chunks = bit_chunks(cell_count);
    let mut unique = HashSet::<Vec<u64>>::new();
    let mut placements = Vec::new();

    for variant in &shape.variants {
        let max_r = variant.iter().map(|&(r, _)| r).max().unwrap() as usize;
        let max_c = variant.iter().map(|&(_, c)| c).max().unwrap() as usize;

        if max_r >= height || max_c >= width {
            continue;
        }

        for base_r in 0..=(height - max_r - 1) {
            for base_c in 0..=(width - max_c - 1) {
                let mut bits = vec![0u64; chunks];

                for &(dr, dc) in variant {
                    let r = base_r + dr as usize;
                    let c = base_c + dc as usize;
                    let idx = bit_index(width, r, c);
                    set_bit(&mut bits, idx);
                }

                if unique.insert(bits.clone()) {
                    placements.push(Placement { bits });
                }
            }
        }
    }

    placements
}

fn region_can_fit(shapes: &[Shape], region: &RegionRequest) -> bool {
    if region.counts.len() > shapes.len() {
        return false;
    }

    let total_area_needed: usize = region
        .counts
        .iter()
        .enumerate()
        .map(|(shape_idx, &count)| count * shapes[shape_idx].area)
        .sum();

    let board_area = region.width * region.height;
    if total_area_needed > board_area {
        return false;
    }

    let placements_by_shape: Vec<Vec<Placement>> = shapes
        .iter()
        .map(|shape| build_placements_for_shape(shape, region.width, region.height))
        .collect();

    for (shape_idx, &count) in region.counts.iter().enumerate() {
        if count > 0 && placements_by_shape[shape_idx].is_empty() {
            return false;
        }
    }

    let mut counts = region.counts.clone();
    counts.resize(shapes.len(), 0);

    let mut start_index = vec![0usize; shapes.len()];
    let mut occupied = vec![0u64; bit_chunks(board_area)];
    let mut dead_states = HashSet::<StateKey>::new();

    fn search(
        shapes: &[Shape],
        placements_by_shape: &[Vec<Placement>],
        counts: &mut [usize],
        start_index: &mut [usize],
        occupied: &mut [u64],
        remaining_area: usize,
        board_area: usize,
        dead_states: &mut HashSet<StateKey>,
    ) -> bool {
        if counts.iter().all(|&c| c == 0) {
            return true;
        }

        let key = StateKey {
            occupied: occupied.to_vec(),
            counts: counts.to_vec(),
            starts: start_index.to_vec(),
        };

        if dead_states.contains(&key) {
            return false;
        }

        let free_area = board_area - occupied_count(occupied);
        if remaining_area > free_area {
            dead_states.insert(key);
            return false;
        }

        let mut chosen_shape = None;
        let mut chosen_valid_count = usize::MAX;
        let mut chosen_area = 0usize;

        for shape_idx in 0..counts.len() {
            if counts[shape_idx] == 0 {
                continue;
            }

            let placements = &placements_by_shape[shape_idx];
            let from = start_index[shape_idx];

            let mut valid_count = 0usize;
            for placement in placements.iter().skip(from) {
                if !overlaps(occupied, &placement.bits) {
                    valid_count += 1;
                }
            }

            if valid_count == 0 {
                dead_states.insert(key);
                return false;
            }

            let area = shapes[shape_idx].area;

            if valid_count < chosen_valid_count
                || (valid_count == chosen_valid_count && area > chosen_area)
            {
                chosen_shape = Some(shape_idx);
                chosen_valid_count = valid_count;
                chosen_area = area;
            }
        }

        let shape_idx = chosen_shape.unwrap();
        let placements = &placements_by_shape[shape_idx];
        let start = start_index[shape_idx];
        let saved_start = start;

        for pi in start..placements.len() {
            let placement = &placements[pi];

            if overlaps(occupied, &placement.bits) {
                continue;
            }

            apply_or_inplace(occupied, &placement.bits);
            counts[shape_idx] -= 1;

            let old_start = start_index[shape_idx];
            start_index[shape_idx] = pi;

            if search(
                shapes,
                placements_by_shape,
                counts,
                start_index,
                occupied,
                remaining_area - shapes[shape_idx].area,
                board_area,
                dead_states,
            ) {
                return true;
            }

            start_index[shape_idx] = old_start;
            counts[shape_idx] += 1;
            apply_xor_inplace(occupied, &placement.bits);
        }

        start_index[shape_idx] = saved_start;
        dead_states.insert(key);
        false
    }

    search(
        shapes,
        &placements_by_shape,
        &mut counts,
        &mut start_index,
        &mut occupied,
        total_area_needed,
        board_area,
        &mut dead_states,
    )
}

pub fn p1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);

    regions
        .iter()
        .filter(|region| region_can_fit(&shapes, region))
        .count()
}

#[cfg(test)]
mod test {
    use super::p1;

    #[test]
    fn p1_test() {
        let input = r#"0:
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
12x5: 1 0 1 0 3 2"#;

        assert_eq!(p1(input), 2);
    }
}