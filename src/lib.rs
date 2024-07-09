use anyhow::Error;
use std::process::exit;

#[macro_use]
mod macros;

pub trait ExitOnError<T>: internal::Sealed {
    fn exit_on_error(self) -> T;
}

impl<T, E> ExitOnError<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn exit_on_error(self) -> T {
        match self {
            Err(error) => {
                let error = error.into();
                error!(error);
                error.chain().skip(1).for_each(|cause| caused_by!(cause));
                exit(1);
            }
            Ok(value) => value,
        }
    }
}

impl<T> ExitOnError<T> for Option<T> {
    fn exit_on_error(self) -> T {
        match self {
            None => {
                error!("unexpected None");
                exit(1);
            }
            Some(value) => value,
        }
    }
}

mod internal {
    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> where E: Into<super::Error> {}
    impl<T> Sealed for Option<T> {}
}
