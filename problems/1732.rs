pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    gain.into_iter().fold(
        (0, 0), |(acc, maior), num| (acc + num, maior.max(acc + num))
    ).1
}

#[cfg(test)]
mod tests {
    use crate::largest_altitude;

    #[test]
    fn test_1() {
        assert_eq!(largest_altitude(vec![-5,1,5,0,-7]), 1)
    }

    #[test]
    fn test_2() {
        assert_eq!(largest_altitude(vec![-4,-3,-2,-1,4,3,2]), 0)
    }
}
