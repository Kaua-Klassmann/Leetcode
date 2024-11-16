pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_reach: usize = 0;

    for (index, &jumps) in nums.iter().enumerate() {
        if index > max_reach {
            return false;
        }

        max_reach = max_reach.max(index + jumps as usize);

        if max_reach >= nums.len() - 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::can_jump;

    #[test]
    fn test_1() {
        assert_eq!(can_jump(vec![2,3,1,1,4]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(can_jump(vec![3,2,1,0,4]), false);
    }
}