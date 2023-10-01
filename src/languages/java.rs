use std::process::{Command, Stdio};

use anyhow::Result;
use primitypes::{contest::Language, status::Status};

use crate::{
    code_executor::{CodeExecutor, CodeExecutorError, CodeExecutorResult, LanguageExecutor},
    command::JailedCommand,
};

#[derive(Default, Clone)]
pub struct Java;

impl LanguageExecutor for CodeExecutor<Java>
where
    Self: Send + Sync,
{
    fn prepare(&self) -> Result<CodeExecutorResult, CodeExecutorError> {
        let mut command = Command::new("/usr/bin/javac");

        // create executable
        let file_name = format!("Main.{}", Self::get_file_type());

        let child = command
            .current_dir(format!("./{}/{}", self.directory, self.id))
            .args(vec![&file_name])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let status_result = child.status.success();
        println!("{:?}", child);
        if status_result {
            Ok(CodeExecutorResult {
                status: Some(child.status),
                output: Some(child),
                ..Default::default()
            })
        } else {
            Err(CodeExecutorError::InternalError(
                crate::code_executor::CodeExecutorInternalError {
                    status: Status::CompilationError,
                    output: Some(child),
                    ..Default::default()
                },
            ))
        }
    }

    fn execute_command(&self) -> std::process::Command {
        let mut c = Command::new("/usr/bin/java");
        c.arg("Main");
        c
    }

    fn nsjail_execute_command(&self) -> JailedCommand {
        JailedCommand::new("/usr/bin/java".to_string())
            .arg("-cp")
            .arg(&format!("/playground/{}/Main", self.id))
            .arg("Main")
    }

    fn get_file_type() -> String {
        "java".to_string()
    }
    fn is_compiled() -> bool {
        true
    }
    fn language() -> Language {
        Language::Java
    }
}
