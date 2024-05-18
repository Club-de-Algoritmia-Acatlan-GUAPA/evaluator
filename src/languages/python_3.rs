use std::process::Command;

use primitypes::contest::Language;

use crate::{
    code_executor::{CodeExecutor, LanguageExecutor},
    command::JailedCommand,
    consts::LANGUAGE,
};

#[derive(Default)]
pub struct Python3;
impl LanguageExecutor for CodeExecutor<Python3> {
    fn execute_command(&self) -> Command {
        let python3 = LANGUAGE.get(&Language::Python3).expect("Python3 now working");
        Command::new(python3.path.as_str())
    }

    fn get_file_type() -> String {
        "py".to_string()
    }

    fn nsjail_execute_command(&self) -> JailedCommand {
        let python3 = LANGUAGE.get(&Language::Python3).expect("Python3 now working");
        JailedCommand::new(python3.path.clone()).arg(&format!(
            "{0}/{1}/{1}.{2}",
            self.resources, self.id, self.file_type
        ))
    }

    fn is_compiled() -> bool {
        false
    }

    fn language() -> Language {
        Language::Python3
    }
}
