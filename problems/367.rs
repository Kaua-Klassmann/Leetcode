pub fn is_perfect_square(num: i32) -> bool {
    let (mut l, mut r) = (0, 46340);

    while l <= r {
        let mid: i32 = (l + r) / 2;

        let resp: i32 = mid * mid;

        if resp == num {
            return true;
        }

        if resp < num {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::is_perfect_square;

    #[test]
    fn test_1() {
        assert_eq!(is_perfect_square(16), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_perfect_square(14), false)
    }
}