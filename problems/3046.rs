pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut arr: [i8; 100] = [0; 100];

    for num in nums{
        if arr[num as usize - 1] > 1 {
            return false;
        }

        arr[num as usize - 1] += 1;
    }

    true
}

#[cfg(test)]
mod test {
    use crate::is_possible_to_split;

    #[test]
    fn test_1() {
        assert_eq!(is_possible_to_split(vec![1,1,2,2,3,4]), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_possible_to_split(vec![1,1,1,1]), false)
    }
}