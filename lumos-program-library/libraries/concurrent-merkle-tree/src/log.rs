macro_rules! lumos_logging {
    ($message:literal, $($arg:tt)*) => {
        #[cfg(feature = "log")]
        ::lumos_program::msg!($message, $($arg)*);
    };
    ($message:literal) => {
        #[cfg(feature = "log")]
        ::lumos_program::msg!($message);
    };
}

macro_rules! log_compute {
    () => {
        #[cfg(all(feature = "sol-log", feature = "log"))]
        ::lumos_program::log::sol_log_compute_units();
    };
}
