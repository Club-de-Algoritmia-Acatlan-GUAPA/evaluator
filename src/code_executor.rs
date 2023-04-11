use anyhow::Result;

use std::fs;
use std::io::Write;
use std::process::Output;

use crate::benchmark::run_and_meassure;
use crate::types::{Status, TestCase};

pub struct ExecutorMetadata {
    pub time_limit: i32,
    pub id: i32,
    pub code: String,
}

pub trait Language {
    fn new_lang(id: i32) -> Self;
    fn prepare(&self) -> Result<CodeExecutorResult>;
    fn execute_command(&self) -> std::process::Command;
    fn get_file_type(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct CodeExecutorResult {
    pub err: Option<Status>,
    pub output: Option<Output>,
}

pub struct CodeExecutor<L: Language> {
    pub lang: L,
    pub metadata: ExecutorMetadata,
    //pub file_ending: String,
    //compilar el checker
    //compilar el archivo si es necesario
    //guardar el codigo en donde se debe
}
impl<L: Language> CodeExecutor<L> {
    pub fn new(id: i32) -> Self {
        let metadata = ExecutorMetadata {
            id,
            time_limit: 1,
            code: "".to_string(),
        };
        CodeExecutor {
            metadata,
            lang: L::new_lang(id),
        }
    }
    pub fn code(&mut self, code: String) {
        self.metadata.code = code;
    }
    pub fn prepare_code_env(&self) -> Result<CodeExecutorResult> {
        self.create_code_file()?;
        self.lang.prepare()
    }
    pub fn execute(&self, test_case: &TestCase) -> Result<CodeExecutorResult> {
        let mut command = self.lang.execute_command();
        command.current_dir("./playground");

        let execution_result = run_and_meassure(command, test_case);
        execution_result
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
}
