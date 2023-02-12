use std::collections::HashSet;

fn main() {
    // #1 HashSet implements Default & Extend<T>
    let hs = [4, 18, 64, 108, 126, 512, 700, 1024];
    let (pw_two, others): (HashSet<i32>, HashSet<i32>) = hs.iter().partition(|&n| n & (n - 1) == 0);

    assert_eq!(pw_two.len(), 4);
    assert_eq!(others.len(), 4);

    // #2 String implements Default & Extend<T>
    let (maj, min): (String, String) = "Rust Rocks".chars().partition(|&c| c.is_uppercase());
    assert_eq!(maj, "RR");
    assert_eq!(min, "ust ocks");
}
