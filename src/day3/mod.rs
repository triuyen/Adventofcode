fn max_subsequence(line: &str, keep: usize) -> String {
    let digits: Vec<char> = line.trim().chars().collect();

    if digits.len() <= keep {
        return digits.into_iter().collect();
    }

    let mut to_remove = digits.len() - keep;
    let mut stack: Vec<char> = Vec::with_capacity(digits.len());

    for &c in &digits {
        while to_remove > 0
            && !stack.is_empty()
            && *stack.last().unwrap() < c
        {
            stack.pop();
            to_remove -= 1;
        }

        stack.push(c);
    }

    stack.truncate(keep);
    stack.into_iter().collect()
}

fn parse_big_number(s: &str) -> u128 {
    s.parse::<u128>()
        .unwrap_or_else(|e| panic!("Nombre invalide {:?}: {}", s, e))
}

pub fn p1(input: &str) -> u128 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let best = max_subsequence(line, 2);
            parse_big_number(&best)
        })
        .sum()
}

pub fn p2(input: &str) -> u128 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let best = max_subsequence(line, 12);
            parse_big_number(&best)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{max_subsequence, p1, p2};

    #[test]
    fn max_subsequence_test() {
        assert_eq!(max_subsequence("987654321111111", 2), "98");
        assert_eq!(max_subsequence("811111111111119", 2), "89");
        assert_eq!(max_subsequence("234234234234278", 2), "78");
        assert_eq!(max_subsequence("818181911112111", 2), "92");

        assert_eq!(max_subsequence("987654321111111", 12), "987654321111");
        assert_eq!(max_subsequence("811111111111119", 12), "811111111119");
        assert_eq!(max_subsequence("234234234234278", 12), "434234234278");
        assert_eq!(max_subsequence("818181911112111", 12), "888911112111");
    }

    #[test]
    fn p1_test() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(p1(input), 357);
    }

    #[test]
    fn p2_test() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(p2(input), 3121910778619);
    }
}