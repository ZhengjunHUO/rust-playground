use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Flight {
    node_id: i32,
    hops_left: i32,
    cost_accumulated: i32,
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.cost_accumulated == other.cost_accumulated
    }
}

impl Eq for Flight {}

impl Ord for Flight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost_accumulated.cmp(&other.cost_accumulated).reverse()
    }
}

impl PartialOrd for Flight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut dict = HashMap::<i32, Vec<[i32; 2]>>::new();

    for f in flights {
        dict.entry(f[0])
            .and_modify(|e| e.push([f[1], f[2]]))
            .or_insert(vec![[f[1], f[2]]]);
    }

    let mut pq = BinaryHeap::new();
    pq.push(Flight {
        node_id: src,
        hops_left: k + 1,
        cost_accumulated: 0,
    });

    while pq.len() > 0 {
        let curr = pq.pop().unwrap();
        if curr.node_id == dst {
            return curr.cost_accumulated;
        }

        if curr.hops_left > 0 {
            dict.entry(curr.node_id).and_modify(|e| {
                for i in e {
                    pq.push(Flight {
                        node_id: i[0],
                        hops_left: curr.hops_left - 1,
                        cost_accumulated: curr.cost_accumulated + i[1],
                    });
                }
            });
        }
    }

    return -1;
}
