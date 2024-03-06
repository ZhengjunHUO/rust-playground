use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use unionfind::UF;

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
        // 使用reverse来实现一个Min Priority Queue
        self.cost_accumulated.cmp(&other.cost_accumulated).reverse()
    }
}

impl PartialOrd for Flight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Solve leetcode [0787] Cheapest Flights Within K Stops
#[allow(unused_variables)]
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    // 字典形式的邻接表，value为[邻接结点编号，花费]
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

    while !pq.is_empty() {
        // 优先pop出累计花费最少的点
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

    -1
}

#[test]
fn test_find_cheapest_price() {
    let mut f = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    assert_eq!(find_cheapest_price(3, f, 0, 2, 1), 200);

    f = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    assert_eq!(find_cheapest_price(3, f, 0, 2, 0), 500);
}

// Solve leetcode [0684] Redundant Connection
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut uf = UF::new(n + 1);

    for edg in edges.iter() {
        if uf.is_linked(edg[0] as usize, edg[1] as usize) {
            return edg.clone();
        }

        uf.union(edg[0] as usize, edg[1] as usize);
    }

    vec![]
}

#[test]
fn test_find_redundant_connection() {
    let edges = vec![
        vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]],
    ];
    let expected = vec![vec![2, 3], vec![1, 4]];
    let rslt: Vec<Vec<i32>> = edges
        .into_iter()
        .map(|a| find_redundant_connection(a))
        .collect();
    assert_eq!(rslt, expected);
}
