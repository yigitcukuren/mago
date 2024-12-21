use std::error::Error;

pub mod clap;

pub fn print(error: impl Error) {
    mago_feedback::error!("{}", error);
    mago_feedback::debug!("{:#?}", error);

    if let Some(source) = error.source() {
        mago_feedback::debug!("{:#?}", source);
    }
}

pub fn bail<T>(error: impl Error) -> T {
    print(error);

    // we exit with a non-zero status code to indicate an error
    // if this build is debug build, we will panic instead
    if cfg!(debug_assertions) {
        panic!("bail");
    } else {
        std::process::exit(1);
    }
}
