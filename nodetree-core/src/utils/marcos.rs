macro_rules! all_some {
    () => {
        true
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head.is_some() && all_some!($($tail),*)
    };
}

macro_rules! all_none {
    () => {
        true
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head.is_none() && all_none!($($tail),*)
    };
}

#[macro_export]
macro_rules! log_and_bail {
    ($fmt:expr $(, $arg:expr)*) => {{
        let msg = format!(concat!(" [{}:{}]", $fmt), file!(), line!(), $($arg)*);
        tracing::error!("{}", msg);
        anyhow::bail!(msg);
    }};
}

pub(crate) use all_none;
pub(crate) use all_some;
pub(crate) use log_and_bail;
