pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed: Vec<i32> = flowerbed.clone();
    let mut remaining: i32 = n;
    let length: usize = flowerbed.len();

    if length == 1 {
        return remaining == 0 || (flowerbed[0] == 0 && remaining <= 1);
    }

    if flowerbed[0] == 0 && flowerbed[1] == 0 {
        remaining -= 1;
        flowerbed[0] = 1;
    }

    if flowerbed[length-2] == 0 && flowerbed[length-1] == 0 {
        remaining -= 1;
        flowerbed[length-1] = 1;
    }

    for i in 1..length-1 {
        if flowerbed[i] == 0 && flowerbed[i-1] == 0 && flowerbed[i+1] == 0 {
            flowerbed[i] = 1;
            remaining -= 1;

            if remaining == 0 {
                return true;
            }
        }
    }

    remaining <= 0
}

#[cfg(test)]
mod tests {
    use crate::can_place_flowers;

    #[test]
    fn test_1() {
        assert_eq!(can_place_flowers(vec![1,0,0,0,1], 1), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(can_place_flowers(vec![1,0,0,0,1], 2), false)
    }
}