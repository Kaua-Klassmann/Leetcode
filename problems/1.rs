use std::io::stdin;
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    let mut resp: Vec<i32> = Vec::with_capacity(2 as usize);

    for (index, &num) in nums.iter().enumerate() {
        match map.get(&num) {
            Some(&i) => {
                resp.push(i as i32);
                resp.push(index as i32);
            },
            None => {
                map.insert(target-num, index);
            }
        }
    }

    resp
}

fn main() {
    let mut valores_input: String = String::new();
    stdin().read_line(&mut valores_input).expect("");

    let num_len: i32 = valores_input.trim().parse::<i32>().unwrap();
    let mut nums: Vec<i32> = Vec::with_capacity(num_len as usize);

    for _ in 0..num_len {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("");

        let valor: i32 = input.trim().parse::<i32>().unwrap();
        nums.push(valor);
    }

    let mut target_input: String = String::new();
    stdin().read_line(&mut target_input).expect("");

    let target: i32 = target_input.trim().parse::<i32>().unwrap();

    println!("{:?}", two_sum(nums, target));
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