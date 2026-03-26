use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            let mut parts = line.split(',');

            let x = parts
                .next()
                .unwrap_or_else(|| panic!("X manquant dans {:?}", line))
                .trim()
                .parse::<i64>()
                .unwrap_or_else(|e| panic!("X invalide dans {:?}: {}", line, e));

            let y = parts
                .next()
                .unwrap_or_else(|| panic!("Y manquant dans {:?}", line))
                .trim()
                .parse::<i64>()
                .unwrap_or_else(|e| panic!("Y invalide dans {:?}: {}", line, e));

            let z = parts
                .next()
                .unwrap_or_else(|| panic!("Z manquant dans {:?}", line))
                .trim()
                .parse::<i64>()
                .unwrap_or_else(|e| panic!("Z invalide dans {:?}: {}", line, e));

            Point { x, y, z }
        })
        .collect()
}

fn squared_distance(a: Point, b: Point) -> i128 {
    let dx = (a.x - b.x) as i128;
    let dy = (a.y - b.y) as i128;
    let dz = (a.z - b.z) as i128;
    dx * dx + dy * dy + dz * dz
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut counts = HashMap::new();

        for i in 0..n {
            let root = self.find(i);
            *counts.entry(root).or_insert(0usize) += 1;
        }

        counts.into_values().collect()
    }
}

fn all_pairs(points: &[Point]) -> Vec<(i128, usize, usize)> {
    let n = points.len();
    let mut pairs = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((squared_distance(points[i], points[j]), i, j));
        }
    }

    pairs.sort_unstable_by_key(|&(dist, i, j)| (dist, i, j));
    pairs
}

fn solve_with_limit(input: &str, limit: usize) -> usize {
    let points = parse_input(input);
    let n = points.len();

    if n < 3 {
        return 0;
    }

    let pairs = all_pairs(&points);
    let mut dsu = Dsu::new(n);

    for &(_, a, b) in pairs.iter().take(limit) {
        dsu.union(a, b);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

pub fn p1(input: &str) -> usize {
    solve_with_limit(input, 1000)
}

pub fn p2(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    if n <= 1 {
        return 0;
    }

    let pairs = all_pairs(&points);
    let mut dsu = Dsu::new(n);

    for &(_, a, b) in &pairs {
        if dsu.union(a, b) && dsu.components == 1 {
            return points[a].x * points[b].x;
        }
    }

    panic!("Impossible de connecter tous les points");
}

#[cfg(test)]
mod test {
    use super::{p1, p2, solve_with_limit};

    #[test]
    fn p1_example_test() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(solve_with_limit(input, 10), 40);
    }

    #[test]
    fn p2_example_test() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(p2(input), 25272);
    }

    // Seulement là si tu veux garder un test direct sur p1 réel
    #[test]
    fn p1_small_guard() {
        let input = "\
0,0,0
1,0,0
2,0,0";

        assert_eq!(p1(input), 0);
    }
}