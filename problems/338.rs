pub fn count_bits(n: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![0; (n+1) as usize];
    
    for num in 0..=n {
        vec[num as usize] = num.count_ones() as i32;
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::count_bits;

    #[test]
    fn test_1() {
        assert_eq!(count_bits(2), vec![0,1,1])
    }

    #[test]
    fn test_2() {
        assert_eq!(count_bits(5), vec![0,1,1,2,1,2])
    }
}
