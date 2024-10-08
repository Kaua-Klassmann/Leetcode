use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    let mut resp: Vec<i32> = Vec::with_capacity(2 as usize);

    for (index, &num) in nums.iter().enumerate() {
        match map.get(&num) {
            Some(&i) => {
                resp.push(i as i32);
                resp.push(index as i32);
                break;
            },
            None => {
                map.insert(target-num, index);
            }
        }
    }

    resp
}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1])
    }

    #[test]
    fn test_2() {
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2])
    }

    #[test]
    fn test_3() {
        assert_eq!(two_sum(vec![3,3], 6), vec![0,1])
    }
}
