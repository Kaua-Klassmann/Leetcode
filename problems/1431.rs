pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    let mut resp: Vec<bool> = Vec::with_capacity(candies.len());

    for &number in &candies {
        resp.push(!(number + extra_candies < max));
    }

    resp
}

#[cfg(test)]
mod test {
    use crate::kids_with_candies;

    #[test]
    fn test_1() {
        assert_eq!(kids_with_candies(vec![4,2,1,1,2], 1), vec![true,false,false,false,false])
    }

    #[test]
    fn test_2() {
        assert_eq!(kids_with_candies(vec![12,1,12], 10), vec![true,false,true])
    }
}
