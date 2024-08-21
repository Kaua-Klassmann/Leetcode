pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut index: i32 = m;

    for &mut num in nums2 {
        nums1[index as usize] = num;
        index += 1;
    }

    nums1.sort();
}