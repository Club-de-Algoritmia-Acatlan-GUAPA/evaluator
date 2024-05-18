use std::{
    process::{Command, ExitStatus, Output},
    time::Duration,
};

use anyhow::Result;
use async_trait::async_trait;
use primitypes::{contest::Language, status::Status};
use tokio::fs;
use tracing::{debug, info};

use crate::{
    command::JailedCommand,
    configuration::{CmdStr, EvaluationType},
    consts::LANGUAGE,
};

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
pub trait Execution {}

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
    fn set_code(&mut self, code: String);

    fn set_id(&mut self, id: u128);

    async fn create_code_file(&self) -> Result<()>;

    async fn prepare_code_env(&mut self) -> Result<CodeExecutorResult, CodeExecutorError>;

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

    fn parse_args(&self, data: &Vec<&str>) -> Vec<String>;
}

#[derive(Default, Debug, Clone)]
pub struct CodeExecutor<L: ?Sized> {
    pub id: u128,
    pub time_limit: i32,
    pub code: String,
    pub checker: Option<String>,
    pub directory: String,
    pub executable: CmdStr,
    pub _marker: std::marker::PhantomData<L>,
    pub playground: String,
    pub resources: String,
    pub user_code_file: String,
    pub exec_code_file: String,
}

//impl<L: Default> CodeExecutor<L>
//where
//    Self: LanguageExecutor,
//{
//    pub fn new(playground: &str) -> Self {
//        CodeExecutor {
//            file_type: Self::get_file_type(),
//            playground: playground.to_string(),
//            executable: LANGUAGE
//                .get(&Self::language())
//                .expect("Language not available")
//                .clone(),
//            ..Default::default()
//        }
//    }
//}

//#[async_trait]
//impl<L: Default> CodeExecutorImpl for CodeExecutor<L>
//where
//    Self: LanguageExecutor,
//{
//    fn set_code(&mut self, code: String) {
//        self.code = code;
//    }
//
//    fn set_id(&mut self, id: u128) {
//        self.id = id;
//    }
//
//    fn name(&mut self, name: String) {
//        self.file_name = Some(name)
//    }
//
//    async fn create_code_file(&self) -> Result<()> {
//        let file_name = if let Some(file_name) = self.file_name.as_ref() {
//            file_name.clone()
//        } else {
//            self.id.to_string()
//        };
//        let file_name = match Self::language() {
//            Language::Java => format!("{}/{}/Main.java", self.playground,
// self.id),            _ => format!(
//                "{}/{}/{}.{}",
//                self.playground, self.id, file_name, self.file_type
//            ),
//        };
//        debug!("CREATING CODE FILE file = {}", file_name);
//        match fs::write(&file_name, self.code.as_bytes()).await {
//            Ok(()) => return Ok(()),
//            Err(e) => {
//                debug!("File : {file_name} , error: {:?}", &e);
//                return Err(e.into());
//            },
//        }
//    }
//
//    async fn create_id_dir(&self) -> Result<()> {
//        let dir = format!("{}/{}", self.playground, &self.id);
//        debug!("CREATING ID DIRECTORY file = {}", dir);
//        match tokio::fs::create_dir(&dir).await {
//            Err(e) => match e.kind() {
//                std::io::ErrorKind::AlreadyExists => Ok(()),
//                _ => {
//                    debug!("File : {} , error: {:?}", &dir, &e);
//                    return Err(e.into());
//                },
//            },
//            Ok(_) => Ok(()),
//        }
//    }
//
//    async fn prepare_code_env(&self) -> Result<CodeExecutorResult,
// CodeExecutorError> {        info!("CREATING CODE ENV");
//        self.create_id_dir().await?;
//        self.create_code_file().await?;
//        self.prepare()
//    }
//
//    async fn destroy(&self) -> Result<()> {
//        let dir = format!("{}/{}", self.playground, self.id);
//        info!("DESTROYING {}", dir);
//        Ok(tokio::fs::remove_dir_all(dir).await?)
//    }
//
//    fn execute(
//        &self,
//        input_file: &str,
//        output_file: &str,
//    ) -> Result<CodeExecutorResult, CodeExecutorError> {
//        #[cfg(not(target_os = "linux"))]
//        {
//            let mut command = self.execute_command();
//            if !Self::is_compiled() {
//                command.arg(format!(
//                    "{0}/{1}/{1}.{2}",
//                    self.playground, &self.id, &self.file_type
//                ));
//            }
//            debug!("OPENING file = {}", output_file);
//            let input = std::fs::File::open(input_file)?;
//            debug!("CREATING file = {}", output_file);
//            let output = std::fs::File::create(output_file)?;
//
//            command.stdin(input).stdout(output);
//            debug!("EXECUTING command {:?}", command);
//            crate::benchmark::run_and_meassure_2(&mut command)
//        }
//
//        #[cfg(target_os = "linux")]
//        {
//            debug!("CREATING file = {}", output_file);
//            let output = std::fs::File::create(output_file)?;
//            let command = self
//                .nsjail_execute_command()
//                .current_dir("/app/evaluator")
//                .mount("/app/evaluator/resources/", "/resources")
//                .mount("/app/evaluator/playground/", "/playground")
//                .config_file("/app/evaluator/resources/nsjail.cfg")
//                .arg("<")
//                .arg(input_file)
//                .stdout(output);
//            debug!("EXECUTING command {:?}", command);
//            crate::benchmark::run_and_meassure(command)
//        }
//    }
//
//    // TODO remove all hardcoded strings
//    fn execute_nsjail(
//        &self,
//        input_file: &str,
//        output_file: &str,
//    ) -> Result<CodeExecutorResult, CodeExecutorError> {
//        debug!("CREATING file = {}", output_file);
//        let output = std::fs::File::create(output_file)?;
//        let command = self
//            .nsjail_execute_command()
//            .current_dir("/app/evaluator")
//            .mount("/app/evaluator/resources/", "/resources")
//            .mount("/app/evaluator/playground/", "/playground")
//            .config_file("/app/evaluator/resources/nsjail.cfg")
//            .arg("<")
//            .arg(input_file)
//            .stdout(output);
//        debug!("EXECUTING command {:?}", command);
//        crate::benchmark::run_and_meassure(command)
//    }
//}

pub trait LanguageExecutor2: Send + Sync {
    fn prepare(&self) -> Result<CodeExecutorResult, CodeExecutorError> {
        Ok(CodeExecutorResult {
            status: None,
            ..Default::default()
        })
    }
    fn execute_command(&self) -> Command;
    fn nsjail_execute_command(&self) -> JailedCommand;
}

impl<L: Default> CodeExecutor<L>
where
    Self: LanguageExecutor2 + Execution,
{
    pub fn new2(playground: &str, language: &Language) -> Self {
        CodeExecutor {
            playground: playground.to_string(),
            executable: LANGUAGE
                .get(language)
                .expect("Language not available")
                .clone(),
            ..Default::default()
        }
    }
}
#[async_trait]
impl<L: Default> CodeExecutorImpl for CodeExecutor<L>
where
    Self: LanguageExecutor2 + Execution,
{
    fn set_code(&mut self, code: String) {
        self.code = code;
    }

    fn set_id(&mut self, id: u128) {
        self.id = id;
    }

    async fn create_code_file(&self) -> Result<()> {
        let file_name = match self.executable.eval_type {
            EvaluationType::Java => format!("{}/{}/Main.java", self.playground, self.id),
            _ => self.user_code_file.clone(),
        };
        debug!("CREATING CODE FILE file = {}", self.user_code_file);
        match fs::write(&file_name, self.code.as_bytes()).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                debug!("File : {file_name} , error: {:?}", &e);
                return Err(e.into());
            },
        }
    }

    async fn create_id_dir(&self) -> Result<()> {
        let dir = format!("{}/{}", self.playground, &self.id);
        debug!("CREATING ID DIRECTORY file = {}", dir);
        match tokio::fs::create_dir(&dir).await {
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => Ok(()),
                _ => {
                    debug!("File : {} , error: {:?}", &dir, &e);
                    return Err(e.into());
                },
            },
            Ok(_) => Ok(()),
        }
    }

    async fn prepare_code_env(&mut self) -> Result<CodeExecutorResult, CodeExecutorError> {
        info!("CREATING CODE ENV");

        self.user_code_file = format!(
            "{}/{}/{}.{}",
            self.playground, self.id, self.id, self.executable.file_type
        );

        self.directory = format!("{}/{}", self.playground, self.id);

        self.exec_code_file = format!("/{}/{}/{}", self.playground, self.id, self.id);

        self.create_id_dir().await?;
        self.create_code_file().await?;
        self.prepare()
    }

    async fn destroy(&self) -> Result<()> {
        info!("DESTROYING {}", self.directory);
        Ok(tokio::fs::remove_dir_all(&self.directory).await?)
    }

    fn execute(
        &self,
        input_file: &str,
        output_file: &str,
    ) -> Result<CodeExecutorResult, CodeExecutorError> {
        #[cfg(not(target_os = "linux"))]
        {
            let mut command = self.execute_command();
            //if matches!(self.executable.eval_type, EvaluationType::Compiled) {
            //    command.arg(format!(
            //        "{0}/{1}/{1}.{2}",
            //        self.playground, &self.id, &self.file_type
            //    ));
            //}
            debug!("OPENING file = {}", output_file);
            let input = std::fs::File::open(input_file)?;
            debug!("CREATING file = {}", output_file);
            let output = std::fs::File::create(output_file)?;

            command.stdin(input).stdout(output);
            debug!("EXECUTING command {:?}", command);
            crate::benchmark::run_and_meassure_2(&mut command)
        }

        #[cfg(target_os = "linux")]
        {
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

    fn parse_args(&self, data: &Vec<&str>) -> Vec<String> {
        data.iter()
            .map(|s| {
                s.replace("$file", self.user_code_file.as_str())
                    .replace("$executable", self.exec_code_file.as_str())
                    .to_string()
            })
            .collect()
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
