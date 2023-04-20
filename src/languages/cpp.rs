use anyhow::Result;

use std::{
    io::Read,
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus, Output, Stdio},
};

use crate::code_executor::{CodeExecutor, CodeExecutorResult, LanguageExecutor};
use crate::types::Status;

trait CPP {
    fn get_cpp_version(&self) -> String;
}

#[derive(Default)]
pub struct Cpp17;
#[derive(Default)]
pub struct Cpp11;

impl CPP for CodeExecutor<Cpp17> {
    fn get_cpp_version(&self) -> String {
        "-std=c++1z".to_string()
    }
}
impl CPP for CodeExecutor<Cpp11> {
    fn get_cpp_version(&self) -> String {
        "-std=c++11".to_string()
    }
}
impl <L> LanguageExecutor for CodeExecutor<L>
where Self : CPP + Send + Sync {
    fn prepare(&self) -> Result<CodeExecutorResult> {
        let mut command = Command::new("g++-12");

        // create executable
        let child = command
            .current_dir("./playground")
            .args(vec![
                &self.get_cpp_version(), 
                &format!("{}.{}", self.id, Self::get_file_type()),
                &"-o".to_string(),
                &format!("{}", self.id),
            ])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match child.spawn() {
            Ok(child) => child,
            Err(v) => {
                return Ok(CodeExecutorResult {
                    err: Some(Status::RuntimeError),
                    output: Some(Output {
                        status: ExitStatus::from_raw(1),
                        stdout: v.to_string().as_bytes().to_vec(),
                        stderr: vec![],
                    }),
                });
            }
        };
        // wait for any other possible runtime error
        let status = child.wait()?;
        let stdout = child.stdout.map_or_else(Vec::new, |stdout| {
            stdout.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>()
        });
        let stderr = child.stderr.map_or_else(Vec::new, |stdout| {
            stdout.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>()
        });
        if !status.success() {
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: Some(Output {
                    status: ExitStatus::from_raw(1),
                    stdout,
                    stderr,
                }),
            });
        }
        Ok(CodeExecutorResult {
            err: None,
            output: Some(Output {
                status,
                stdout,
                stderr,
            }),
        })
    }

    fn execute_command(&self) -> std::process::Command {
        Command::new(format!("./{}", self.id))
    }

    fn get_file_type() -> String {
        "cpp".to_string()
    }
}

#[test]
pub fn test_execute_function() -> Result<()> {
    use crate::code_executor::CodeExecutor;
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
    executor.prepare_code_env()?;

    let _ = executor.prepare_code_env()?;
    let res = executor.execute(&test_cases[21]);
    let _ = dbg!(res);
    Ok(())
}
