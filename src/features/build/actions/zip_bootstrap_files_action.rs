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
    let directory_entries = fs::read_dir("target/lambda")?;

    for entry_result in directory_entries {
      let directory_entry = entry_result?;
      let directory_path = directory_entry.path();

      if directory_path.is_dir() {
        let bootstrap_file_path = directory_path.join("bootstrap");

        if bootstrap_file_path.exists() {
          let zip_file_path = directory_path.join("bootstrap.zip");

          let mut bootstrap_file = fs::File::open(&bootstrap_file_path)?;
          let zip_file = fs::File::create(&zip_file_path)?;
          let mut zip_writer = ZipWriter::new(zip_file);

          zip_writer.start_file("bootstrap", SimpleFileOptions::default())?;

          let mut buffer = Vec::new();
          io::copy(&mut bootstrap_file, &mut buffer)?;
          zip_writer.write_all(&buffer)?;

          zip_writer.finish()?;
          println!(r#"Artifact created. Location: "{:?}""#, zip_file_path);
        }
      }
    }

    Ok(())
  }
}
