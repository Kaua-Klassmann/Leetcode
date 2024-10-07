pub fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    let (mut a, mut b) = (1, 1);

    for _ in 2..n {
        (a, b) = (b, a + b);
    }

    b
}

#[cfg(test)]
mod test {
    use crate::fib;

    #[test]
    fn test_1() {
        assert_eq!(fib(2), 1)
    }

    #[test]
    fn test_2() {
        assert_eq!(fib(3), 2)
    }

    #[test]
    fn test_3() {
        assert_eq!(fib(4), 3)
    }
}
