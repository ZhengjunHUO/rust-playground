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
                r => panic!("Init compress stream failed, return error code: {}", r),
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

            // 3. close compress stream
            let res = BZ2_bzCompressEnd(&mut stream as *mut _);
            match res {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"), // -2
                r if r == (BZ_OK as _) => {}                                 // 0
                r => panic!("Close compress stream failed, return error code: {}", r),
            }

            // 4. init decompress bz stream
            let mut stream_de: bz_stream = mem::zeroed();
            let res = BZ2_bzDecompressInit(&mut stream_de as *mut _, 4, 0);
            match res {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"), // -2
                r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),     // -3
                r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"), // -9
                r if r == (BZ_OK as _) => {}                                 // 0
                r => panic!("Init decompress stream failed, return error code: {}", r),
            }

            // 5. decompress compressed text
            stream_de.next_in = compressed.as_ptr() as *mut _;
            stream_de.avail_in = compressed.len() as _;
            stream_de.next_out = decompressed.as_mut_ptr() as *mut _;
            stream_de.avail_out = decompressed.len() as _;

            let res = BZ2_bzDecompress(&mut stream_de as *mut _);
            match res {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"), // -2
                r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),     // -3
                r if r == (BZ_DATA_ERROR as _) => panic!("BZ_DATA_ERROR"),   // -4
                r if r == (BZ_DATA_ERROR_MAGIC as _) => panic!("BZ_DATA_ERROR_MAGIC"), // -5
                r if r == (BZ_OK as _) => panic!("BZ_OK"),                   // 0
                r if r == (BZ_STREAM_END as _) => {}                         // 4
                r => panic!("Decompress failed, return error code: {}", r),
            }

            // 6. close decompress stream
            let res = BZ2_bzDecompressEnd(&mut stream_de as *mut _);
            match res {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"), // -2
                r if r == (BZ_OK as _) => {}                                 // 0
                r => panic!("Close decompress stream failed, return error code: {}", r),
            }

            // 7. check if origin text equals to the compressed && decompressed result
            assert_eq!(text, &decompressed[..]);
        }
    }
}
