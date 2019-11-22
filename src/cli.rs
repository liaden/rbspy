#[macro_use]
use clap::Clap;
use crate::core::types::pid_t;
use std::path::PathBuf;
use std::str::{FromStr};


/// The kinds of things we can call `rbspy record` on.
#[derive(Clap, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Target2 {
    Pid {
        /// Pid of the Ruby process you want to profile.
        #[clap(short="p", long="pid")]
        pid: Vec<pid_t>
    },
    Subprocess {
        prog: String,
        args: Vec<String>
    },
}

#[derive(Clap, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum OutputFormat {
    Flamegraph,
    Callgrind,
    Speedscope,
    Summary,
    SummaryByLine,
}

use OutputFormat::*;

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(format: &str) -> Result<Self, String> {
        match format {
            "flamegraph"      => Ok(Flamegraph),
            "callgrind"       => Ok(Callgrind),
            "speedscope"      => Ok(Speedscope),
            "summary"         => Ok(Summary),
            "summary_by_line" => Ok(SummaryByLine),
            _                 => Err(format!("Unknown format {}", format))
        }
    }
}

#[derive(Clap, Debug, Clone, Eq, PartialEq, Ord, PartialOrd,  Hash)]
#[clap(name = "rbspy")]
pub enum Opt {
    #[clap(name = "snapshot")]
    Snapshot(Snapshot),

    // #[clap(name = "record")]
    Record(Record),

    // #[clap(name = "report")]
    Report(Report),
}

fn parse_duration(value: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    match value.parse::<u64>() {
        Err(e) => Err(e),
        Ok(integer_duration) => Ok(std::time::Duration::from_secs(integer_duration)),
    }
}

#[derive(Clap, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Record {
    #[clap(flatten)]
    target: Target2,

    #[clap(name="")]
    out_path: PathBuf,
    raw_path: PathBuf,

    #[clap(short="r", long="rate")]
    sample_rate: u32,

    #[clap(name="duration", parse(try_from_str="parse_duration"))]
    maybe_duration: Option<std::time::Duration>,

    #[clap(long="format")]
    format: OutputFormat,

    /// Don't drop root privileges when running a Ruby program as a subprocess
    #[clap(long="no-drop-sudo")]
    no_drop_root: bool,

    /// Record all subprocesses of the given PID or command
    #[clap(short="s", long="sub-processes")]
    with_subprocesses: bool,

    /// Don't print the summary profiling data every second
    #[clap(long="silent")]
    silent: bool
}

#[derive(Clap, Debug, Clone, Eq, PartialEq, Ord, PartialOrd,  Hash)]
pub struct Snapshot {
    /// Pid of the Ruby process you want to profile.
    #[clap(short="p", long="pid")]
    pid: pid_t
}

#[derive(Clap, Debug, Clone, Eq, PartialEq, Ord, PartialOrd,  Hash)]
pub struct Report {
    #[clap(long="format")]
    format: OutputFormat,

    #[clap(long="input")]
    input: PathBuf,

    #[clap(long="output")]
    output: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli() {
        assert_eq!(Opt::Snapshot(Snapshot { pid: 5 }), Opt::parse_from(&["rbspy", "snapshot", "--pid", "5"]));
        assert_eq!(Opt::Snapshot(Snapshot { pid: 5 }), Opt::parse_from(&["rbspy", "snapshot", "-p", "5"]));
    }

}
