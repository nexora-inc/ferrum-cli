#[derive(Debug)]
pub enum AppError {
  IoError(std::io::Error),
  ZipError(zip::result::ZipError),
  CommandError(String),
  Error(String),
}

impl From<std::io::Error> for AppError {
  fn from(error: std::io::Error) -> Self {
    AppError::IoError(error)
  }
}

impl From<zip::result::ZipError> for AppError {
  fn from(error: zip::result::ZipError) -> Self {
    AppError::ZipError(error)
  }
}
