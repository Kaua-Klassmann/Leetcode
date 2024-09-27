pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let (mut casa_1, mut casa_2) = (cost[1], cost[0]);

    for index in 2..cost.len() as usize {
        (casa_1, casa_2) = (casa_1.min(casa_2) + cost[index], casa_1)
    }

    casa_1.min(casa_2)
}

#[cfg(test)]
mod test {
    use crate::min_cost_climbing_stairs;

    #[test]
    fn test_1() {
        assert_eq!(min_cost_climbing_stairs(vec![10,15,20]), 15)
    }

    #[test]
    fn test_2() {
        assert_eq!(min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]), 6)
    }
}