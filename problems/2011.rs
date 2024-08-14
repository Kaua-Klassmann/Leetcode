pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut resul: i32 = 0;

    for i in &operations{
        resul += match i.as_bytes()[1] {
            b'+' => 1, 
            b'-' => -1,
            _ => 0
        };
    }

    resul
}

#[cfg(test)]
mod test {
    use crate::final_value_after_operations;

    #[test]
    fn test_1() {
        assert_eq!(final_value_after_operations(vec![
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string()
        ]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(final_value_after_operations(vec![
            "++X".to_string(),
            "++X".to_string(),
            "X++".to_string()
        ]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(final_value_after_operations(vec![
            "X++".to_string(),
            "++X".to_string(),
            "--X".to_string(),
            "X--".to_string()
        ]), 0);
    }
}
