pub fn p1()-> usize{
    todo!();
}

pub fn p2()-> usize{
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input: &str = include_str!("d1_test.txt");
        assert_eq!(p1(), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(), 0);
    }
}