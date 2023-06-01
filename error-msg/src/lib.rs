#[macro_export]
macro_rules! error_msg {
	($msg:literal) => {
		anyhow::anyhow!(format!("{}:{} error_msg: {}", file!(), line!(), $msg))
	};
    ($fmt:expr, $($arg:tt)*) => {
        anyhow::anyhow!(format!("{}:{} error_msg: {}", file!(), line!(), format!($fmt, $($arg)*)))
    };
}