pub fn tribonacci(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    if n == 2 {
        return 1;
    }

    let (mut a, mut b, mut c) = (0, 1, 1);

    for _ in 3..=n {
        (a, b, c) = (b, c, a+b+c);
    }

    c
}

#[cfg(test)]
mod test {
    use crate::tribonacci;

    #[test]
    fn test_1() {
        assert_eq!(tribonacci(4), 4)
    }

    #[test]
    fn test_2() {
        assert_eq!(tribonacci(25), 1389537)
    }
}