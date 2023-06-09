use anyhow::Result;

use std::{
    fs,
    io::Write,
    process::{Command, Output},
};

use crate::benchmark::run_and_meassure;
use crate::types::{Status, TestCase};

#[derive(Debug, Clone)]
pub struct CodeExecutorResult {
    pub err: Option<Status>,
    pub output: Option<Output>,
}
pub trait LanguageExecutor: Send + Sync {
    fn prepare(&self) -> Result<CodeExecutorResult> {
        Ok(CodeExecutorResult {
            err: None,
            output: None,
        })
    }
    fn execute_command(&self) -> Command;
    fn get_file_type() -> String;
}

pub trait CodeExecutorImpl: Send + Sync {
    fn code(&mut self, code: String);

    fn set_id(&mut self, id: i32);

    fn create_code_file(&self) -> Result<()>;

    fn prepare_code_env(&self) -> Result<CodeExecutorResult>;

    fn execute(&self, test_case: &TestCase) -> Result<CodeExecutorResult>;
}

#[derive(Default, Debug)]
pub struct CodeExecutor<L: ?Sized> {
    pub id: i32,
    pub time_limit: i32,
    pub code: String,
    pub checker: Option<String>,
    pub file_type: String,
    pub _marker: std::marker::PhantomData<L>,
}

impl<L: Default> CodeExecutor<L>
where
    Self: LanguageExecutor,
{
    pub fn new() -> Self {
        CodeExecutor {
            file_type: Self::get_file_type(),
            ..Default::default()
        }
    }
}

impl<L: Default> CodeExecutorImpl for CodeExecutor<L>
where
    Self: LanguageExecutor,
{
    fn code(&mut self, code: String) {
        self.code = code;
    }

    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    fn create_code_file(&self) -> Result<()> {
        let mut file = fs::File::create(format!("./playground/{}.{}", self.id, self.file_type))?;
        file.write_all(self.code.as_bytes())?;
        Ok(())
    }

    fn prepare_code_env(&self) -> Result<CodeExecutorResult> {
        self.create_code_file()?;
        self.prepare()
    }

    fn execute(&self, test_case: &TestCase) -> Result<CodeExecutorResult> {
        let mut command = self.execute_command();
        run_and_meassure(
            command
                .current_dir("./playground")
                .arg(format!("{}.{}", &self.id, &self.file_type)),
            test_case,
        )
    }
}
