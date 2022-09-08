use std::io::Result;
use std::path::Path;

use clap::{crate_description, crate_name, Arg, ArgMatches, Command};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::organizer::Organizer;
use crate::{finder, utils};

const LOG_FILE: &str = "candentia.log";

pub fn parse_cli() {
    let version = env!("CARGO_PKG_VERSION");
    let args = build_cli(version);
    setup_logger().expect("Could not setup logger");
    utils::print_welcome_text(version);
    match args.subcommand() {
        Some(("organize", org_matches)) => OrganizerCli::new(org_matches).organize_scans(),
        _ => unreachable!("Invalid subcommand"),
    }
}

#[cfg(not(tarpaulin_include))]
fn build_cli(version: &str) -> ArgMatches {
    Command::new(crate_name!())
        .version(version)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(crate_description!())
        .subcommand(
            Command::new("organize")
                .about("Organize scans into voucher directories")
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                        .help("File parent directory")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output directory")
                        .default_value(".")
                        .takes_value(true),
                ),
        )
        .get_matches()
}

fn setup_logger() -> Result<()> {
    let log_dir = std::env::current_dir()?;
    let target = log_dir.join(LOG_FILE);
    let tofile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)} - {l} - {m}\n",
        )))
        .build(target)?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(tofile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .expect("Failed building log configuration");

    log4rs::init_config(config).expect("Cannot initiate log configuration");

    Ok(())
}

trait InputCli {
    fn parse_dir(&self) -> &Path;
    fn parse_output(&self) -> &Path;
}

struct OrganizerCli<'a> {
    matches: &'a ArgMatches,
}

impl InputCli for OrganizerCli<'_> {
    fn parse_dir(&self) -> &Path {
        Path::new(self.matches.value_of("dir").expect("No directory provided"))
    }

    fn parse_output(&self) -> &Path {
        Path::new(self.matches.value_of("output").expect("No output provided"))
    }
}

impl OrganizerCli<'_> {
    fn new(matches: &ArgMatches) -> OrganizerCli {
        OrganizerCli { matches }
    }

    fn organize_scans(&self) {
        let scan_paths = finder::find_scans(self.parse_dir());
        let scans = Organizer::new(&scan_paths, self.parse_output());
        scans.organize();
    }
}
