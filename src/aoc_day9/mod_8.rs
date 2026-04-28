pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day8/input_day8.txt")
        .expect("Cannot read input_day8.txt");
    println!("Day 8 p1 = {}", p1(&input));
    println!("Day 8 p2 = {}", p2(&input));
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut p = l.trim().split(',');
            let x = p.next().unwrap().trim().parse().unwrap();
            let y = p.next().unwrap().trim().parse().unwrap();
            let z = p.next().unwrap().trim().parse().unwrap();
            Point { x, y, z }
        })
        .collect()
}

fn dist(a: Point, b: Point) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz // squared distance, avoids sqrt
}

/// Build all pairs (sq_distance, i, j) sorted by distance ascending.
fn sorted_pairs(points: &[Point]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((dist(points[i], points[j]), i, j));
        }
    }
    pairs.sort_unstable();
    pairs
}

// DSU (Disjoint Set Union / Union-Find) with a live component count
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
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Returns true iff this call actually merged two distinct components.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            self.parent[a] = b;
            self.size[b] += self.size[a];
        } else {
            self.parent[b] = a;
            self.size[a] += self.size[b];
        }
        self.components -= 1;
        true
    }
}

pub fn p1(input: &str) -> usize {
    let points = parse(input);
    let n = points.len();
    let pairs = sorted_pairs(&points);

    // Union the 1000 closest pairs
    let mut dsu = Dsu::new(n);
    for &(_, i, j) in pairs.iter().take(1000) {
        dsu.union(i, j);
    }

    // Path-compress every node once so parent[i] points directly at its root.
    // After this loop, no &mut borrow on `dsu` is needed for the reads below.
    for i in 0..n {
        dsu.find(i);
    }

    // Pure reads now -> no borrow-checker conflict.
    let mut sizes: Vec<usize> = (0..n)
        .filter(|&i| dsu.parent[i] == i)
        .map(|i| dsu.size[i])
        .collect();

    // Top 3 component sizes, multiplied
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

pub fn p2(input: &str) -> i64 {
    let points = parse(input);
    let n = points.len();
    let pairs = sorted_pairs(&points);

    let mut dsu = Dsu::new(n);

    // Walk pairs in increasing distance, performing Kruskal-style merges.
    // The pair whose merge brings `components` down to 1 is the answer.
    for &(_, i, j) in &pairs {
        if dsu.union(i, j) && dsu.components == 1 {
            return points[i].x * points[j].x;
        }
    }

    panic!("Could not connect all junction boxes into a single circuit");
}

#[cfg(test)]
mod tests {
    use super::p2;

    const EXAMPLE: &str = "\
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

    #[test]
    fn p2_example() {
        assert_eq!(p2(EXAMPLE), 25272);
    }
}
