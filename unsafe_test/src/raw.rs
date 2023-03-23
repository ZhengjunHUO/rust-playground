use std::marker::PhantomData;
use std::mem;

pub struct RefAndFlag<'a, T> {
    // usize的最低位用来存放1 bit的bool值
    ptr_with_bool: usize,
    // 不占用空间，用来挂靠'a和T
    lifetime_holder: PhantomData<&'a T>,
}

impl<'a, T: 'a> RefAndFlag<'a, T> {
    pub fn new(rf: &'a T, f: bool) -> RefAndFlag<T> {
        // 对T的要求其对齐大于2^1, 不适用于u8, bool等
        assert!(mem::align_of::<T>() % 2 == 0);
        RefAndFlag {
            ptr_with_bool: rf as *const T as usize | f as usize,
            lifetime_holder: PhantomData,
        }
    }

    pub fn get_ref(&self) -> &'a T {
        unsafe {
            // 将最低位清零
            let p = (self.ptr_with_bool & !1) as *const T;
            // 转换为&T
            &*p
        }
    }

    pub fn get_flag(&self) -> bool {
        // 查看最低位存放的bool值
        self.ptr_with_bool & 1 != 0
    }
}
