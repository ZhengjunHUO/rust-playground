use std::collections::HashMap;

// Solve leetcode [0076] Minimum Window Substring
pub fn min_window(s: String, t: String) -> String {
    // 需要找到的各元素，及对应的次数（值减为0 => 该元素已找齐）
    let mut elem_counter = HashMap::new();
    for i in t.bytes() {
        elem_counter.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }

    // 需要找到的元素个数（值减为0 => valid状态）
    let mut counter = t.len();
    // 左右指针
    let mut lp = 0;
    let mut rp = 0;
    // 存储结果
    let mut start_pos = 0;
    let mut min_size = 100000;

    // 右指针遍历到底
    while rp < s.len() {
        //右指针行动
        if let Some(v) = elem_counter.get(&s.as_bytes()[rp]) {
            if *v > 0 {
                counter -= 1;
            }
        }

        elem_counter
            .entry(s.as_bytes()[rp])
            .and_modify(|e| *e -= 1)
            .or_insert(-1);
        rp += 1;

        while counter == 0 {
            //更新较优解
            if (rp - lp) < min_size {
                start_pos = lp;
                min_size = rp - lp;
            }

            //左指针行动（仅在valid状态下移动）
            elem_counter
                .entry(s.as_bytes()[lp])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            if let Some(v) = elem_counter.get(&s.as_bytes()[lp]) {
                if *v > 0 {
                    counter += 1;
                }
            }
            lp += 1;
        }
    }

    if min_size < 100000 {
        return s[start_pos..start_pos + min_size].to_string();
    }

    String::from("")
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
