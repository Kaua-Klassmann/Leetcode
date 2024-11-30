pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
    capacity.sort_by(|a, b| b.cmp(a));

    let mut remaining_apples: i32 = apple.iter().sum();

    for (i, c) in capacity.into_iter().enumerate() {
        remaining_apples -= c;

        if remaining_apples < 1 {
            return i as i32 + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::minimum_boxes;

    #[test]
    fn test_1() {
        assert_eq!(minimum_boxes(vec![1,3,2], vec![4,3,1,5,2]), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(minimum_boxes(vec![5,5,5], vec![2,4,2,7]), 4)
    }
}