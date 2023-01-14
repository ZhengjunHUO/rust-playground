// Solve leetcode [0322] Coin Change
#[allow(dead_code)]
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
