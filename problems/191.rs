pub fn hamming_weight(mut n: i32) -> i32 {
    let mut num: i32 = 0;

    while n != 0 {
        num += n & 1;
        n >>= 1;
    }

    num

    // easy solution: n.count_ones() as i32
}

#[cfg(test)]
mod tests {
    use crate::hamming_weight;

    #[test]
    fn test_1() {
        assert_eq!(hamming_weight(11), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(hamming_weight(128), 1)
    }

    #[test]
    fn test_3() {
        assert_eq!(hamming_weight(2147483645), 30)
    }
}