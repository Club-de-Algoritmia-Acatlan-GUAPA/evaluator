use anyhow::Result;

use std::{
    fs,
    io::Write,
    process::{Command, Output},
};

use crate::benchmark::run_and_meassure;
use crate::types::Status;

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

    fn set_id(&mut self, id: u128);

    fn directory(&mut self, directory: &str);

    fn name(&mut self, name: String);

    fn create_code_file(&self) -> Result<()>;

    fn prepare_code_env(&self) -> Result<CodeExecutorResult>;

    fn execute(&self, input: String, args: Vec<String>) -> Result<CodeExecutorResult>;
}

#[derive(Default, Debug, Clone)]
pub struct CodeExecutor<L: ?Sized> {
    pub id: u128,
    pub time_limit: i32,
    pub code: String,
    pub checker: Option<String>,
    pub file_type: String,
    pub directory: String,
    pub file_name: Option<String>,
    pub _marker: std::marker::PhantomData<L>,
}

impl<L: Default> CodeExecutor<L>
where
    Self: LanguageExecutor,
{
    pub fn new() -> Self {
        CodeExecutor {
            file_type: Self::get_file_type(),
            directory: "playground".to_string(),
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

    fn set_id(&mut self, id: u128) {
        self.id = id;
    }

    fn name(&mut self, name: String) {
        self.file_name = Some(name.clone());
    }

    fn directory(&mut self, directory: &str) {
        self.directory = directory.to_string();
    }
    fn create_code_file(&self) -> Result<()> {
        let file_name = if let Some(file_name) = self.file_name.as_ref() {
            file_name.clone()
        } else {
            self.id.to_string()
        };
        let file_name = format!("./{}/{}.{}", self.directory, file_name, self.file_type);
        let mut file = fs::File::create(file_name)?;
        file.write_all(self.code.as_bytes())?;
        Ok(())
    }

    fn prepare_code_env(&self) -> Result<CodeExecutorResult> {
        self.create_code_file()?;
        self.prepare()
    }

    fn execute(&self, input: String, args: Vec<String>) -> Result<CodeExecutorResult> {
        let mut command = self.execute_command();
        //println!("{} {} {:?}",
        //    format!("./{}", self.directory),
        //    format!("{}.{}", &self.id, &self.file_type),
        //    args
        //);


        run_and_meassure(
            command
                .current_dir(format!("./{}", self.directory))
                .arg(format!("{}.{}", &self.id, &self.file_type))
                .args(args),
            input.clone(),
        )
    }
}
