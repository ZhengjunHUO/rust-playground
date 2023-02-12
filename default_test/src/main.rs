use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Extend;

#[derive(Default)]
struct Collector<T> {
    some_field: usize,
    dict: HashSet<T>,
}

impl<T> Extend<T> for Collector<T>
where
    T: Eq + Hash,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        HashSet::extend(&mut self.dict, iter);
    }
}

fn main() {
    // #1 HashSet implements Default & Extend<T>
    let hs: [i32; 8] = [4, 18, 64, 108, 126, 512, 700, 1024];
    //let (pw_two, others): (HashSet<i32>, HashSet<i32>) = hs.iter().partition(|&n| n & (n - 1) == 0);
    let (pw_two, others): (Collector<i32>, Collector<i32>) =
        hs.into_iter().partition(|n| n & (n - 1) == 0);

    assert_eq!(pw_two.some_field, 0);
    assert_eq!(pw_two.dict.len(), 4);
    assert_eq!(others.dict.len(), 4);

    // #2 String implements Default & Extend<T>
    let (maj, min): (String, String) = "Rust Rocks".chars().partition(|&c| c.is_uppercase());
    assert_eq!(maj, "RR");
    assert_eq!(min, "ust ocks");
}
