// Solve leetcode [0322] Coin Change
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // 最小面额为1，故最大值不会超过amount/1
    let mut dp = vec![amount + 1; ((amount + 1) as usize).try_into().unwrap()];

    // 确保dp[0]为0, 即amount为0时需要0枚硬币
    dp[0] = 0;

    for i in 0..=amount {
        for c in &coins {
            if i - c < 0 {
                continue;
            }

            // dp[i]即凑到金额i需要的硬币数等于: min{dp[i-c]|c属于coins}+1
            let temp = dp[(i - c) as usize] + 1;
            if temp < dp[i as usize] {
                dp[i as usize] = temp
            }
        }
    }

    if dp[dp.len() - 1] == amount + 1 {
        return -1;
    } else {
        return dp[dp.len() - 1];
    }
}

#[test]
fn test_coin_change() {
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
