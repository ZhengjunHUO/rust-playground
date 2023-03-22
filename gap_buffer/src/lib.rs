use std::ops::Range;
use std::ptr::{copy, copy_nonoverlapping, drop_in_place, read, write};

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

    pub fn move_insert_point_to(&mut self, pos: usize) {
        if pos > self.len() {
            panic!("Index {} out of range {}", pos, self.len())
        }

        unsafe {
            // 在gap前方插入，需要将gap前移，路径上的内容移动到gap后方
            if pos < self.gap.start {
                let count = self.gap.start - pos;
                copy(
                    self.get_ptr_at(pos),
                    self.get_mut_ptr_at(self.gap.end - count),
                    count,
                );
            }

            // 需要将gap首部后移至pos，路径上的内容移动到gap前方
            if pos > self.gap.start {
                let count = pos - self.gap.start;
                copy(
                    self.get_ptr_at(self.gap.end),
                    self.get_mut_ptr_at(self.gap.start),
                    count,
                );
            }

            self.gap = pos..pos + self.gap.len();
        }
    }

    pub fn insert(&mut self, elem: T) {
        if self.gap.len() == 0 {
            self.double_bufsize();
        }

        unsafe {
            write(self.get_mut_ptr_at(self.gap.start), elem);
        }
        self.gap.start += 1;
    }

    pub fn insert_iter(&mut self, iter: impl Iterator<Item = T>) {
        for elem in iter {
            self.insert(elem);
        }
    }

    // 删除插入点处的元素，即在gap区域之后的第一个元素
    pub fn remove_from_insert_point(&mut self) -> Option<T> {
        if self.gap.end == self.cap() {
            return None;
        }

        let rslt = unsafe { read(self.get_ptr_at(self.gap.end)) };
        self.gap.end += 1;
        Some(rslt)
    }

    pub fn double_bufsize(&mut self) {
        let mut new_cap = self.cap() * 2;
        // 第一次插入元素触发，初值为4
        if new_cap == 0 {
            new_cap = 4;
        }

        let mut doubled_chunk = Vec::with_capacity(new_cap);
        // 新增的空间用来扩展gap，其右边界右移
        let new_gap = self.gap.start..self.gap.end + (doubled_chunk.capacity() - self.cap());

        unsafe {
            // 把原本在gap前的内容copy到新buffer的头部
            copy_nonoverlapping(
                self.get_ptr_at(0),
                doubled_chunk.as_mut_ptr(),
                self.gap.start,
            );
            // 把原本在gap后的内容copy到新buffer的尾部
            copy_nonoverlapping(
                self.get_ptr_at(self.gap.end),
                doubled_chunk.as_mut_ptr().offset(new_gap.end as isize),
                self.cap() - self.gap.end,
            );
        }

        self.chunk = doubled_chunk;
        self.gap = new_gap;
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.gap.start {
                drop_in_place(self.get_mut_ptr_at(i))
            }
            for i in self.gap.end..self.cap() {
                drop_in_place(self.get_mut_ptr_at(i))
            }
        }
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
        assert_eq!(gb.len(), 18);
        assert_eq!(gb.cap(), 32);
        assert_eq!(gb.pos(), 18);

        gb.move_insert_point_to(2);
        assert_eq!(gb.pos(), 2);

        gb.insert_iter("skillful ".chars());
        assert_eq!(gb.len(), 27);
        assert_eq!(gb.cap(), 32);
        assert_eq!(gb.get(11), Some(&'r'));
        assert_eq!(gb.get(26), Some(&'?'));
    }
}
