//! This crate provides utilities for exiting processes on errors
//! gracefully, leveraging [`anyhow`] to display detailed error
//! context and chained messages.
//!
//! # Examples
//!
//! Exiting on error:
//!
//! ```should_panic
//! use anyhow::{Context, anyhow};
//! use eoe::ExitOnError;
//!
//! Err::<(), _>(anyhow!("Mm-noom-ba-deh"))
//!     .context("Doom-boom-ba-beh")
//!     .context("Doo-boo-boom-ba-beh-beh")
//!     .exit_on_error();
//! ```
//!
//! <details>
//! <summary> Show output </summary>
//! <div style="background-color: #1e1e1e; font-family: monospace; padding: 10px; border-radius: 5px;">
//!     <span style="color: #f14c4c; font-weight: bold">error</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Doo-boo-boom-ba-beh-beh</span><br>
//!     <span style="color: #f14c4c; font-weight: bold">caused by</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Doom-boom-ba-beh</span><br>
//!     <span style="color: #f14c4c; font-weight: bold">caused by</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Mm-noom-ba-deh</span><br>
//! </div>
//! </details>
//!
//! Or if you prefer the word *quit*:
//!
//! ```should_panic
//! use anyhow::{Context, anyhow};
//! use eoe::QuitOnError;
//!
//! Err::<(), _>(anyhow!("Mm-ba-ba-beh, mm-ba-ba-beh"))
//!     .context("Dee-day-da, ee-day-da")
//!     .quit_on_error();
//! ```
//!
//! <details>
//! <summary> Show output </summary>
//! <div style="background-color: #1e1e1e; font-family: monospace; padding: 10px; border-radius: 5px;">
//!     <span style="color: #f14c4c; font-weight: bold">error</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Dee-day-da, ee-day-da</span><br>
//!     <span style="color: #f14c4c; font-weight: bold">caused by</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Mm-ba-ba-beh, mm-ba-ba-beh</span><br>
//! </div>
//! </details>
//!
//! The output style is customizable:
//!
//! ```should_panic
//! use eoe::{ExitOnError, Segment, Style};
//!
//! let _ = eoe::ERROR.set(Segment {
//!     style: Style::new().bold().blue(),
//!     value: "Watchin' some good friends screamin'",
//! });
//! let _ = eoe::SEP.set(Segment {
//!     style: Style::new(),
//!     value: " 😱 ",
//! });
//! let _ = eoe::MESSAGE_STYLE.set(Style::new().italic().yellow());
//! let _ = eoe::MESSAGE_ON_NONE.set("Let me out");
//!
//! None::<()>.exit_on_error();
//! ```
//!
//! <details>
//! <summary> Show output </summary>
//! <div style="background-color: #1e1e1e; font-family: monospace; padding: 10px; border-radius: 5px;">
//!     <span style="color: #3b8eea; font-weight: bold">Watchin' some good friends screamin'</span><span> 😱 </span><span style="color: #e5e510; font-style: italic">Let me out</span><br>
//! </div>
//! </details>

pub use owo_colors::Style;

use anyhow::Error;
use owo_colors::{OwoColorize, Stream};
use std::fmt::Display;
use std::io::{self, Write, stderr};
use std::process::exit;
use std::sync::OnceLock;

/// The *error* label.
pub static ERROR: OnceLock<Segment<&str>> = OnceLock::new();
/// The *caused by* label.
pub static CAUSED_BY: OnceLock<Segment<&str>> = OnceLock::new();
/// The separator between the label and the message.
pub static SEP: OnceLock<Segment<&str>> = OnceLock::new();
/// The style of messages.
pub static MESSAGE_STYLE: OnceLock<Style> = OnceLock::new();
/// The message to display when exiting on `None`.
pub static MESSAGE_ON_NONE: OnceLock<&str> = OnceLock::new();

/// A trait for exiting processes gracefully.
pub trait ExitOnError<T>: internal::Sealed {
    /// Exits the process with an error message if the result is an error
    /// or the option is `None`.
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
                print_error(error);
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
                print_none();
                exit(1);
            }
            Some(value) => value,
        }
    }
}

/// Well, if you prefer the word *quit* to *exit*.
///
/// A trait for quitting processes gracefully.
pub trait QuitOnError<T>: internal::Sealed {
    /// Quits the process with an error message if the result is an error
    /// or the option is `None`.
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

/// A message segment.
#[derive(Debug)]
pub struct Segment<T> {
    /// The segment's style.
    pub style: Style,
    /// The segment's value, typically a piece of text.
    pub value: T,
}

impl<T> Segment<T> {
    fn display(&self, stream: Stream) -> impl Display
    where
        T: Display,
    {
        self.value
            .if_supports_color(stream, |value| value.style(self.style))
    }
}

fn print_error<E>(error: E)
where
    E: Into<Error>,
{
    if let Err(error) = try_print_error(error) {
        panic!("failed printing to stderr: {error}");
    }
}

fn try_print_error<E>(error: E) -> io::Result<()>
where
    E: Into<Error>,
{
    let mut stderr = stderr().lock();

    for (index, message) in error.into().chain().enumerate() {
        let label = if index == 0 {
            ERROR.get_or_init(|| default::ERROR)
        } else {
            CAUSED_BY.get_or_init(|| default::CAUSED_BY)
        };
        let sep = SEP.get_or_init(|| default::SEP);
        let message = Segment {
            style: *MESSAGE_STYLE.get_or_init(|| default::MESSAGE_STYLE),
            value: message,
        };

        writeln!(
            stderr,
            "{}{}{}",
            label.display(Stream::Stderr),
            sep.display(Stream::Stderr),
            message.display(Stream::Stderr)
        )?;
    }

    Ok(())
}

fn print_none() {
    if let Err(error) = try_print_none() {
        panic!("failed printing to stderr: {error}");
    }
}

fn try_print_none() -> io::Result<()> {
    let mut stderr = stderr().lock();

    let label = ERROR.get_or_init(|| default::ERROR);
    let sep = SEP.get_or_init(|| default::SEP);
    let message = Segment {
        style: *MESSAGE_STYLE.get_or_init(|| default::MESSAGE_STYLE),
        value: MESSAGE_ON_NONE.get_or_init(|| default::MESSAGE_ON_NONE),
    };

    writeln!(
        stderr,
        "{}{}{}",
        label.display(Stream::Stderr),
        sep.display(Stream::Stderr),
        message.display(Stream::Stderr)
    )
}

mod default {
    use super::{Segment, Style};

    pub(super) const ERROR: Segment<&str> = Segment {
        style: Style::new().red().bold(),
        value: "error:",
    };
    pub(super) const CAUSED_BY: Segment<&str> = Segment {
        style: Style::new().red().bold(),
        value: "caused by:",
    };
    pub(super) const SEP: Segment<&str> = Segment {
        style: Style::new(),
        value: " ",
    };
    pub(super) const MESSAGE_STYLE: Style = Style::new();
    pub(super) const MESSAGE_ON_NONE: &str = "unexpected None";
}

mod internal {
    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> where E: Into<super::Error> {}
    impl<T> Sealed for Option<T> {}
}
