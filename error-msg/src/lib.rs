#[macro_export]
macro_rules! error_msg {
	($msg:literal) => {
		anyhow!(format!("{}:{} error_msg: {}", file!(), line!(), $msg))
	};
    ($fmt:expr, $($arg:tt)*) => {
        anyhow!(format!("{}:{} error_msg: {}", file!(), line!(), format!($fmt, $($arg)*)))
    };
}