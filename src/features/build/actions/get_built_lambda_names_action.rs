use std::fs;

use crate::core::AppError;

pub trait IGetBuiltLambdaNames {
  fn execute(&self) -> Result<Vec<String>, AppError>;
}

pub struct GetBuiltLambdaNames;

impl GetBuiltLambdaNames {
  pub fn new() -> Self {
    Self {}
  }
}

impl IGetBuiltLambdaNames for GetBuiltLambdaNames {
  fn execute(&self) -> Result<Vec<String>, AppError> {
    let mut folder_names: Vec<String> = vec![];
    let dir_entry_result = fs::read_dir("target/lambda");

    if let Err(error) = dir_entry_result {
      return Err(AppError::Error(error.to_string()))
    }

    let dir_entry = dir_entry_result.unwrap();

    for entry_result in dir_entry {
      if let Err(error) = entry_result {
        return Err(AppError::Error(error.to_string()))
      }

      let entry = entry_result.unwrap();

      let entry_path = entry.path();

      if entry_path.is_dir() {
        if let Some(folder_name) = entry_path.file_name().and_then(|name| name.to_str()) {
          folder_names.push(folder_name.to_string());
        }
      }
    }

    Ok(folder_names)
  }
}
