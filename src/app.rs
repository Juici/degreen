use std::path::Path;
use std::{fmt, io, process};

use clap::{App as ClapApp, AppSettings as ClapSettings, Arg, ArgMatches, Shell};
use pkg::bin_name;

const APP_NAME: &str = "degreen";

const AFTER_HELP: &str =
    "By default degreen will only run on the files. Use the --recursive (-r) flag to run \
     recursively over directories along with their contents.";

const COMPLETIONS: &str = "completions";
const FORCE: &str = "force";
const RECURSIVE: &str = "recursive";
const VERBOSE: &str = "verbose";
const FILE: &str = "file";

fn cli() -> ClapApp<'static, 'static> {
    ClapApp::new(pkg_name!())
        .version(pkg_version!())
        .about(pkg_description!())
        .after_help(AFTER_HELP)
        .max_term_width(84)
        .setting(ClapSettings::DeriveDisplayOrder)
        .setting(ClapSettings::DisableHelpSubcommand)
        .arg(
            Arg::with_name(COMPLETIONS)
                .long("completions")
                .help("Generate completion scripts for your shell")
                .value_name("SHELL")
                .possible_values(&Shell::variants())
                .conflicts_with_all(&[FORCE, RECURSIVE, VERBOSE, FILE]),
        )
        .arg(
            Arg::with_name(FORCE)
                .short("f")
                .long("force")
                .help("Disable file prompt"),
        )
        .arg(
            Arg::with_name(RECURSIVE)
                .short("r")
                .long("recursive")
                .help("Run recursively over the contents of directories"),
        )
        .arg(
            Arg::with_name(VERBOSE)
                .short("v")
                .long("verbose")
                .help("Enable verbose output"),
        )
        .arg(
            Arg::with_name(FILE)
                .value_name("FILE")
                .min_values(1)
                .required_unless("completions"),
        )
}

pub struct App {
    matches: ArgMatches<'static>,
}

pub struct Settings<'a> {
    pub force: bool,
    pub recursive: bool,
    pub verbose: bool,
    pub files: Vec<&'a Path>,
}

impl App {
    pub fn new() -> Self {
        App {
            matches: cli().get_matches(),
        }
    }

    pub fn gen_completions(for_shell: Shell) {
        let bin_name = bin_name().unwrap_or(APP_NAME);

        // Wrapped stdout that calls `process::exit(0)` when on a broken pipe error.
        let mut stdout = WrappedStdout(io::stdout());
        cli().gen_completions_to(bin_name, for_shell, &mut stdout);
    }

    pub fn completion_shell(&self) -> Option<Shell> {
        self.matches
            .value_of("completions")
            .map(|shell| shell.parse().unwrap())
    }

    pub fn settings(&self) -> Settings {
        Settings {
            force: self.matches.is_present(FORCE),
            recursive: self.matches.is_present(RECURSIVE),
            verbose: self.matches.is_present(VERBOSE),
            files: self
                .matches
                .values_of_os(FILE)
                .map(|values| values.map(Path::new).collect())
                .unwrap_or_else(Vec::new),
        }
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
