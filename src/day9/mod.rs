use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            let (x, y) = line
                .split_once(',')
                .unwrap_or_else(|| panic!("Coordonnée invalide: {:?}", line));

            let x = x
                .trim()
                .parse::<i64>()
                .unwrap_or_else(|e| panic!("X invalide dans {:?}: {}", line, e));

            let y = y
                .trim()
                .parse::<i64>()
                .unwrap_or_else(|e| panic!("Y invalide dans {:?}: {}", line, e));

            Point { x, y }
        })
        .collect()
}

fn rect_area(a: Point, b: Point) -> i64 {
    let width = (a.x - b.x).abs() + 1;
    let height = (a.y - b.y).abs() + 1;
    width * height
}

pub fn p1(input: &str) -> i64 {
    let points = parse_input(input);

    if points.len() < 2 {
        return 0;
    }

    let mut best = 0i64;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            best = best.max(rect_area(points[i], points[j]));
        }
    }

    best
}

fn build_segments(values: &[i64]) -> (Vec<Segment>, HashMap<i64, usize>) {
    let mut uniq = values.to_vec();
    uniq.sort_unstable();
    uniq.dedup();

    let mut segments = Vec::new();
    let mut exact_index = HashMap::new();

    for i in 0..uniq.len() {
        let v = uniq[i];

        let idx = segments.len();
        segments.push(Segment { start: v, end: v });
        exact_index.insert(v, idx);

        if i + 1 < uniq.len() {
            let next = uniq[i + 1];
            if next > v + 1 {
                segments.push(Segment {
                    start: v + 1,
                    end: next - 1,
                });
            }
        }
    }

    (segments, exact_index)
}

fn mark_boundary(
    points: &[Point],
    xs: &[Segment],
    ys: &[Segment],
    x_index: &HashMap<i64, usize>,
    y_index: &HashMap<i64, usize>,
) -> Vec<Vec<bool>> {
    let h = ys.len();
    let w = xs.len();
    let mut boundary = vec![vec![false; w]; h];

    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];

        if a.x == b.x {
            let x_idx = *x_index
                .get(&a.x)
                .unwrap_or_else(|| panic!("X introuvable dans l'index: {}", a.x));

            let y_min = a.y.min(b.y);
            let y_max = a.y.max(b.y);

            for (yi, seg) in ys.iter().enumerate() {
                if seg.start >= y_min && seg.end <= y_max {
                    boundary[yi][x_idx] = true;
                }
            }
        } else if a.y == b.y {
            let y_idx = *y_index
                .get(&a.y)
                .unwrap_or_else(|| panic!("Y introuvable dans l'index: {}", a.y));

            let x_min = a.x.min(b.x);
            let x_max = a.x.max(b.x);

            for (xi, seg) in xs.iter().enumerate() {
                if seg.start >= x_min && seg.end <= x_max {
                    boundary[y_idx][xi] = true;
                }
            }
        } else {
            panic!("Segment non orthogonal entre {:?} et {:?}", a, b);
        }
    }

    boundary
}

fn flood_outside(boundary: &[Vec<bool>], start_y: usize, start_x: usize) -> Vec<Vec<bool>> {
    let h = boundary.len();
    let w = boundary[0].len();

    let mut outside = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();

    if !boundary[start_y][start_x] {
        outside[start_y][start_x] = true;
        queue.push_back((start_y, start_x));
    }

    let dirs = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in dirs {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny < 0 || nx < 0 || ny >= h as isize || nx >= w as isize {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if boundary[ny][nx] || outside[ny][nx] {
                continue;
            }

            outside[ny][nx] = true;
            queue.push_back((ny, nx));
        }
    }

    outside
}

fn build_prefix_bad(filled: &[Vec<bool>]) -> Vec<Vec<i64>> {
    let h = filled.len();
    let w = filled[0].len();

    let mut pref = vec![vec![0i64; w + 1]; h + 1];

    for y in 0..h {
        for x in 0..w {
            let bad = if filled[y][x] { 0i64 } else { 1i64 };
            pref[y + 1][x + 1] = pref[y][x + 1] + pref[y + 1][x] - pref[y][x] + bad;
        }
    }

    pref
}

fn query_sum(pref: &[Vec<i64>], y1: usize, x1: usize, y2: usize, x2: usize) -> i64 {
    pref[y2 + 1][x2 + 1] - pref[y1][x2 + 1] - pref[y2 + 1][x1] + pref[y1][x1]
}

pub fn p2(input: &str) -> i64 {
    let points = parse_input(input);

    if points.len() < 2 {
        return 0;
    }

    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut xs_raw: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys_raw: Vec<i64> = points.iter().map(|p| p.y).collect();

    xs_raw.push(min_x - 1);
    xs_raw.push(max_x + 1);
    ys_raw.push(min_y - 1);
    ys_raw.push(max_y + 1);

    let (xs, x_index) = build_segments(&xs_raw);
    let (ys, y_index) = build_segments(&ys_raw);

    let boundary = mark_boundary(&points, &xs, &ys, &x_index, &y_index);

    let outside_start_x = *x_index
        .get(&(min_x - 1))
        .unwrap_or_else(|| panic!("Index X extérieur introuvable"));
    let outside_start_y = *y_index
        .get(&(min_y - 1))
        .unwrap_or_else(|| panic!("Index Y extérieur introuvable"));

    let outside = flood_outside(&boundary, outside_start_y, outside_start_x);

    let h = ys.len();
    let w = xs.len();
    let mut filled = vec![vec![false; w]; h];

    for y in 0..h {
        for x in 0..w {
            filled[y][x] = boundary[y][x] || !outside[y][x];
        }
    }

    let pref_bad = build_prefix_bad(&filled);

    let red_tiles: HashSet<Point> = points.iter().copied().collect();
    let red_list: Vec<Point> = red_tiles.iter().copied().collect();

    let mut best = 0i64;

    for i in 0..red_list.len() {
        for j in (i + 1)..red_list.len() {
            let a = red_list[i];
            let b = red_list[j];

            let area = rect_area(a, b);
            if area <= best {
                continue;
            }

            let x1 = a.x.min(b.x);
            let x2 = a.x.max(b.x);
            let y1 = a.y.min(b.y);
            let y2 = a.y.max(b.y);

            let cx1 = *x_index
                .get(&x1)
                .unwrap_or_else(|| panic!("X rectangle introuvable: {}", x1));
            let cx2 = *x_index
                .get(&x2)
                .unwrap_or_else(|| panic!("X rectangle introuvable: {}", x2));
            let cy1 = *y_index
                .get(&y1)
                .unwrap_or_else(|| panic!("Y rectangle introuvable: {}", y1));
            let cy2 = *y_index
                .get(&y2)
                .unwrap_or_else(|| panic!("Y rectangle introuvable: {}", y2));

            if query_sum(&pref_bad, cy1, cx1, cy2, cx2) == 0 {
                best = area;
            }
        }
    }

    best
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        assert_eq!(p1(input), 50);
    }

    #[test]
    fn p2_test() {
        let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        assert_eq!(p2(input), 24);
    }
}