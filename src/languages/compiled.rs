use std::process::{Command, Stdio};

use anyhow::Result;
use primitypes::{contest::Language, status::Status};

use crate::{
    code_executor::{
        CodeExecutor, CodeExecutorError, CodeExecutorImpl, CodeExecutorResult, Execution,
        LanguageExecutor2,
    },
    command::JailedCommand,
};

#[derive(Default, Clone)]
pub struct Compiled;
impl Execution for CodeExecutor<Compiled> {}
impl LanguageExecutor2 for CodeExecutor<Compiled>
where
    Self: Send + Sync,
{
    fn prepare(&self) -> Result<CodeExecutorResult, CodeExecutorError> {
        let path = &self.executable.path;
        let mut command = Command::new(path);

        let vec: Vec<_> = self.executable.args.iter().map(|s| s.as_str()).collect();
        let args: Vec<_> = self.parse_args(&vec);

        // create executable

        let child = command
            .current_dir(format!("./{}/{}", self.directory, self.id))
            .args( args
                //vec![
                //&self.get_cpp_version(),
                //..args,
                //&file_name,
                ////&"-Wall".to_string(),
                ////&"-Wextra".to_string(),
                ////&"-Wconversion".to_string(),
                ////&"-static".to_string(),
                ////&"-Wl,-stack_size=268435456".to_string(),
                ////&"-O2".to_string(),
                //&"-o".to_string(),
                //&exec_name, ]
            )
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
        Command::new(&self.exec_code_file)
        //format!("{0}/{1}/{1}", self.playground, self.id))
    }

    fn nsjail_execute_command(&self) -> JailedCommand {
        //JailedCommand::new(format!("{0}/{1}/{1}", self.playground, self.id))
        JailedCommand::new(self.exec_code_file.clone())
    }
}

//#[ignore = "reason"]
//#[tokio::test]
//pub async fn test_execute_function() -> Result<()> {
//    use crate::{
//        code_executor::{CodeExecutor, CodeExecutorImpl},
//        utils::get_testcases,
//    };
//    let code = r#"
//    #include<bits/stdc++.h>
//
//    using namespace std;
//    // TLE
//    void solve() {
//        map<int, int>m;
//        int n , target;
//        cin>>n>>target;
//        vector<int>arr(n);
//        for(auto &x: arr)cin>>x;
//        for(int idx = 0; idx < n; idx++) {
//            for(int i = idx + 1 ; i < n ; i++) {
//                if(arr[i] + arr[idx] == target) {
//                    cout<<idx + 1 << " "<< i + 1;
//                    return;
//                }
//            }
//        }
//        cout<<"IMPOSSIBLE"<<endl;
//    }
//    int main() {
//       solve();
//    }
//    "#;
//
//    let test_cases =
// get_testcases("./tests/sum_of_two_values/stdio".to_string());
//
//    let mut executor = CodeExecutor::<Cpp17>::new();
//    executor.code(code.to_string());
//    executor.set_id(12);
//    executor.prepare_code_env().await.unwrap();
//
//    let _ = executor.prepare_code_env().await.unwrap();
//    let res = executor.execute(&test_cases[1].input_case);
//    let _ = executor.destroy().await;
//    let _ = dbg!(res);
//    Ok(())
//}
