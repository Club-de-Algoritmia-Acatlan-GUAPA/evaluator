use std::process::Command;

use crate::{
    code_executor::{CodeExecutor, CodeExecutorImpl, Execution, LanguageExecutor2},
    command::JailedCommand,
};
#[derive(Default, Clone)]
pub struct Interpreted;
impl Execution for CodeExecutor<Interpreted> {}
impl LanguageExecutor2 for CodeExecutor<Interpreted> {
    fn nsjail_execute_command(&self) -> JailedCommand {
        let vec: Vec<_> = self.executable.args.iter().map(|s| s.as_str()).collect();
        let args = self.parse_args(&vec);
        let ref_args: Vec<_> = args.iter().map(|s| (*s).as_str()).collect();
        JailedCommand::new(self.executable.path.clone()).args(ref_args.as_slice())
    }

    fn execute_command(&self) -> Command {
        let vec: Vec<_> = self.executable.args.iter().map(|s| s.as_str()).collect();
        let args = self.parse_args(&vec);
        let mut c = Command::new(self.executable.path.as_str());
        c.args(&args);
        c
    }
}
