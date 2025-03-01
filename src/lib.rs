//! This crate provides utilities for exiting processes on errors
//! gracefully, leveraging [`anyhow`] to display detailed error
//! context and chained messages.
//!
//! It is recommended to use the re-exported version of [`anyhow`]
//! to avoid potential version conflicts.

pub use anyhow;

use std::process;

#[macro_use]
mod macros;

/// Exits the process with an error message if the result is an error
/// or the option is `None`.
///
/// # Examples
///
/// On error:
///
/// ```should_panic
/// use eoe::ExitOnError;
/// use eoe::anyhow::{Context, anyhow};
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
    E: Into<anyhow::Error>,
{
    /// Exits the process with an error message if the result is an error.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use eoe::ExitOnError;
    /// use eoe::anyhow::{Context, anyhow};
    ///
    /// Err::<(), _>(anyhow!("Mm-noom-ba-deh"))
    ///     .context("Doom-boom-ba-beh")
    ///     .context("Doo-boo-boom-ba-beh-beh")
    ///     .exit_on_error();
    /// ```
    fn exit_on_error(self) -> T {
        match self {
            Err(error) => {
                let error = error.into();
                error!(error);
                error.chain().skip(1).for_each(|cause| caused_by!(cause));
                process::exit(1);
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
                error!("unexpected None");
                process::exit(1);
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
/// use eoe::QuitOnError;
/// use eoe::anyhow::{Context, anyhow};
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
    E: Into<anyhow::Error>,
{
    /// Quits the process with an error message if the result is an error.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use eoe::QuitOnError;
    /// use eoe::anyhow::{Context, anyhow};
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

mod internal {
    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> where E: Into<anyhow::Error> {}
    impl<T> Sealed for Option<T> {}
}
