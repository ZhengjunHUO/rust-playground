use std::ops::Range;

pub struct GapBuffer<T> {
    // 分配一块有capacity的内存，直接使用raw pointer操作数据
    // len始终为0，从Rust的角度看chunk一直未被使用
    chunk: Vec<T>,

    // chunk中空白区域首尾的指针，加减1移动type T大小的字节
    // 类似列表中的index
    gap: Range<usize>,
}

impl<T> GapBuffer<T> {
    pub fn new() -> GapBuffer<T> {
        GapBuffer {
            chunk: Vec::new(),
            gap: 0..0,
        }
    }

    pub fn insert_iter(&mut self, iter: impl Iterator<Item = T>) {
        // TO IMPLEMENT
    }

    pub fn set_pos(&mut self, pos: usize) {
        // TO IMPLEMENT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_buffer() {
        let mut gb = GapBuffer::new();
        gb.insert_iter("A rustacean here ?".chars());
        gb.set_pos(2);
        gb.insert_iter("skillful ".chars());
    }
}
