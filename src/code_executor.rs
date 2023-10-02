use std::{
    process::{Command, ExitStatus, Output},
    time::Duration,
};

use anyhow::Result;
use async_trait::async_trait;
use primitypes::{contest::Language, status::Status};
use tokio::{fs, io::AsyncWriteExt};
use tracing::{debug, info};

use crate::command::JailedCommand;

#[derive(Debug, Clone, Default)]
pub struct CodeExecutorResult {
    pub status: Option<ExitStatus>,
    pub output: Option<Output>,
    pub duration: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct CodeExecutorInternalError {
    pub status: Status,
    pub output: Option<Output>,
    pub duration: Duration,
}
#[derive(Debug)]
pub enum CodeExecutorError {
    InternalError(CodeExecutorInternalError),
    ExternalError(anyhow::Error),
}

impl<E> From<E> for CodeExecutorError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::ExternalError(err.into())
    }
}

pub trait LanguageExecutor: Send + Sync {
    fn prepare(&self) -> Result<CodeExecutorResult, CodeExecutorError> {
        Ok(CodeExecutorResult {
            status: None,
            ..Default::default()
        })
    }
    fn execute_command(&self) -> Command;
    fn nsjail_execute_command(&self) -> JailedCommand;
    fn is_compiled() -> bool;
    fn language() -> Language;
    fn get_file_type() -> String;
}

#[async_trait]
pub trait CodeExecutorImpl: Send + Sync {
    fn code(&mut self, code: String);

    fn set_id(&mut self, id: u128);

    fn directory(&mut self, directory: &str);

    fn name(&mut self, name: String);

    async fn create_code_file(&self) -> Result<()>;

    async fn prepare_code_env(&self) -> Result<CodeExecutorResult, CodeExecutorError>;

    async fn create_id_dir(&self) -> Result<()>;

    async fn destroy(&self) -> Result<()>;

    fn execute(
        &self,
        input_file: &str,
        output_file: &str,
    ) -> Result<CodeExecutorResult, CodeExecutorError>;

    fn execute_nsjail(
        &self,
        input_file: &str,
        output_file: &str,
    ) -> Result<CodeExecutorResult, CodeExecutorError>;
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

#[async_trait]
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
        self.file_name = Some(name)
    }

    fn directory(&mut self, directory: &str) {
        self.directory = directory.to_string();
    }

    async fn create_code_file(&self) -> Result<()> {
        let file_name = if let Some(file_name) = self.file_name.as_ref() {
            file_name.clone()
        } else {
            self.id.to_string()
        };
        let file_name = match Self::language() {
            Language::Java => format!("./playground/{}/Main.java", self.id),
            _ => format!("./playground/{}/{}.{}", self.id, file_name, self.file_type),
        };
        debug!("CREATING CODE FILE file = {}", file_name);
        let mut file = fs::File::create(file_name).await?;
        file.write_all(self.code.as_bytes()).await?;
        Ok(())
    }

    async fn create_id_dir(&self) -> Result<()> {
        let dir = format!("./playground/{}", &self.id);
        debug!("CREATING ID DIRECTORY file = {}", dir);
        match tokio::fs::create_dir(dir).await {
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => Ok(()),
                _ => return Err(e.into()),
            },
            Ok(_) => Ok(()),
        }
    }

    async fn prepare_code_env(&self) -> Result<CodeExecutorResult, CodeExecutorError> {
        info!("CREATING CODE ENV");
        self.create_id_dir().await?;
        self.create_code_file().await?;
        self.prepare()
    }

    async fn destroy(&self) -> Result<()> {
        let dir = format!("./playground/{}", self.id);
        info!("DESTROYING {}", dir);
        Ok(tokio::fs::remove_dir_all(dir).await?)
    }

    fn execute(
        &self,
        input_file: &str,
        output_file: &str,
    ) -> Result<CodeExecutorResult, CodeExecutorError> {
        let mut command = self.execute_command();
        if !Self::is_compiled() {
            command.arg(format!(
                "./playground/{0}/{0}.{1}",
                &self.id, &self.file_type
            ));
        }
        debug!("OPENING file = {}", output_file);
        let input = std::fs::File::open(input_file)?;
        debug!("CREATING file = {}", output_file);
        let output = std::fs::File::create(output_file)?;

        command.stdin(input).stdout(output);
        debug!("EXECUTING command {:?}", command);
        crate::benchmark::run_and_meassure_2(&mut command)
    }

    // TODO remove all hardcoded strings
    fn execute_nsjail(
        &self,
        input_file: &str,
        output_file: &str,
    ) -> Result<CodeExecutorResult, CodeExecutorError> {
        debug!("CREATING file = {}", output_file);
        let output = std::fs::File::create(output_file)?;
        let command = self
            .nsjail_execute_command()
            .current_dir("/app/evaluator")
            .mount("/app/evaluator/resources/", "/resources")
            .mount("/app/evaluator/playground/", "/playground")
            .config_file("/app/evaluator/resources/nsjail.cfg")
            .arg("<")
            .arg(input_file)
            .stdout(output);
        debug!("EXECUTING command {:?}", command);
        crate::benchmark::run_and_meassure(command)
    }
}
