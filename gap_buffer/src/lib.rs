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

    pub fn cap(&self) -> usize {
        self.chunk.capacity()
    }

    // GapBuffer中实际存放数据长度为capacity减去gap的大小
    pub fn len(&self) -> usize {
        self.cap() - self.gap.len()
    }

    // 当前插入点为gap区域的头部
    pub fn pos(&self) -> usize {
        self.gap.start
    }

    // 存储内容在逻辑上的index映射到chunk中物理的index
    // 如果实际上落在gap区域之后则需要加上gap大小的offset
    pub fn raw_index_from(&self, idx: usize) -> usize {
        if idx < self.gap.start {
            return idx;
        }

        idx + self.gap.len()
    }

    // idx不能超过chunk的capacity的范围
    unsafe fn get_ptr_at(&self, idx: usize) -> *const T {
        self.chunk.as_ptr().offset(idx as isize)
    }

    // 和get_ptr_at类似，返回一个可变的raw pointer
    unsafe fn get_mut_ptr_at(&mut self, idx: usize) -> *mut T {
        self.chunk.as_mut_ptr().offset(idx as isize)
    }

    // 获取存储内容的第idx个元素的ref
    pub fn get(&self, idx: usize) -> Option<&T> {
        let raw_idx = self.raw_index_from(idx);
        if raw_idx < self.cap() {
            unsafe { return Some(&*self.get_ptr_at(raw_idx)) }
        }
        None
    }

    pub fn insert_iter(&mut self, iter: impl Iterator<Item = T>) {
        // TO IMPLEMENT
    }

    pub fn move_insert_point_to(&mut self, pos: usize) {
        // TO IMPLEMENT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_buffer() {
        let mut gb = GapBuffer::new();
        assert_eq!(gb.len(), 0);
        assert_eq!(gb.cap(), 0);
        assert_eq!(gb.pos(), 0);

        gb.insert_iter("A rustacean here ?".chars());
        //assert_eq!(gb.len(), 18);
        gb.move_insert_point_to(2);
        gb.insert_iter("skillful ".chars());
    }
}
