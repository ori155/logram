use std::path::PathBuf;

use crate::source::LogRecord;

#[derive(Debug)]
pub enum FilesystemEvent {
    Created { path: PathBuf },
    Writed { path: PathBuf, new_content: String },
    Removed { path: PathBuf },
    Renamed { from: PathBuf, to: PathBuf },
}

impl FilesystemEvent {
    pub fn into_record(self) -> LogRecord {
        let (title, body) = match self {
            FilesystemEvent::Created { path } => {
                let title = String::from("Filesystem");
                let body = format!("{} created", path.display());

                (title, body)
            }
            FilesystemEvent::Writed { path, new_content } => {
                let title = path.display().to_string();
                let body = new_content;

                (title, body)
            }
            FilesystemEvent::Removed { path } => {
                let title = String::from("Filesystem");
                let body = format!("{} removed", path.display());

                (title, body)
            }
            FilesystemEvent::Renamed { from, to } => {
                let title = String::from("Filesystem");
                let body = format!("{} renamed to {}", from.display(), to.display());

                (title, body)
            }
        };

        LogRecord { title, body }
    }
}
