//! This crate provides utilities for exiting processes on errors
//! gracefully, leveraging [`anyhow`] to display detailed error
//! context and chained messages.

use anyhow::Error;
use owo_colors::{OwoColorize, Stream, Style};
use std::fmt::Display;
use std::io::{StderrLock, Write, stderr};
use std::process::exit;
use std::sync::OnceLock;

pub static ERROR: OnceLock<Segment<&str>> = OnceLock::new();
pub static CAUSED_BY: OnceLock<Segment<&str>> = OnceLock::new();
pub static SEP: OnceLock<Segment<&str>> = OnceLock::new();
pub static MESSAGE_STYLE: OnceLock<Style> = OnceLock::new();
pub static MESSAGE_ON_NONE: OnceLock<&str> = OnceLock::new();

/// Exits the process with an error message if the result is an error
/// or the option is `None`.
///
/// # Examples
///
/// On error:
///
/// ```should_panic
/// use anyhow::{Context, anyhow};
/// use eoe::ExitOnError;
///
/// Err::<(), _>(anyhow!("Mm-noom-ba-deh"))
///     .context("Doom-boom-ba-beh")
///     .context("Doo-boo-boom-ba-beh-beh")
///     .exit_on_error();
/// ```
///
/// On `None`:
///
/// ```should_panic
/// # use eoe::ExitOnError;
/// #
/// None::<()>.exit_on_error();
/// ```
pub trait ExitOnError<T>: internal::Sealed {
    fn exit_on_error(self) -> T;
}

impl<T, E> ExitOnError<T> for Result<T, E>
where
    E: Into<Error>,
{
    /// Exits the process with an error message if the result is an error.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use anyhow::{Context, anyhow};
    /// use eoe::ExitOnError;
    ///
    /// Err::<(), _>(anyhow!("Mm-noom-ba-deh"))
    ///     .context("Doom-boom-ba-beh")
    ///     .context("Doo-boo-boom-ba-beh-beh")
    ///     .exit_on_error();
    /// ```
    fn exit_on_error(self) -> T {
        match self {
            Err(error) => {
                let error: Error = error.into();
                let mut stderr = stderr().lock();
                print_error(&mut stderr, &error);
                error.chain().skip(1).for_each(|cause| {
                    print_caused_by(&mut stderr, cause);
                });
                exit(1);
            }
            Ok(value) => value,
        }
    }
}

impl<T> ExitOnError<T> for Option<T> {
    /// Exits the process with an error message if the option is `None`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use eoe::ExitOnError;
    ///
    /// None::<()>.exit_on_error();
    /// ```
    fn exit_on_error(self) -> T {
        match self {
            None => {
                let mut stderr = stderr().lock();
                let message = MESSAGE_ON_NONE.get_or_init(|| Fallback::MESSAGE_ON_NONE);
                print_error(&mut stderr, message);
                exit(1);
            }
            Some(value) => value,
        }
    }
}

/// Well, if you prefer the word `quit` to `exit`.
///
/// Quits the process with an error message if the result is an error
/// or the option is `None`.
///
/// # Examples
///
/// On error:
///
/// ```should_panic
/// use anyhow::{Context, anyhow};
/// use eoe::QuitOnError;
///
/// Err::<(), _>(anyhow!("Mm-ba-ba-beh, mm-ba-ba-beh"))
///     .context("Dee-day-da, ee-day-da")
///     .quit_on_error();
/// ```
///
/// On `None`:
///
/// ```should_panic
/// # use eoe::QuitOnError;
/// #
/// None::<()>.quit_on_error();
/// ```
pub trait QuitOnError<T>: internal::Sealed {
    fn quit_on_error(self) -> T;
}

impl<T, E> QuitOnError<T> for Result<T, E>
where
    E: Into<Error>,
{
    /// Quits the process with an error message if the result is an error.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use anyhow::{Context, anyhow};
    /// use eoe::QuitOnError;
    ///
    /// Err::<(), _>(anyhow!("Mm-ba-ba-beh, mm-ba-ba-beh"))
    ///     .context("Dee-day-da, ee-day-da")
    ///     .quit_on_error();
    /// ```
    fn quit_on_error(self) -> T {
        self.exit_on_error()
    }
}

impl<T> QuitOnError<T> for Option<T> {
    /// Quits the process with an error message if the option is `None`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use eoe::QuitOnError;
    ///
    /// None::<()>.quit_on_error();
    /// ```
    fn quit_on_error(self) -> T {
        self.exit_on_error()
    }
}

#[derive(Debug)]
pub struct Segment<T> {
    pub style: Style,
    pub value: T,
}

impl<T> Segment<T>
where
    T: Display,
{
    fn display(&self, stream: Stream) -> impl Display {
        self.value
            .if_supports_color(stream, |value| value.style(self.style))
    }
}

struct Fallback;

impl Fallback {
    const ERROR: Segment<&str> = Segment {
        style: Style::new().red().bold(),
        value: "error",
    };
    const CAUSED_BY: Segment<&str> = Segment {
        style: Style::new().red().bold(),
        value: "caused by",
    };
    const SEP: Segment<&str> = Segment {
        style: Style::new().red().bold(),
        value: ": ",
    };
    const MESSAGE_STYLE: Style = Style::new();
    const MESSAGE_ON_NONE: &str = "unexpected None";
}

fn print_error<M>(stderr: &mut StderrLock, message: M)
where
    M: Display,
{
    let label = ERROR.get_or_init(|| Fallback::ERROR);
    print(stderr, label, message);
}

fn print_caused_by<M>(stderr: &mut StderrLock, message: M)
where
    M: Display,
{
    let label = CAUSED_BY.get_or_init(|| Fallback::CAUSED_BY);
    print(stderr, label, message);
}

fn print<M>(stderr: &mut StderrLock, label: &Segment<&str>, message: M)
where
    M: Display,
{
    let sep = SEP.get_or_init(|| Fallback::SEP);
    let message = Segment {
        style: *MESSAGE_STYLE.get_or_init(|| Fallback::MESSAGE_STYLE),
        value: message,
    };
    if let Err(io_error) = writeln!(
        stderr,
        "{}{}{}",
        label.display(Stream::Stderr),
        sep.display(Stream::Stderr),
        message.display(Stream::Stderr)
    ) {
        panic!("failed printing to stderr: {}", io_error);
    }
}

mod internal {
    use super::Error;

    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> where E: Into<Error> {}
    impl<T> Sealed for Option<T> {}
}
