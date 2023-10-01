use std::process::Command;

use primitypes::contest::Language;

use crate::{
    code_executor::{CodeExecutor, LanguageExecutor},
    command::JailedCommand,
};

#[derive(Default)]
pub struct Python3;
impl LanguageExecutor for CodeExecutor<Python3> {
    fn execute_command(&self) -> Command {
        Command::new("python3")
    }

    fn get_file_type() -> String {
        "py".to_string()
    }

    fn nsjail_execute_command(&self) -> JailedCommand {
        JailedCommand::new("/usr/bin/python3".to_string()).arg(&format!(
            "/playground/{0}/{0}.{1}",
            &self.id, &self.file_type
        ))
    }

    fn is_compiled() -> bool {
        false
    }

    fn language() -> Language {
        Language::Python3
    }
}
