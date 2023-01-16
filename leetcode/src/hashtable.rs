use std::collections::HashMap;

// Solve leetcode [0001] Two Sum
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();

    for (idx, val) in nums.iter().enumerate() {
        let diff = target - val;
        match hm.get(&diff) {
            Some(i) => return vec![*i, idx as i32],
            None => {
                hm.insert(val, idx as i32);
                continue;
            }
        }
    }

    // violate the problem's convention
    return vec![-1, -1];
}
