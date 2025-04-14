use std::{fs::File, io::BufReader, path::Path};

use serde::de::DeserializeOwned;

use crate::core::Result;

/// Provides functionalities for file operations.
pub trait FileUtilProvider {
  /// Reads a file from the given path and deserializes its JSON content.
  ///
  /// Accepts any type that can be converted into a `Path` reference.
  /// The target type `T` must implement `DeserializeOwned`.
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T> where T: DeserializeOwned;
}

/// A concrete implementation for file operations.
pub struct FileUtil;

impl FileUtil {
  /// Creates a new `FileUtil` instance.
  pub fn new() -> Self { Self {} }
}

impl FileUtilProvider for FileUtil {
  /// Reads a JSON file and deserializes it using buffered reading.
  ///
  /// This approach avoids reading the entire file into a string first,
  /// potentially saving memory and improving performance for large files.
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T>
  where T: DeserializeOwned {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
  }
}
