use ftp::FtpStream;
use regex::Regex;
use std::{fs, path};
use std::process;
use clap::Parser;
use tempfile;

/// Enum with all possible source types
enum SourceType {
    FILE,
    FOLDER,
    FTP,
    NONE,
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Source Path: can be file path to file, folder or ftp addres
    pub(crate) source: path::PathBuf,

    /// Username: only needed when ftp auth
    #[arg(short, long)]
    pub(crate) user: Option<String>,

    /// Password: only needed when ftp auth
    #[arg(short, long)]
    pub(crate) password: Option<String>,

    /// Destination: where files get written to
    pub(crate) dest: path::PathBuf,
}

/// Match given path to source types.
///
fn match_src(pfad: &std::path::Path) -> SourceType {
    return if pfad.is_file() {
        SourceType::FILE
    } else if pfad.is_dir() {
        SourceType::FOLDER
    } else if pfad.starts_with("ftp:") {
        SourceType::FTP
    } else {
        SourceType::NONE
    };
}

/// Get all files from given source and copy into temporary dictionary
///
/// All files which match `JDSave_[0-9]+` are selected.
/// These files get copied into a temporary directory.
/// The function then return the path to temp directory.
///
pub fn get_files(pfad: std::path::PathBuf) -> tempfile::TempDir {
    let re = Regex::new(r"JDSave_[0-9]+").unwrap();
    let tmpdir = tempfile::tempdir().expect("Could not create tmp dir");

    match match_src(pfad.as_path()) {
        SourceType::FILE => {
            let file = tmpdir.path().join("file_1");
            println!("found file {}", file.display());
            fs::copy(pfad, file).expect("failed to create tmp file");
        }

        SourceType::FOLDER => {
            let mut i: u16 = 0;
            for entry in pfad.read_dir().expect("failed read_dir") {
                if let Ok(entry) = entry {
                    // check if file has valid file song_name
                    println!("found file {:?}", entry.path());
                    if !re.is_match(entry.file_name().to_str().unwrap()) {
                        println!("skipped file {:?}", entry.file_name());
                        continue;
                    }

                    let file = tmpdir.path().join(format!("file_{}", i));
                    fs::copy(entry.path(), file).expect("failed to create tmp file");
                }
                i += 1;
            }
        }

        SourceType::FTP => {
            let ftp_stream = FtpStream::connect(format!("{}", pfad.to_str().unwrap()))
                .expect("Couldn't connect to the server...");
            
            // TODO
        }

        SourceType::NONE => {
            println!("Could not resolve given path");
            process::exit(1);
        }
    }

    return tmpdir;
}
