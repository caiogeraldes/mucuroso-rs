use crate::{config::Config, utils::CsvSet};
use clap::{Parser, Subcommand};
use mucuroso::garmin::garmin_session::GarminSessionData;
use mucuroso::session::SessionData;
use rustbreak::FileDatabase;
use rustbreak::deser::Ron;
use std::{fs::File, path::PathBuf};

mod config;
mod utils;

#[allow(unused_imports)]
use mucuroso::constants::GARMIN_EXERCISES;

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
    /// Get by session number
    Get {
        session_number: usize,
        #[command(subcommand)]
        format: Option<Format>,
    },
}

#[derive(Subcommand, Clone)]
enum Format {
    Stdout,
    Json { output_file: Option<String> },
    Csv { output_file: Option<String> },
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
                let garmin_session_data = GarminSessionData::try_from_reader(&mut fp).unwrap();
                let session_data =
                    mucuroso::session::SessionData::try_from(garmin_session_data).unwrap();

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
            match GarminSessionData::try_from_reader(&mut fp) {
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
                println!("Session\tDate\t\t\t\tWeight");
                for (i, session) in sessions.into_iter().enumerate() {
                    println!(
                        "{}\t{}\t",
                        i + 1,
                        session.timestamp.format("%a %b %e %T %Y"),
                        // session.total_weight()
                    )
                }
            } else {
                println!("DB is empty.")
            }
        }
        Commands::Get {
            session_number,
            format,
        } => {
            let db = FileDatabase::<Vec<SessionData>, Ron>::load_from_path(config.db_file).unwrap();
            let sessions = db.read(|db| db.clone()).unwrap();
            if let Some(session) = sessions.get(session_number - 1) {
                match format {
                    None | Some(Format::Stdout) => {
                        println!("{}", serde_json::to_string_pretty(session).unwrap());
                    }
                    Some(Format::Json { output_file }) => match output_file {
                        Some(output_file) => {
                            let output = File::create_new(output_file).unwrap();
                            serde_json::to_writer_pretty(output, session).unwrap();
                        }
                        None => {
                            eprintln!("Please provide valid name for json output file.")
                        }
                    },
                    Some(Format::Csv { output_file }) => match output_file {
                        Some(output_file) => {
                            let output = File::create_new(output_file).unwrap();
                            let mut wtr = csv::Writer::from_writer(output);
                            for set in &session.sets {
                                wtr.serialize(CsvSet::from(set)).unwrap();
                            }
                            wtr.flush().unwrap();
                        }
                        None => {
                            eprintln!("Please provide valid name for json output file.")
                        }
                    },
                }
            } else {
                eprintln!(
                    "Session number {} out of range. Max value: {}.",
                    session_number,
                    sessions.len()
                );
                std::process::exit(1);
            }
        }
    }
}
