#[macro_export]
macro_rules! capture_then_stringify {
    ($exp:expr) => {
        stringify!($exp)
    };
}

#[macro_export]
macro_rules! capture_then_check_tokens {
    ($exp:expr) => {
        check_tokens!($exp)
    };
}

#[macro_export]
macro_rules! check_tokens {
    ($a:tt ^ $b:tt) => {
        "this is an exponent"
    };
    (($i:ident)) => {
        "this is an identifier"
    };
    ($($other:tt)*) => {
        "this could be anything"
    };
}

#[macro_export]
macro_rules! capture_then_check_attribute {
    (#[$item:meta]) => {
        check_attribute!(#[$item])
    };
}

#[macro_export]
macro_rules! check_attribute {
    (#[macro_export]) => {
        "this is macro_export attribute"
    };
    (#[test]) => {
        "this is test attribute"
    };
    ($($item:tt)*) => {
        concat!("unknown attribute: ", stringify!($($item)*))
    };
}

#[macro_export]
macro_rules! pop_head {
    () => { "" };
    ($head:tt $($rest:tt)*) => {
        concat!("[", stringify!($head), "]\n", pop_head!($($rest)*))
    };
}

#[macro_export]
macro_rules! pop_tail {
    () => { "" };
    ($head:tt $($rest:tt)*) => {
        concat!(pop_tail!($($rest)*), "[", stringify!($head), "]\n")
    };
}
