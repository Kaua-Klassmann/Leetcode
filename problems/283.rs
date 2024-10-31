pub fn move_zeroes(nums: &mut Vec<i32>) {
    let length = nums.len();

    nums.retain(|num| *num != 0);

    for _ in nums.len()..length {
        nums.push(0);
    }
}