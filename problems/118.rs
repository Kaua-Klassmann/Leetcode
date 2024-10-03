pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut resp: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

    for index_row in 0..num_rows as usize {
        let mut row: Vec<i32> = vec![1; index_row + 1];

        for index in 1..index_row {
            row[index] = resp[index_row - 1][index - 1] + resp[index_row - 1][index];
        }

        resp.push(row);
    }
    
    resp
}

#[cfg(test)]
mod test {
    use crate::generate;

    #[test]
    fn test_1() {
        assert_eq!(generate(5), vec![vec![1], vec![1,1], vec![1,2,1], vec![1,3,3,1], vec![1,4,6,4,1]])
    }

    #[test]
    fn test_2() {
        assert_eq!(generate(1), vec![vec![1]])
    }
}
