use std::{
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};

use serde::de::DeserializeOwned;

use crate::core::Result;

/// Provides functionalities for file operations.
pub trait FileUtilProvider {
  /// Reads a file from the given path and deserializes its JSON content.
  ///
  /// Utilizes buffered reading for efficiency, especially with larger files.
  ///
  /// # Type Parameters
  ///
  /// * `T`: The target type to deserialize the JSON into. Must implement
  /// `DeserializeOwned`
  ///
  /// # Arguments
  ///
  /// * `file_path`: A type convertible to a `Path` reference, specifying the
  /// file location.
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be opened, read, or if the content is
  /// not valid JSON parsable into type `T`
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T> where T: DeserializeOwned;

  /// Reads all lines from a text file into a vector of strings.
  ///
  /// Useful for simple line-based file formats like `.env` files
  /// (without parsing).
  /// Lines are read using buffered I/O.
  ///
  /// # Arguments
  ///
  /// * `file_path`: A type convertible to a `Path` reference.
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be opened or if an I/O error occurs
  /// while reading any line.
  fn read_lines(&self, file_path: impl AsRef<Path>) -> Result<Vec<String>>;
}

/// A concrete implementation of `FileUtilProvider` using standard file
/// operations.
pub struct FileUtil;

impl FileUtil {
  /// Creates a new `FileUtil` instance.
  pub fn new() -> Self { Self {} }
}

impl FileUtilProvider for FileUtil {
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T>
  where T: DeserializeOwned {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
  }

  fn read_lines(&self, file_path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    Ok(reader.lines().collect::<io::Result<Vec<String>>>()?)
  }
}
