use thiserror::Error;

use crate::ffi::{
    TIX_SWITCH_ALREADY_ON_PROJECT, TIX_SWITCH_PROJECT_ALREADY_EXISTS, TIX_SWITCH_PROJECT_NOT_FOUND,
    TixError, tix_switch_project,
};

#[derive(Debug, Error)]
pub enum SwitchError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

#[derive(Debug, strum::Display)]
pub enum SwitchResult {
    #[strum(to_string = "Switched to project {0}")]
    Switched(String),
    #[strum(to_string = "Created and switched to new project {0}")]
    Created(String),
}

pub fn switch(project: &str, create: bool) -> Result<SwitchResult, SwitchError> {
    let project_ptr = std::ffi::CString::new(project).map_err(|_| SwitchError::InvalidArgument)?;
    let create_flag = if create { 1 } else { 0 };
    let result = unsafe { tix_switch_project(project_ptr.as_ptr(), create_flag) };

    match result {
        0 => Ok(SwitchResult::Switched(project.to_string())),
        1 => Ok(SwitchResult::Created(project.to_string())),
        TIX_SWITCH_PROJECT_NOT_FOUND => Err(SwitchError::TixError(TixError::SwitchProjectNotFound(project.to_string()))),
        TIX_SWITCH_PROJECT_ALREADY_EXISTS => {
            Err(SwitchError::TixError(TixError::SwitchProjectAlreadyExists(project.to_string())))
        }
        TIX_SWITCH_ALREADY_ON_PROJECT => {
            Err(SwitchError::TixError(TixError::SwitchAlreadyOnProject(project.to_string())))
        }
        _ => Err(SwitchError::TixError(TixError::UnknownError)),
    }
}
