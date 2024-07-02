use std::process::Command;

use primitypes::contest::Language;

use crate::{
    code_executor::{CodeExecutor, Execution, LanguageExecutor2},
    command::JailedCommand,
};
#[derive(Default, Clone)]
pub struct Interpreted;
impl Execution for CodeExecutor<Interpreted> {}
impl LanguageExecutor2 for CodeExecutor<Interpreted> {
    fn nsjail_execute_command(&self) -> JailedCommand {
        let args: Vec<_> = self.executable.args.iter().map(|s| s.as_str()).collect();
        JailedCommand::new(self.executable.path.clone()).args(&args)
    }

    fn execute_command(&self) -> Command {
        let args: Vec<_> = self.executable.args.iter().map(|s| s.as_str()).collect();
        let mut c = Command::new(self.executable.path.clone());
        c.args(&args);
        c
    }
}
