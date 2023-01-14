#![allow(unused_imports)]
mod binary_search;
mod dynamic_prog;
mod graph;
mod hashtable;
mod sliding_window;

use crate::binary_search::*;
use crate::dynamic_prog::*;
use crate::graph::*;
use crate::hashtable::*;
use crate::sliding_window::*;

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

    #[test]
    fn test_min_window() {
        let s = vec![String::from("ADOBECODEBANC"), String::from("a")];
        let t = vec![String::from("ABC"), String::from("a")];
        let o = vec![String::from("BANC"), String::from("a")];

        let rslt: Vec<String> = s
            .into_iter()
            .zip(t.into_iter())
            .map(|(a, b)| min_window(a, b))
            .collect();
        assert_eq!(rslt, o);
    }

    #[test]
    fn test_binary_search() {
        let w = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![3, 2, 2, 4, 1, 4],
            vec![1, 2, 3, 1, 1],
        ];
        let d = vec![5, 3, 4];
        let o = vec![15, 6, 3];

        let rslt: Vec<i32> = w
            .into_iter()
            .zip(d.into_iter())
            .map(|(a, b)| ship_within_days(a, b))
            .collect();
        assert_eq!(rslt, o);
    }

    #[test]
    fn test_dynamic_prog() {
        let coins = vec![vec![1, 2, 5], vec![2], vec![1], vec![1], vec![1]];
        let amounts = vec![11, 3, 0, 1, 2];
        let wants = vec![3, -1, 0, 1, 2];

        let rslt: Vec<i32> = coins
            .into_iter()
            .zip(amounts.into_iter())
            .map(|(a, b)| coin_change(a, b))
            .collect();
        assert_eq!(rslt, wants);
    }
}
