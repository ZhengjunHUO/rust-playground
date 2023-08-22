use crossbeam::scope;

fn main() {
    let list = vec![100, 35, 27, 1024, 198, 666, 1080];
    assert_eq!(1080, calc_max(&list));

    let empty_list = vec![];
    assert_eq!(0, calc_max(&empty_list));
}

fn calc_max(list: &[u32]) -> u32 {
    let n = list.len();

    match n {
        0 => return 0,
        1 => return list[0],
        _ => {
            let pivot = n / 2;
            scope(|s| {
                let left_handler = s.spawn(|_| { calc_max(&list[..pivot]) });
                let right_handler = s.spawn(|_| { calc_max(&list[pivot..]) });

                let left_max = left_handler.join().unwrap();
                let right_max = right_handler.join().unwrap();
                left_max.max(right_max)
            }).unwrap()
        }
    }
}
