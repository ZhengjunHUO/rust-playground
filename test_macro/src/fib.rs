#[macro_export]
macro_rules! expr_len {
    () => {0};
    ($first:expr) => {1};
    ($first:expr, $($rest:expr),*) => (1 + expr_len!($($rest),*));
}

#[macro_export]
macro_rules! print_fibo {
    ($vec:ident [ $indx: ident ]: $nty:ty = $($bases:expr),+ => $rule:expr) => {{
        use std::ops::Index;

        const BUF_SIZE: usize = expr_len!($($bases),+);

        // mapping the given index to inner buffer's index
        struct FiboWithOffset<'b> {
            buf_ref: &'b [$nty; BUF_SIZE],
            offset: usize,
        }

        impl<'b> Index<usize> for FiboWithOffset<'b> {
            type Output = $nty;

            fn index(&self, index: usize) -> &$nty {
                &self.buf_ref[index + BUF_SIZE - self.offset]
            }
        }

        // store the current position and last BUF_SIZE values
        struct Fibo {
            buf: [$nty; BUF_SIZE],
            idx: usize,
        }

        impl Iterator for Fibo {
            type Item = $nty;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx < BUF_SIZE {
                    // get return value directly
                    let ret = self.buf[self.idx];
                    // no need to update buf
                    // update index
                    self.idx += 1;
                    Some(ret)
                } else {
                    // calculate return value
                    let ret = {
                        let $vec = FiboWithOffset {
                            buf_ref: &self.buf,
                            offset: self.idx,
                        };
                        let $indx = self.idx;
                        $rule
                    };

                    // update buf
                    {
                        use std::mem::swap;
                        let mut swap_val = ret;
                        for i in (0..BUF_SIZE).rev() {
                            swap(&mut swap_val, &mut self.buf[i]);
                        }
                    }
                    // update index
                    self.idx += 1;
                    Some(ret)
                }
            }
        }

        Fibo {
            buf: [$($bases),*],
            idx: 0,
        }
    }};
}
