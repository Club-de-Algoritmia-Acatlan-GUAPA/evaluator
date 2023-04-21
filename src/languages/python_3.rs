use std::process::Command;

use crate::code_executor::{CodeExecutor, LanguageExecutor};

#[derive(Default)]
pub struct Python3;
impl LanguageExecutor for CodeExecutor<Python3> {
    fn execute_command(&self) -> Command {
        Command::new("python3")
    }

    fn get_file_type() -> String {
        "py".to_string()
    }
}
