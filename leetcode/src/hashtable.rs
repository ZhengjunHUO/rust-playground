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

#[test]
fn test_two_sum() {
    let mut nums = [
        Some(vec![2, 7, 11, 15]),
        Some(vec![3, 2, 4]),
        Some(vec![3, 3]),
    ];
    let target = [9, 6, 6];
    let result = [vec![0, 1], vec![1, 2], vec![0, 1]];

    let size = target.len();
    for i in 0..size {
        assert_eq!(two_sum(nums[i].take().unwrap(), target[i]), result[i]);
    }
}
