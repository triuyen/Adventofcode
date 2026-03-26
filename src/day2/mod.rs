pub fn is_invalid_id_p1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

pub fn is_invalid_id_p2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }

        let repeat_count = len / pattern_len;
        if repeat_count < 2 {
            continue;
        }

        let pattern = &s[..pattern_len];
        let mut valid = true;

        for i in 1..repeat_count {
            let start = i * pattern_len;
            let end = start + pattern_len;

            if &s[start..end] != pattern {
                valid = false;
                break;
            }
        }

        if valid {
            return true;
        }
    }

    false
}

pub fn p1(input: &str) -> u64 {
    let line = input.trim();
    if line.is_empty() {
        return 0;
    }

    let mut sum = 0u64;

    for range in line.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }

        let (start, end) = range
            .split_once('-')
            .unwrap_or_else(|| panic!("Range invalide: {:?}", range));

        let start: u64 = start
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Début invalide dans {:?}: {}", range, e));

        let end: u64 = end
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Fin invalide dans {:?}: {}", range, e));

        for n in start..=end {
            if is_invalid_id_p1(n) {
                sum += n;
            }
        }
    }

    sum
}

pub fn p2(input: &str) -> u64 {
    let line = input.trim();
    if line.is_empty() {
        return 0;
    }

    let mut sum = 0u64;

    for range in line.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }

        let (start, end) = range
            .split_once('-')
            .unwrap_or_else(|| panic!("Range invalide: {:?}", range));

        let start: u64 = start
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Début invalide dans {:?}: {}", range, e));

        let end: u64 = end
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Fin invalide dans {:?}: {}", range, e));

        for n in start..=end {
            if is_invalid_id_p2(n) {
                sum += n;
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::{is_invalid_id_p1, is_invalid_id_p2, p1, p2};

    #[test]
    fn invalid_id_p1_test() {
        assert!(is_invalid_id_p1(11));
        assert!(is_invalid_id_p1(6464));
        assert!(is_invalid_id_p1(123123));

        assert!(!is_invalid_id_p1(101));
        assert!(!is_invalid_id_p1(123321));
        assert!(!is_invalid_id_p1(111));
    }

    #[test]
    fn invalid_id_p2_test() {
        assert!(is_invalid_id_p2(11));
        assert!(is_invalid_id_p2(6464));
        assert!(is_invalid_id_p2(123123));
        assert!(is_invalid_id_p2(123123123));
        assert!(is_invalid_id_p2(1212121212));
        assert!(is_invalid_id_p2(1111111));

        assert!(!is_invalid_id_p2(101));
        assert!(!is_invalid_id_p2(123321));
        assert!(!is_invalid_id_p2(12341235));
    }

    #[test]
    fn p1_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

        assert_eq!(p1(input), 1227775554);
    }

    #[test]
    fn p2_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

        assert_eq!(p2(input), 4174379265);
    }
}