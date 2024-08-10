use std::io::stdin;

fn count_elements(nums: Vec<i32>) -> i32 {
    let _max: i32 = *nums.iter().max().unwrap();
    let _min: i32 = *nums.iter().min().unwrap();

    nums.into_iter().filter(|&x| x != _max && x != _min).count() as i32
}

fn main() {
    let mut valores_input: String = String::new();
    stdin().read_line(&mut valores_input).expect("");

    let valores: i32 = valores_input.trim().parse::<i32>().unwrap();

    let mut nums: Vec<i32> = Vec::with_capacity(valores as usize);

    for _ in 0..valores {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("");

        nums.push(input.trim().parse::<i32>().unwrap());
    }

    println!("{:?}", count_elements(nums));
}

#[cfg(test)]
mod tests {
    use crate::count_elements;

    #[test]
    fn test_1() {
        assert_eq!(count_elements(vec![11,7,2,15]), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(count_elements(vec![-3,3,3,90]), 2)
    }
}