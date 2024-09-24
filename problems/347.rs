use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    nums.into_iter().for_each(|num| {
        *counter.entry(num).or_insert(0) -= 1;
    });

    let mut heapq: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(k as usize + 1);

    counter.into_iter().for_each(|nums| {
        heapq.push((nums.1, nums.0));

        if heapq.len() > k as usize {
            heapq.pop();
        }
    });

    heapq.into_iter().map(|num| num.1).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use crate::top_k_frequent;

    #[test]
    fn test_1() {
        assert_eq!(top_k_frequent(vec![1,1,1,2,2,3], 2), vec![2,1])
    }

    #[test]
    fn test_2() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1])
    }
}
