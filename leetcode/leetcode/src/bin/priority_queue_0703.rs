use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct KthLargest<T: Ord + Copy> {
    // 最小优先队列
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord + Copy> KthLargest<T> {
    pub fn new(k: usize, nums: &[T]) -> Self {
        let mut heap = BinaryHeap::new();
        for &i in nums {
            heap.push(Reverse(i));
        }

        // 只保留最大的k个，heap顶为剩余k个中的最小值
        for _ in 0..(nums.len() - k) {
            heap.pop();
        }

        KthLargest { heap }
    }

    pub fn add(&mut self, num: T) -> T {
        // 加入新值后重新计算最小值，总数变为k+1个
        self.heap.push(Reverse(num));
        // 去掉heap顶第k+1大的值（当前最小值）
        self.heap.pop();
        // 返回heap顶（第k大的值）
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kthlargest_add() {
        let mut k = KthLargest::new(3, &[4, 5, 8, 2]);
        assert_eq!(k.add(3), 4);
        assert_eq!(k.add(5), 5);
        assert_eq!(k.add(10), 5);
        assert_eq!(k.add(9), 8);
        assert_eq!(k.add(4), 8);
    }
}