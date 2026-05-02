use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    path::PathBuf,
};

#[derive(Debug)]
pub enum IncludeError {
    Cycle(PathBuf),
    UnterminatedInclude(usize),
    EmptyPath(usize),
    EscapesRoot(PathBuf),
    Read { path: PathBuf, source: IoError },
}

impl Display for IncludeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Cycle(path) => write!(f, "include cycle detected: {}", path.display()),
            Self::UnterminatedInclude(byte) => {
                write!(f, "unterminated include starting at byte {byte}")
            }
            Self::EmptyPath(byte) => write!(f, "empty include path at byte {byte}"),
            Self::EscapesRoot(path) => {
                write!(f, "include path escapes root: {}", path.display())
            }
            Self::Read { path, source } => {
                write!(f, "failed to read {}: {source}", path.display())
            }
        }
    }
}

impl Error for IncludeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            _ => None,
        }
    }
}
