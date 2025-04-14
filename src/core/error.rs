/// Defines the specific category of an `Error`.
#[derive(Debug)]
pub enum ErrorKind {
  /// Error indicating the `ferrum.json` configuration file was not found.
  MissingFerrumConfig,

  /// Error reading a file from the file system.
  FileRead,

  /// Error during JSON parsing.
  JsonDeserialization,

  /// Profile specified via arguments was not found in the configuration.
  ProfileNotFound,

  /// Represents an unexpected or unhandled error condition.
  Unhandled,
}

/// Represents an application-specific error.
///
/// Contains details about the error kind and a descriptive message.
/// Fields are private to ensure controlled construction via associated
/// functions like `new`.
#[derive(Debug)]
pub struct Error {
  /// The specific category of error that occurred.
  kind: ErrorKind,

  /// A detailed message describing the error, suitable for display.
  message: String,
}

impl Error {
  /// Creates a new `Error` instance with the specified kind and message.
  ///
  /// # Arguments
  ///
  /// * `kind` - The `ErrorKind` categorizing this error.
  /// * `message` - A string slice containing the descriptive error message.
  pub fn new(kind: ErrorKind, message: &str) -> Self {
    Self { kind, message: message.to_string() }
  }

  /// Returns a reference to the specific `ErrorKind` for this error.
  pub fn kind(&self) -> &ErrorKind {
    &self.kind
  }

  /// Returns a string slice of the descriptive error message.
  pub fn message(&self) -> &str {
    &self.message
  }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
      Self {
        kind: ErrorKind::FileRead,
        message: error.to_string(),
      }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
      Self {
        kind: ErrorKind::JsonDeserialization,
        message: error.to_string(),
      }
    }
}
