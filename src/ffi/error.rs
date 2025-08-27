use thiserror::Error;

#[derive(Debug, Error)]
pub enum TixError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Not a tix repository")]
    NotARepository,
    #[error("Command failed")]
    CommandFailed,
    #[error("File system error")]
    FileSystemError,
    #[error("Unknown error")]
    UnknownError,
    #[error("Failed to create tix workspace")]
    InitWorkspaceCreationFailed,
    #[error("Access denied")]
    InitAccessDenied,
    #[error("Invalid configuration key")]
    ConfigInvalidKey,
    #[error("Remote already exists")]
    RemoteAlreadyExists,
    #[error("Invalid remote name")]
    RemoteInvalidName,
    #[error("Project '{0}' not found")]
    SwitchProjectNotFound(String),
    #[error("Project '{0}' already exists")]
    SwitchProjectAlreadyExists(String),
    #[error("Already on project '{0}'")]
    SwitchAlreadyOnProject(String),
    #[error("Invalid priority value")]
    InvalidPriority,
    #[error("Invalid title")]
    InvalidTitle,
}
