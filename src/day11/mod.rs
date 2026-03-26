use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let line = line.trim();

        let (node, rest) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("Ligne invalide: {:?}", line));

        let node = node.trim().to_string();
        let neighbors = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        graph.insert(node, neighbors);
    }

    graph
}

fn count_paths_p1_memo(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    let mut total = 0u64;

    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total += count_paths_p1_memo(graph, next, target, memo);
        }
    }

    memo.insert(current.to_string(), total);
    total
}

fn count_paths_p2_memo(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let seen_dac = seen_dac || current == "dac";
    let seen_fft = seen_fft || current == "fft";

    if current == target {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    let key = (current.to_string(), seen_dac, seen_fft);

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut total = 0u64;

    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total += count_paths_p2_memo(graph, next, target, seen_dac, seen_fft, memo);
        }
    }

    memo.insert(key, total);
    total
}

pub fn p1(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths_p1_memo(&graph, "you", "out", &mut memo)
}

pub fn p2(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths_p2_memo(&graph, "svr", "out", false, false, &mut memo)
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        assert_eq!(p1(input), 5);
    }

    #[test]
    fn p2_test() {
        let input = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        assert_eq!(p2(input), 2);
    }
}