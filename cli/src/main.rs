use clap::{Parser, Subcommand};
use garmin_gym_fit_parser::session::SessionData;
use rustbreak::FileDatabase;
use rustbreak::deser::Ron;
use std::{fs::File, path::PathBuf};

use crate::config::Config;

mod config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a data repository
    Init {
        /// Fit File containing the first register in the DataBase
        fit_file: PathBuf,
    },
    /// Checks if fit file is parsable
    Add {
        /// Fit file
        fit_file: PathBuf,
    },
    /// Checks if fit file is parsable
    Check {
        /// Fit file
        fit_file: PathBuf,
    },
    /// Lists all sessions in data base
    List,
}

fn main() {
    let cli = Cli::parse();

    let config: Config = confy::load("mucuroso", "config").unwrap();

    match &cli.command {
        Commands::Init { fit_file } => {
            let mut fp = File::open(fit_file).unwrap();
            let db = FileDatabase::<Vec<SessionData>, Ron>::create_at_path(
                config.db_file,
                vec![SessionData::try_from_reader(&mut fp).unwrap()],
            )
            .unwrap();
            db.save().unwrap();
        }
        Commands::Add { fit_file } => {
            let mut fp = File::open(fit_file).unwrap();
            let session_data = SessionData::try_from_reader(&mut fp).unwrap();

            let db = FileDatabase::<Vec<SessionData>, Ron>::load_from_path(config.db_file).unwrap();
            db.write(|db| {
                db.push(session_data);
                db.dedup();
                db.sort();
            })
            .unwrap();
            db.save().unwrap();
        }
        Commands::Check { fit_file } => {
            let mut fp = File::open(fit_file).unwrap();
            match SessionData::try_from_reader(&mut fp) {
                Ok(_) => {
                    println!("File {:#?} is parsable", fit_file);
                    std::process::exit(exitcode::OK);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::List => {
            let db = FileDatabase::<Vec<SessionData>, Ron>::load_from_path(config.db_file).unwrap();
            let sessions = db.read(|db| db.clone()).unwrap();
            for (i, session) in sessions.into_iter().enumerate() {
                println!(
                    "Session {}: {}",
                    i + 1,
                    session.timestamp().format("%a %b %e %T %Y")
                )
            }
        }
    }
}
