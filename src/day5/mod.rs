fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut reading_ranges = true;

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let (start, end) = line
                .split_once('-')
                .unwrap_or_else(|| panic!("Range invalide: {:?}", line));

            let start = start
                .trim()
                .parse::<u64>()
                .unwrap_or_else(|e| panic!("Début invalide dans {:?}: {}", line, e));

            let end = end
                .trim()
                .parse::<u64>()
                .unwrap_or_else(|e| panic!("Fin invalide dans {:?}: {}", line, e));

            ranges.push((start, end));
        } else {
            let id = line
                .parse::<u64>()
                .unwrap_or_else(|e| panic!("ID invalide {:?}: {}", line, e));

            ids.push(id);
        }
    }

    (ranges, ids)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| start <= id && id <= end)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

pub fn p1(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);

    ids.into_iter()
        .filter(|&id| is_fresh(id, &ranges))
        .count()
}

pub fn p2(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);

    merged
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(p1(input), 3);
    }

    #[test]
    fn p2_test() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(p2(input), 14);
    }
}