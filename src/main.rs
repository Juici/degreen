#[cfg(not(unix))]
compile_error!("degreen can only be compiled for unix");

extern crate clap;
#[macro_use]
extern crate pkg;

mod app;
#[macro_use]
mod error;

use std::fs::{self, File, Metadata};
use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;

use app::{App, Settings};
use error::Result;

fn run() -> Result<bool> {
    let app = App::new();

    if let Some(shell) = app.completion_shell() {
        App::gen_completions(shell);
        return Ok(true);
    }

    let settings = app.settings();

    for path in &settings.files {
        let meta = match path.metadata() {
            Ok(meta) => meta,
            Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
                return error!("'{}' is not a valid file or directory", path.display());
            }
            Err(err) => if settings.force {
                continue;
            } else {
                return Err(err.into());
            },
        };
        let file_type = meta.file_type();

        // Deny symlinks to avoid cycles.
        if file_type.is_symlink() {
            return error!("cannot degreen symlink '{}'", path.display());
        }

        let path = path.canonicalize()?;

        if file_type.is_file() {
            // Degreen single file.
            degreen_file(path, meta, &settings)?;
        } else if file_type.is_dir() {
            // Recursive.
            if !settings.recursive {
                return error!("cannot degreen directory '{}'", path.display());
            }

            degreen_dir(path, &settings)?;
        } else {
            // Something has gone wrong here.
            unreachable!();
        }
    }

    Ok(true)
}

fn degreen_dir(dir: PathBuf, settings: &Settings) -> Result<bool> {
    for entry in dir.read_dir()? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => if settings.force {
                return Ok(false);
            } else {
                return Err(err.into());
            },
        };

        let meta = entry.metadata()?;
        let file_type = meta.file_type();
        let path = dir.join(entry.path());

        // Deny symlinks to avoid cycles.
        if file_type.is_symlink() {
            return error!("cannot degreen symlink '{}'", path.display());
        }
        let path = path.canonicalize()?;

        if file_type.is_file() {
            degreen_file(path, meta, settings)?;
        } else if file_type.is_dir() {
            degreen_dir(path, settings)?;
        } else {
            // Something has gone wrong here.
            unreachable!();
        }
    }

    Ok(true)
}

fn degreen_file(file: PathBuf, meta: Metadata, settings: &Settings) -> Result<bool> {
    let mut perms = meta.permissions();
    let mut mode: u32 = perms.mode();

    let buf = {
        let mut fd = File::open(&file)?;
        let mut buf = [0; 4];
        fd.read_exact(&mut buf)?;
        buf
    };

    const ELF: [u8; 4] = [0x7F, b'E', b'L', b'F'];
    const SHEBANG: [u8; 2] = [b'#', b'!'];

    if buf == ELF {
        if settings.verbose {
            println!("'{}' looks like an ELF file", file.display());
        }
        return Ok(false);
    } else if buf[..2] == SHEBANG {
        if settings.verbose {
            println!("'{}' looks like it has a shebang line", file.display());
        }
        return Ok(false);
    }

    // TODO: prompt

    // Disable execute bits.
    mode &= !0o111;
    perms.set_mode(mode);

    fs::set_permissions(file, perms)?;

    Ok(true)
}

fn main() {
    let result = run();

    match result {
        Err(error) => {
            eprintln!("error: {}", error);
            process::exit(1);
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
