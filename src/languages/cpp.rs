use anyhow::Result;
use std::fs;
use std::io::{Read, Write};

use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::time::{Duration, Instant};

use crate::types::{CodeExecutor, CodeExecutorResult, Status, TestCase};

pub struct Cpp {
    pub file_ending: String,
    pub file_for_execution: String, // TODO : convert to OsStr
    pub executable_name: String, // TODO : convert to OsStr
}
impl Cpp {
    pub fn new(content: String) -> Result<Self> {
        let mut file = fs::File::create("./playground/foo.cpp")?;
        file.write_all(content.as_bytes())?;

        Ok(Self {
            file_ending: "cpp".to_string(),
            file_for_execution: String::from("foo.cpp"),
            executable_name: String::from("foo"),
        })
    }
}
impl CodeExecutor for Cpp {
    fn execute(&self, testcase: &TestCase) -> Result<CodeExecutorResult> {
        let mut command = Command::new("g++-12");

        // create ecxecutable
        let child = command
            .current_dir("./playground")
            .arg("-std=c++1z")
            .arg(&self.file_for_execution)
            .arg("-o")
            .arg(&self.executable_name)
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
        // run executable
        let comp_res = child.wait()?;
        if !comp_res.success() {
            dbg!(&child);
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: Some(Output {
                    status: ExitStatus::from_raw(1),
                    stdout: child.stdout.unwrap().bytes().filter_map(|x| x.ok()).collect::<Vec<_>>(),
                    stderr: child.stderr.unwrap().bytes().filter_map(|x| x.ok()).collect::<Vec<_>>(),
                }),
            });
        }
        let mut command = Command::new(format!("./{}",self.executable_name));

        let child = command
            .current_dir("./playground")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match child.spawn() {
            Ok(child) => child,
            Err(v) => {
                dbg!(&v);
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

        let child_stdin = child.stdin.as_mut().expect("F");
        if child_stdin
            .write_all(testcase.input_case.as_bytes())
            .is_err()
        {
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: Some(Output {
                    status: ExitStatus::from_raw(1),
                    stdout: child
                        .stdout
                        .unwrap()
                        .bytes()
                        .filter_map(|x| x.ok())
                        .collect::<Vec<_>>(),
                    stderr: child
                        .stderr
                        .unwrap()
                        .bytes()
                        .filter_map(|x| x.ok())
                        .collect::<Vec<_>>(),
                }),
            });
        }
        let one_sec = Duration::from_secs(1);
        let now = Instant::now();
        loop {
            let result = child.try_wait();

            match result {
                Ok(Some(_)) => {
                    break;
                }
                Ok(None) => {
                    if now.elapsed() > one_sec {
                        child.kill().unwrap();
                        child.kill()?;
                        return Ok(CodeExecutorResult {
                            err: Some(Status::TimeLimitExceeded),
                            output: None,
                        });
                    }
                }
                Err(e) => {
                    panic!("Error: {e}");
                }
            }
        }

        let status = child.wait().unwrap();
        let stdout = child.stdout.unwrap();
        let mut stderr = vec![];

        if !status.success() {
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: Some(Output {
                    status,
                    stdout: stdout.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>(),
                    stderr: vec![],
                }),
            });
        }
        if let Some(v) = child.stderr {
            stderr = v.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>();
        }

        Ok(CodeExecutorResult {
            err: None,
            output: Some(Output {
                status,
                stdout: stdout.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>(),
                stderr,
            }),
        })
    }
}

#[test] 
pub fn test_execute_function()->Result<()> { 
    use crate::utils::get_testcases;
    let code = r#"
    #include<bits/stdc++.h>
 
    using namespace std;
    // accepted
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

    let executor  = Cpp::new(code.to_string())?;
    let val = executor.execute(&test_cases[6]).unwrap();
    dbg!(val);
    Ok(())

}