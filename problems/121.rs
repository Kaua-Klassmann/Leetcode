pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut compra: i32 = prices[0];
    let mut lucro: i32 = 0;

    for i in 1..prices.len() {
        if prices[i] < compra {
            compra = prices[i];
        } else {
            let lucro_temp: i32 = prices[i] - compra;
            if lucro_temp > lucro {
                lucro = lucro_temp;
            }
        }
    }

    lucro
}

#[cfg(test)]
mod test {
    use crate::max_profit;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5)
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(vec![7,6,4,3,1]), 0)
    }
}