#[macro_export]
macro_rules! task_spawn {
    ($future:expr) => {
        task_spawn!($future, FuturePrio::Low)
    };
    ($future:expr, $prio:expr) => {
        spawn_task($future, $prio)
    };
}

#[macro_export]
macro_rules! join {
    ($($future:expr),*) => {
        {
            let mut result = Vec::new();
            $(
                result.push(futures_lite::future::block_on($future));
            )*
            result
        }
    };
}

#[macro_export]
macro_rules! try_join {
    ($($future:expr),*) => {
        {
            let mut result = Vec::new();
            $(
                result.push(catch_unwind(|| futures_lite::future::block_on($future)));
            )*
            result
        }
    };
}
