use anyhow::Result;

use crate::code_executor::{CodeExecutor, CodeExecutorError, CodeExecutorResult, LanguageExecutor};
use primitypes::{contest::Language, status::Status};
use std::process::{Command, Stdio};

use crate::command::JailedCommand;

trait Cpp {
    fn get_cpp_version(&self) -> String;
}

#[derive(Default, Clone)]
pub struct Cpp17;
#[derive(Default)]
pub struct Cpp11;

impl Cpp for CodeExecutor<Cpp17> {
    fn get_cpp_version(&self) -> String {
        "-std=c++1z".to_string()
    }
}
impl Cpp for CodeExecutor<Cpp11> {
    fn get_cpp_version(&self) -> String {
        "-std=c++11".to_string()
    }
}
impl<L> LanguageExecutor for CodeExecutor<L>
where
    Self: Cpp + Send + Sync,
{
    fn prepare(&self) -> Result<CodeExecutorResult, CodeExecutorError> {
        let mut command = Command::new("/usr/bin/g++");

        // create executable
        let file_name = format!("{}.{}", self.id, Self::get_file_type());
        let exec_name = format!("{}", self.id);

        let child = command
            .current_dir(format!("./{}/{}", self.directory, self.id))
            .args(vec![
                &self.get_cpp_version(),
                &file_name,
                &"-o".to_string(),
                &exec_name,
            ])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let status_result = child.status.success();
        println!("{:?}", child);
        if status_result {
            Ok(CodeExecutorResult {
                status: Some(child.status),
                output: Some(child),
                ..Default::default()
            })
        } else {
            Err(CodeExecutorError::InternalError(
                crate::code_executor::CodeExecutorInternalError {
                    status: Status::CompilationError,
                    output: Some(child),
                    ..Default::default()
                },
            ))
        }
    }

    fn execute_command(&self) -> std::process::Command {
        Command::new(format!("./{0}", self.id))
    }

    fn nsjail_execute_command(&self) -> JailedCommand {
        JailedCommand::new(format!("/playground/{0}/{0}", &self.id))
    }

    fn get_file_type() -> String {
        "cpp".to_string()
    }
    fn is_compiled() -> bool {
        true
    }
    fn language() -> Language {
        Language::Cpp11
    }
}

#[tokio::test]
#[ignore = "reason"]
pub async fn test_execute_function() -> Result<()> {
    use crate::code_executor::{CodeExecutor, CodeExecutorImpl};
    use crate::utils::get_testcases;
    let code = r#"
    #include<bits/stdc++.h>
 
    using namespace std;
    // TLE
    void solve() {
        map<int, int>m;
        int n , target;
        cin>>n>>target;
        vector<int>arr(n);
        for(auto &x: arr)cin>>x;
        for(int idx = 0; idx < n; idx++) { 
            for(int i = idx + 1 ; i < n ; i++) {
                if(arr[i] + arr[idx] == target) { 
                    cout<<idx + 1 << " "<< i + 1;
                    return;
                }
            }
        }
        cout<<"IMPOSSIBLE"<<endl;
    }
    int main() {
       solve();
    }
    "#;

    let test_cases = get_testcases("./tests/sum_of_two_values/stdio".to_string());

    let mut executor = CodeExecutor::<Cpp17>::new();
    executor.code(code.to_string());
    executor.set_id(12);
    executor.prepare_code_env().await.unwrap();

    let _ = executor.prepare_code_env().await.unwrap();
    let res = executor.execute(&test_cases[1].input_case);
    let _ = executor.destroy().await;
    let _ = dbg!(res);
    Ok(())
}
