pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut contador: i16 = 0;
    let mut candidato: i32 = 0;

    for &num in &nums {
        if contador == 0 {
            candidato = num;
        }

        if num == candidato {
            contador += 1;
        } else {
            contador -= 1;
        }
    }

    candidato
}

#[cfg(test)]
mod tests {
    use crate::majority_element;

    #[test]
    fn test_1() {
        assert_eq!(majority_element(vec![3,2,3]), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(majority_element(vec![2,2,1,1,1,2,2]), 2)
    }
}