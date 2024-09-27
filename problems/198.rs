pub fn rob(nums: Vec<i32>) -> i32 {
    let (mut casa_1, mut casa_2) = (0, 0);

    for num in nums {
        (casa_1, casa_2) = (casa_1.max(casa_2 + num), casa_1);
    }

    casa_1
}

#[cfg(test)]
mod test {
    use crate::rob;

    #[test]
    fn test_1() {
        assert_eq!(rob(vec![1,2,3,1]), 4)
    }

    #[test]
    fn test_2() {
        assert_eq!(rob(vec![2,7,9,3,1]), 12)
    }
}