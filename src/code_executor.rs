use anyhow::Result;

use std::{
    fs,
    io::Write,
    process::{Command, Output},
};

use crate::benchmark::run_and_meassure;
use crate::types::{Status, TestCase};

#[derive(Default, Clone)]
pub struct ExecutorMetadata {
    pub time_limit: i32,
    pub id: i32,
    pub code: String,
    pub checker: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodeExecutorResult<L : LanguageExecutor> {
    pub err: Option<Status>,
    pub output: Option<Output>,
}
pub trait LanguageExecutor: Send + Sync {
    fn new_lang(id: i32) -> Self
    where
        Self: Sized;
    fn prepare(&self) -> Result<CodeExecutorResult>;
    fn execute_command(&self) -> Command;
    fn get_file_type(&self) -> String;
}

// #[derive(Default)]
// pub struct CodeExecutor<'a> {
//     pub lang: &'a dyn LanguageExecutor,
//     pub metadata: ExecutorMetadata,
// }
pub struct CodeExecutor {
    pub lang: Box<dyn LanguageExecutor>,
    pub metadata: ExecutorMetadata,
}
impl CodeExecutor {
    pub fn new(lang: impl LanguageExecutor + 'static, id: i32) -> Self {
        let metadata = ExecutorMetadata {
            id,
            time_limit: 1,
            ..ExecutorMetadata::default()
        };
        CodeExecutor {
            metadata,
            lang: Box::new(lang),
        }
    }
    pub fn code(&mut self, code: String) {
        self.metadata.code = code;
    }

    fn create_code_file(&self) -> Result<()> {
        let mut file = fs::File::create(format!(
            "./playground/{}.{}",
            self.metadata.id,
            self.lang.get_file_type()
        ))?;
        file.write_all(self.metadata.code.as_bytes())?;
        Ok(())
    }

    pub fn prepare_code_env(&self) -> Result<CodeExecutorResult> {
        self.create_code_file()?;
        // self.create_testlib_checker_file()?;
        self.lang.prepare()
    }
    pub fn execute(&self, test_case: &TestCase) -> Result<CodeExecutorResult> {
        let command = self.lang.execute_command();

        run_and_meassure(command, test_case)
    }
}
