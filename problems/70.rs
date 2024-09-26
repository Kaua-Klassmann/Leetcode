pub fn climb_stairs(n: i32) -> i32 {
    let (mut a, mut b) = (1, 1);

    for _ in 2..=n {
        (a, b) = (b, a + b);
    }

    b
}

#[cfg(test)]
mod test {
    use crate::climb_stairs;

    #[test]
    fn test_1() {
        assert_eq!(climb_stairs(2), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(climb_stairs(3), 3)
    }
}