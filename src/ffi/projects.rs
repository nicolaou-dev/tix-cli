use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_projects, tix_projects_free};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectsError {
    #[error(transparent)]
    TixError(#[from] TixError),
}

pub fn projects() -> Result<Vec<String>, ProjectsError> {
    let mut output_ptr = std::ptr::null_mut();
    let mut count = 0usize;

    let result = unsafe { tix_projects(&mut output_ptr, &mut count) };

    match result {
        0 => {
            if output_ptr.is_null() || count == 0 {
                return Ok(Vec::new());
            }

            let mut projects = Vec::new();

            let array = unsafe { std::slice::from_raw_parts(output_ptr, count) };
            
            for i in 0..count {
                let c_str = unsafe { std::ffi::CStr::from_ptr(array[i]) };
                projects.push(c_str.to_string_lossy().to_string());
            }

            unsafe { tix_projects_free(output_ptr, count) };
            
            Ok(projects)
        }
        TIX_NOT_A_REPOSITORY => Err(ProjectsError::TixError(TixError::NotARepository)),
        TIX_COMMAND_FAILED => Err(ProjectsError::TixError(TixError::CommandFailed)),
        _ => Err(ProjectsError::TixError(TixError::UnknownError)),
    }
}