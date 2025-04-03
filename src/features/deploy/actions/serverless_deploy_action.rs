use std::{
    io::{self, BufRead},
    process::{Command, Stdio},
};

use crate::core::AppError;

pub trait IServerlessDeploy {
    fn execute(&self) -> Result<(), AppError>;
}

pub struct ServerlessDeploy;

impl ServerlessDeploy {
    pub fn new() -> Self {
        Self {}
    }
}

impl IServerlessDeploy for ServerlessDeploy {
    fn execute(&self) -> Result<(), AppError> {
        println!("Executing serverless deploy command...");
        let mut deployment_process = Command::new("serverless")
            .arg("deploy")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdout_stream) = deployment_process.stdout.take() {
            let output_handler = std::thread::spawn(move || {
                let reader = io::BufReader::new(stdout_stream);

                reader.lines().for_each(|line| {
                    if let Ok(output_line) = line { println!("{}", output_line) }
                });
            });

            let process_status = deployment_process.wait()?;
            output_handler.join().unwrap();

            if !process_status.success() {
                return Err(AppError::CommandError(format!(
                    "Deployment command failed with exit code: {:?}",
                    process_status.code().unwrap_or(-1)
                )));
            }
        }

        println!("Project deployed successfully");
        Ok(())
    }
}
