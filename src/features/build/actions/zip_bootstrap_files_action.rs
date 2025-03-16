use std::{io::Write, fs, io};
use zip::{write::SimpleFileOptions, ZipWriter};

use crate::core::AppError;

pub trait IZipBootstrapFiles {
  fn execute(&self) -> Result<(), AppError>;
}

pub struct ZipBootstrapFiles;

impl ZipBootstrapFiles {
  pub fn new() -> Self {
    Self {}
  }
}

impl IZipBootstrapFiles for ZipBootstrapFiles {
  fn execute(&self) -> Result<(), AppError> {
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
        let bootstrap_path = entry_path.join("bootstrap");

        if bootstrap_path.exists() {
          let zip_path = entry_path.join("bootstrap.zip");
          let bootstrap_file_result = fs::File::open(&bootstrap_path);

          if let Err(error) = bootstrap_file_result {
            return Err(AppError::Error(error.to_string()));
          }

          let mut bootstrap_file = bootstrap_file_result.unwrap();
          let zip_file_result = fs::File::create(&zip_path);

          if let Err(error) = zip_file_result {
            return Err(AppError::Error(error.to_string()));
          }

          let zip_file = zip_file_result.unwrap();
          let mut zip_writer = ZipWriter::new(zip_file);

          let start_file_result = zip_writer.start_file(
            "bootstrap",
            SimpleFileOptions::default(),
          );

          if let Err(error) = start_file_result {
            return Err(AppError::Error(error.to_string()))
          }

          let mut buffer = Vec::new();
          let copy_result = io::copy(&mut bootstrap_file, &mut buffer);

          if let Err(error) = copy_result {
            return Err(AppError::Error(error.to_string()))
          }

          let write_result = zip_writer.write_all(&buffer);

          if let Err(error) = write_result {
            return Err(AppError::Error(error.to_string()))
          }

          let zip_finish_result = zip_writer.finish();

          if let Err(error) = zip_finish_result {
            return Err(AppError::Error(error.to_string()))
          }

          println!(r#"Artifact created. Here's the location "{:?}""#, zip_path);
        }
      }
    }

    Ok(())
  }
}
