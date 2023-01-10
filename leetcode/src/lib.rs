mod graph;
mod hashtable;

use crate::graph::*;
use crate::hashtable::*;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_find_cheapest_price() {
        let mut f = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, f, 0, 2, 1), 200);

        f = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, f, 0, 2, 0), 500);
    }
}
