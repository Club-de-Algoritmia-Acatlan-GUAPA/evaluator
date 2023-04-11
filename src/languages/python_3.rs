use anyhow::Result;

use std::process::Command;

// use crate::types::{CodeExecutor, CodeExecutorResult, Status, TestCase};
use crate::code_executor::{Language, CodeExecutorResult};

pub struct Python3 {
    pub file_ending: String,
    pub file_for_execution: String, // TODO : convert to OsStr
    pub id: i32,
}

impl Language for Python3 {
    fn new_lang(id: i32) -> Self {
        let file_ending = "cpp".to_string();

        Self {
            id,
            file_for_execution: format!("{}.{}", id, file_ending),
            file_ending,
        }
    }
    fn prepare(&self) -> Result<CodeExecutorResult> {
        Ok(CodeExecutorResult {
            err: None,
            output: None,
        })
    }
    fn execute_command(&self) -> Command {
        let mut command = Command::new("python3");
        command.arg(&self.file_for_execution);
        command
    }

    fn get_file_type(&self) -> String{
       self.file_ending.clone() 
    }
}
