macro_rules! exit_err {
    ($reason:literal) => {
        eprintln!("{}", $reason);
        eprintln!("This caused RustyAWK to exit with status 1");
        std::process::exit(1);
    };

    ($fmt:literal, $($arg:expr),+ $(,)?) => {
        eprintln!($fmt, $($arg),+);
        eprintln!("This caused RustyAWK to exit with status 1");
        std::process::exit(1);
    };
}
