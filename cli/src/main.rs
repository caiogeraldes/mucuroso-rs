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
    Init,
    /// Checks if fit file is parsable
    Add {
        /// Fit files
        fit_files: Vec<PathBuf>,
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
        Commands::Init => {
            let db = FileDatabase::<Vec<SessionData>, Ron>::create_at_path(config.db_file, vec![])
                .unwrap();
            db.save().unwrap();
        }
        Commands::Add { fit_files } => {
            for fit_file in fit_files {
                if config.keep_fit {
                    let mut path: PathBuf = config.db_file.parent().unwrap().into();
                    path.push("fit/");
                    let _ = std::fs::create_dir(&path);
                    path.push(fit_file.file_name().unwrap());
                    std::fs::copy(fit_file, path).unwrap();
                }
                let mut fp = File::open(fit_file).unwrap();
                let session_data = SessionData::try_from_reader(&mut fp).unwrap();

                let db =
                    FileDatabase::<Vec<SessionData>, Ron>::load_from_path(&config.db_file).unwrap();
                db.write(|db| {
                    db.push(session_data);
                    db.dedup();
                    db.sort();
                })
                .unwrap();
                db.save().unwrap();
            }
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
            if !sessions.is_empty() {
                for (i, session) in sessions.into_iter().enumerate() {
                    println!(
                        "Session {}: {}",
                        i + 1,
                        session.timestamp().format("%a %b %e %T %Y")
                    )
                }
            } else {
                println!("DB is empty.")
            }
        }
    }
}
