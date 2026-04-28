
pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day4/input_day4.txt")
        .expect("Cannot read input_day4.txt");
    println!("Day 4 p2 = {}", p2(&input));
  }


pub fn p2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().chars().collect())
        .collect();

    if grid.is_empty() { return 0; }

    let mut removed_total = 0;

    loop {
        // Find all accessible @ cells (fewer than 4 @ neighbors)
        let accessible: Vec<(usize, usize)> = (0..grid.len())
            .flat_map(|r| (0..grid[r].len()).map(move |c| (r, c)))
            .filter(|&(r, c)| {
                if grid[r][c] != '@' { return false; }
                let dirs = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
                let neighbors = dirs.iter().filter(|(dr, dc)| {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    nr >= 0 && nc >= 0
                    && nr < grid.len() as isize
                    && nc < grid[r].len() as isize
                    && grid[nr as usize][nc as usize] == '@'
                }).count();
                neighbors < 4
            })
            .collect();

        // Stop when nothing left to remove
        if accessible.is_empty() { break; }

        removed_total += accessible.len();

        // Remove them
        for (r, c) in accessible {
            grid[r][c] = '.';
        }
    }

    removed_total
}
