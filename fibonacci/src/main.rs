fn main() {
    let fibo_iter = {
        use std::ops::Index;

        // mapping the given index to inner buffer's index
        struct FiboWithOffset<'b> {
            buf_ref: &'b [u64; 2],
            offset: usize,
        }

        impl<'b> Index<usize> for FiboWithOffset<'b> {
            type Output = u64;

            fn index(&self, index: usize) -> &u64 {
                &self.buf_ref[index + 2 - self.offset]
            }
        }

        // store the current position and last 2 values
        struct Fibo {
            buf: [u64; 2],
            idx: usize,
        }

        impl Iterator for Fibo {
            type Item = u64;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx < 2 {
                    // get return value directly
                    let ret = self.buf[self.idx];
                    // no need to update buf
                    // update index
                    self.idx += 1;
                    Some(ret)
                } else {
                    // calculate return value
                    let ret = {
                        let fo = FiboWithOffset {
                            buf_ref: &self.buf,
                            offset: self.idx,
                        };
                        let n = self.idx;
                        fo[n - 1] + fo[n - 2]
                    };

                    // update buf
                    {
                        use std::mem::swap;
                        let mut swap_val = ret;
                        for i in (0..2).rev() {
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
            buf: [0, 1],
            idx: 0,
        }
    };

    for elem in fibo_iter.take(15) {
        println!("{}", elem)
    }
}
