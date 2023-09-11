// Solve leetcode [1011] Capacity To Ship Packages Within D Days
pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    fn days_needed(weights: &Vec<i32>, capacity: i32) -> i32 {
        let mut rest = capacity;
        let mut rslt = 1;

        for w in weights {
            if *w > capacity {
                return 50001;
            }

            if *w > rest {
                rslt += 1;
                rest = capacity - *w;
            } else {
                rest -= *w;
            }
        }

        rslt
    }

    let mut l = 1;
    let mut r = 25000000 + 1;

    while l < r {
        let m = l + (r - l) / 2;
        if days_needed(&weights, m) <= days {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}

#[test]
fn test_ship_within_days() {
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
