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
