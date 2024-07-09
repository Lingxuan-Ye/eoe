macro_rules! error {
    ($($arg:tt)*) => {
        std::eprintln!(
            "{}: {}",
            ansi_term::Colour::Red.bold().paint("error"),
            std::format_args!("{}", $($arg)*)
        )
    };
}

macro_rules! caused_by {
    ($($arg:tt)*) => {
        std::eprintln!(
            "{}: {}",
            ansi_term::Colour::Red.bold().paint("caused by"),
            std::format_args!("{}", $($arg)*)
        )
    };
}
