#![expect(clippy::as_conversions)]
#![expect(unused)]
#![allow(clippy::missing_const_for_fn)]
use crate::eyre::eyre;
use clap::Parser;
use color_eyre::Section;
use color_eyre::eyre::{self, Result};
use colored::{ColoredString, Colorize};
use command_run::{Command, Error, Output};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::env::args;
use std::{ffi::OsStr, fs, sync::mpsc::Receiver, time::Duration};
use std::{path::Path, sync::mpsc};

static ERRFILE: &str = ".checkpoint.error";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Filename extension to watch
    #[arg(short, long, value_name = "filetype")]
    filetype: String,
    name: Vec<String>,
}

/// State diagram:
/// ```mermaid
/// flowchart LR
/// PASSING-->|fail|FAILING
/// FAILING-->|pass; git commit|PASSING
/// ```
/// Other transitions are no-ops (such as tests passing while in passing state)
#[derive(Debug, Copy, Clone)]
struct SavePoint<'a> {
    program: &'a str,
    args: &'a [String],
    state: State,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Passing,
    Failing,
}
#[allow(clippy::enum_glob_use)]
use State::*;

impl<'a> SavePoint<'a> {
    /// If error file exists, failing, if not, passing
    fn new(program: &'a str, args: &'a [String]) -> Self {
        let state = match fs::exists(ERRFILE) {
            Ok(_) => Passing,
            Err(_) => Failing,
        };
        Self {
            program,
            args,
            state,
        }
    }

    /// main state dispatcher
    fn test(mut self) -> Result<Self> {
        let res = cmdr(self.program, self.args);

        match (&self, res) {
            // noop
            (Self { state: Passing, .. }, Ok(_)) | (Self { state: Failing, .. }, Err(_)) => {
                Ok(self)
            }
            // notify, transition to failing
            (Self { state: Passing, .. }, Err(_)) => Ok(self.fail()),
            // notify, git commit
            (Self { state: Failing, .. }, Ok(_)) => self.pass(),
        }
    }

    /// fixed all errors, git commit
    fn pass(self) -> Result<Self> {
        log(&"Autosaving!".green().bold());
        commit("SAVEPOINT REACHED!")?;
        rm_errfile()?;
        Ok(Self {
            state: Passing,
            ..self
        })
    }

    /// test just failed
    fn fail(self) -> Self {
        log(&"Error!".red().bold());
        let _ = create_errfile();
        Self {
            state: Failing,
            ..self
        }
    }
}

/// Clear ansi terminal and put cursor at top-left
fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn log(message: &ColoredString) {
    let prefix = "🏁 CHECKPOINT: ".blue().bold();
    print!("{prefix}");
    println!("{message}");
}

#[expect(clippy::result_large_err)]
fn cmdr(program: &str, args: &[String]) -> Result<Output, Error> {
    log(&"Running command...".white().bold());
    let mut command = Command::with_args(program, args);
    command.log_command = false;
    command.run()
}
fn main() -> Result<()> {
    // INFO: Setup
    color_eyre::install()?;
    let cli = Cli::parse();
    let extension = cli.filetype;
    let program = cli
        .name
        .first()
        .ok_or_else(|| eyre!("Missing argument: COMMAND"))?;
    let args = cli.name.get(1..).ok_or_else(|| eyre!("no program arg"))?;

    //INFO: File Watcher
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    let mut machine = SavePoint::new(program, args);
    //INFO: Main UI Loop
    loop {
        machine = machine.test()?;
        log(&"Monitoring...".white().bold());
        blockforfile(&rx, &extension);
        clear();
    }
}
fn blockforfile(rx: &Receiver<Result<Event, notify::Error>>, extension: &str) {
    loop {
        match rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(Ok(Event {
                kind: EventKind::Modify(_),
                paths,
                ..
            })) if paths.first().map(|p| p.extension()) == Some(Some(OsStr::new(extension))) => {
                break;
            }
            _ => {
                // ignoring
            }
        }
    }
    while rx.recv_timeout(Duration::from_millis(100)).is_ok() {
        // DRAIN THE CHANNEL
    }
}

fn commit(msg: &str) -> Result<()> {
    let mut command = Command::with_args("git", ["commit", "-am", msg]);
    command.log_command = false;
    if command.run().is_ok() {
        Ok(())
    } else {
        log(&"Fatal error!".red().bold());
        Err(eyre!("Git command error.")
            .with_suggestion(|| "Consider manually removing the `.checkpoint.error` file"))
    }
}

fn create_errfile() -> Result<()> {
    let mut command = Command::with_args("touch", [ERRFILE]);
    command.log_command = false;
    command.run()?;
    Ok(())
}
fn rm_errfile() -> Result<()> {
    let mut command = Command::with_args("rm", [ERRFILE]);
    command.log_command = false;
    command.run()?;
    Ok(())
}
