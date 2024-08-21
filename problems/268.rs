use std::collections::HashSet;

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let tamanho: usize = nums.len();

    let mut set: HashSet<i32> = HashSet::with_capacity(tamanho);

    for &num in &nums {
        set.insert(num);
    }

    for num in 0..tamanho as i32 {
        if !set.contains(&num) {
            return num;
        }
    }

    tamanho as i32
}

#[cfg(test)]
mod tests {
    use crate::missing_number;

    #[test]
    fn test_1() {
        assert_eq!(missing_number(vec![3,0,1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(missing_number(vec![0,1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);
    }
}