//! Command line entry point.
//!
//! Bloom is a GUI application, so on Windows release builds the linker gives it
//! no console of its own. A redirected stdout still works, but printing to the
//! terminal that launched it does not. `attach_console` borrows the parent's
//! console first, and only for the two flags that print and exit.

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "bloom",
    version,
    about = "A hardware accelerated media viewer",
    disable_help_flag = false
)]
pub struct Cli {
    /// Image or video file to open on start
    pub media: Option<PathBuf>,
}

#[cfg(windows)]
fn attach_console() {
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[cfg(not(windows))]
fn attach_console() {}

pub fn parse() -> Cli {
    let args: Vec<String> = std::env::args().collect();
    if args
        .iter()
        .skip(1)
        .any(|a| matches!(a.as_str(), "-h" | "--help" | "-V" | "--version"))
    {
        attach_console();
    }
    Cli::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("bloom").chain(args.iter().copied()))
    }

    #[test]
    fn a_path_is_still_taken_as_the_media_argument() {
        let cli = parse(&["photo.png"]).unwrap();
        assert_eq!(cli.media, Some(PathBuf::from("photo.png")));
    }

    #[test]
    fn no_arguments_leaves_the_media_unset() {
        assert_eq!(parse(&[]).unwrap().media, None);
    }

    #[test]
    fn version_and_help_do_not_become_a_path() {
        for flag in ["--version", "-V", "--help", "-h"] {
            let err = parse(&[flag]).expect_err("should exit, not parse a path");
            assert!(matches!(
                err.kind(),
                clap::error::ErrorKind::DisplayVersion | clap::error::ErrorKind::DisplayHelp
            ));
        }
    }

    #[test]
    fn an_unknown_flag_is_rejected_rather_than_opened() {
        let err = parse(&["--verson"]).expect_err("unknown flag should not parse");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn a_path_that_looks_like_a_flag_works_after_a_separator() {
        let cli = parse(&["--", "--version"]).unwrap();
        assert_eq!(cli.media, Some(PathBuf::from("--version")));
    }
}
