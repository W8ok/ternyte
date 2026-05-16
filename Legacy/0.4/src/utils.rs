use std::{sync::OnceLock, time::Instant};

pub static START: OnceLock<Instant> = OnceLock::new();

#[macro_export]
macro_rules! timestamp {
    () => {{
        let start = $crate::utils::START.get_or_init(|| std::time::Instant::now());
        let elapsed = start.elapsed();
        let ms = elapsed.as_millis();
        let hours = ms / 3600000;
        let minutes = (ms % 3600000) / 60000;
        let seconds = (ms % 60000) / 1000;
        let millis = ms % 1000;
        format!("[{:02}:{:02}:{:02}.{:03}]", hours, minutes, seconds, millis)
    }};
}

#[macro_export]
macro_rules! log {
    (error, $($arg:tt)*) => {
        eprintln!("{} [ERROR] {}", $crate::timestamp!(), format!($($arg)*));
    };
    (warn, $($arg:tt)*) => {
        eprintln!("{} [WARN] {}", $crate::timestamp!(), format!($($arg)*));
    };
    (info, $($arg:tt)*) => {
        println!("{} [INFO] {}", $crate::timestamp!(), format!($($arg)*));
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("{} [DEBUG] {}", $crate::timestamp!(), format!($($arg)*));
    };
}
