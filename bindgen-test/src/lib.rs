#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_comp_decomp() {
        let text = include_str!("../bpf_clang.txt").as_bytes();
        let text_size = text.len();
        let mut compressed: Vec<u8> = vec![0; text_size];
        let mut decompressed: Vec<u8> = vec![0; text_size];

        unsafe {
            // 1. init bz stream
            let mut stream: bz_stream = mem::zeroed();
            let res = BZ2_bzCompressInit(&mut stream as *mut _, 1, 4, 0);

            // check result
            match res {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"), // -2
                r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),     // -3
                r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"), // -9
                r if r == (BZ_OK as _) => {}                                 // 0
                r => panic!("Init failed, return error code: {}", r),
            }

            // 2. compress text
            stream.next_in = text.as_ptr() as *mut _;
            stream.avail_in = text_size as _;
            stream.next_out = compressed.as_mut_ptr() as *mut _;
            stream.avail_out = compressed.len() as _;

            let res = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _); // 2

            // check result
            match res {
                r if r == (BZ_SEQUENCE_ERROR as _) => panic!("BZ_SEQUENCE_ERROR"), // -1
                r if r == (BZ_RUN_OK as _) => panic!("BZ_RUN_OK"),                 // 1
                r if r == (BZ_FLUSH_OK as _) => panic!("BZ_FLUSH_OK"),             // 2
                r if r == (BZ_FINISH_OK as _) => panic!("BZ_FINISH_OK"),           // 3
                r if r == (BZ_STREAM_END as _) => {}                               // 4
                r => panic!("Compress failed, return error code: {}", r),
            }

            // 3. compress finish
            let res = BZ2_bzCompressEnd(&mut stream as *mut _);
            match res {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"), // -2
                r if r == (BZ_OK as _) => {}                                 // 0
                r => panic!("Compress end failed, return error code: {}", r),
            }
        }
    }
}
