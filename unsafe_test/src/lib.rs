pub mod foreign;
pub mod raw;

#[test]
fn test_ref_flag() {
    use crate::raw::RefAndFlag;

    let v = vec![1, 2, 3];
    let rf = RefAndFlag::new(&v, true);
    assert_eq!(rf.get_ref()[2], 3);
    assert_eq!(rf.get_flag(), true);
}

#[test]
fn test_foreign() {
    use crate::foreign::strlen;

    let s = "Feed me to C func!";
    let s_c = std::ffi::CString::new(s).unwrap();
    unsafe {
        assert_eq!(strlen(s_c.as_ptr()), 18);
    }
}
