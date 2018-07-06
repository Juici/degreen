use std::{fmt, io, process};

use clap::{App as ClapApp, AppSettings, Arg, ArgMatches, Shell};
use pkg::bin_name;

const NAME: &str = "degreen";

const AFTER_HELP: &str =
    "By default, when run on a directory, degreen will only run on the files in the directory. Use \
    the --recursive (-r) flag to run recursively over subdirectories along with their contents.";

pub struct App<'a> {
    inner: ClapApp<'a, 'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let inner = ClapApp::new(pkg_name!())
            .version(pkg_version!())
            .about(pkg_description!())
            .after_help(AFTER_HELP)
            .max_term_width(84)
            .setting(AppSettings::DeriveDisplayOrder)
            .setting(AppSettings::DisableHelpSubcommand)
            .arg(
                Arg::with_name("completions")
                    .long("completions")
                    .help("Generate completion scripts for your shell")
                    .value_name("SHELL")
                    .possible_values(&Shell::variants()),
            )
            .arg(
                Arg::with_name("force")
                    .short("f")
                    .long("force")
                    .help("Ignore nonexistent files and never prompt"),
            )
            .arg(
                Arg::with_name("interactive")
                    .short("i")
                    .long("interactive")
                    .help("Prompt before every degreening"),
            )
            .arg(
                Arg::with_name("recursive")
                    .short("r")
                    .long("recursive")
                    .help("Run recursively over the contents of directories"),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Enable verbose output"),
            );

        App { inner }
    }

    pub fn get_matches(self) -> ArgMatches<'a> {
        self.inner.get_matches()
    }

    pub fn gen_completions(mut self, for_shell: Shell) {
        let bin_name = bin_name().unwrap_or(NAME);

        // Wrapped stdout that calls `process::exit(0)` when on a broken pipe error.
        let mut stdout = WrappedStdout(io::stdout());
        self.inner
            .gen_completions_to(bin_name, for_shell, &mut stdout);
    }
}

struct WrappedStdout(io::Stdout);

impl io::Write for WrappedStdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        assert_not_broken_pipe(self.0.lock().write(buf))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        assert_not_broken_pipe(self.0.lock().flush())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        assert_not_broken_pipe(self.0.lock().write_all(buf))
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> {
        assert_not_broken_pipe(self.0.lock().write_fmt(fmt))
    }
}

fn assert_not_broken_pipe<T>(result: Result<T, io::Error>) -> Result<T, io::Error> {
    match result {
        // Exit on a broken pipe, error should be reported by terminal.
        Err(ref error) if error.kind() == io::ErrorKind::BrokenPipe => process::exit(0),
        _ => result,
    }
}
