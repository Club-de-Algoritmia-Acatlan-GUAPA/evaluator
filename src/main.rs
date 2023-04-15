use evaluator::problem_executor::ProblemExecutor;
use evaluator::types::{Checker, Language, PolicyExecution, Problem, Submission, TestCase};
use evaluator::utils;
use evaluator::validator::ValidatorType;
fn main() {
    //TODO pre compile testlib
    let files = utils::get_testcases_names("./tests/sum_of_two_values/stdio".to_string());
    let mut test_cases = vec![];
    files.iter().enumerate().for_each(|(idx, elem)| {
        if elem.len() <= 1 {
            return;
        }
        test_cases.push(TestCase {
            input_case: utils::file_to_string(elem[0].clone()), // input testcase
            output_case: utils::file_to_string(elem[1].clone()), // input testcas
            id: idx as i32,
        });
    });

    let executor = ProblemExecutor::new();
    for (idx, code) in vec![
        get_code_runtime_error(),
        get_code_runtime_error_in_some_cases(),
        get_code_time_limit(),
        get_code_accepted(),
    ]
    .into_iter()
    .enumerate()
    {
        let submission = Submission {
            language: Language::Python3,
            code,
            id: idx as i32,
        };
        let problem = Problem {
            id: "123123".to_string(),
            name: Some("Sum of Two Values".to_string()),
            policy_execution: PolicyExecution::Checker,
            system_policy: None,
            test_cases: test_cases.clone(),
            checker: Some(get_checker()),
            validation_type: ValidatorType::TestLibChecker,
        };
        let res = executor.execute(submission, problem).unwrap();
        // dbg!(res);
        println!("{:?}", res);
    }

    for (idx, code) in vec![
        get_cpp_tle(),
        get_cpp_acc()
    ]
    .into_iter()
    .enumerate()
    {
        let submission = Submission {
            language: Language::Cpp,
            code,
            id: idx as i32,
        };
        let problem = Problem {
            id: "123123".to_string(),
            name: Some("Sum of Two Values".to_string()),
            policy_execution: PolicyExecution::Checker,
            system_policy: None,
            test_cases: test_cases.clone(),
            checker: Some(get_checker()),
            validation_type: ValidatorType::TestLibChecker,
        };
        let res = executor.execute(submission, problem).unwrap();
        // dbg!(res);
        println!("{:?}", res);
    }
    //  todo!("Move all of this to a test");
}

fn get_code_runtime_error() -> String {
    r#"
a,tar= [int(x) for x in raw_input().split(' ')]
arr = [int(x) for x in raw_input().split(' ')]
mapa = {}
for ( i , x) in enumerate(arr):
    mapa[x] = i
for (i,x) in enumerate(arr):
    if tar - x in mapa and mapa[tar-x] != i:
        print i + 1, mapa[tar-x] + 1
        exit()
print("IMPOSSIBLE")
        "#
    .to_string()
}
fn get_cpp_tle() -> String {
    r#"
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
    "#.to_string()
}
fn get_cpp_acc() -> String {
r#"
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
        int val = target  - arr[idx];
        if( m.count(val) ) { 
            cout<<idx + 1 << " "<< m[val] + 1<<endl;
            return;
        }
        m[arr[idx]] = idx;
    }
    cout<<"IMPOSSIBLE"<<endl;
}
int main() {
   solve();
}

"#.to_string()
}
fn get_code_runtime_error_in_some_cases() -> String {
    r#"
IO = lambda: list(map(int, input().split()))
n, target = IO()
arr = IO()
dic = {}

def solve():
    for idx, v in enumerate(arr):
        if target - v in dic:
            print(dic[target - v] + 1, idx + 1)
            IO[1]
            return
        dic[v] = idx
    print("IMPOSSIBLE")
solve()"#
        .to_string()
}

fn get_code_time_limit() -> String {
    r#"
IO = lambda: list(map(int, input().split()))
n, target = IO()
arr = IO()
dic = {}

def solve():
    for idx, v in enumerate(arr):
        if target - v in dic:
            print(dic[target - v] + 1, idx + 1)
            #IO[1]
            return
        dic[v] = idx
    print("IMPOSSIBLE")
solve()"#
        .to_string()
}
fn get_code_accepted() -> String {
    r#"
from time import sleep
IO = lambda: list(map(int, input().split()))
#sleep(100)
n, target = IO()
arr = [ (value, idx + 1) for idx, value in enumerate(IO()) ]
dic = {}

def solve():
    #print(arr)
    arr.sort()
    l , r = 0, n - 1
    while l < r:
        if arr[l][0] + arr[r][0] == target:
            print(arr[l][1], arr[r][1])
            return
        elif arr[l][0] + arr[r][0]  > target :
            r -= 1
        else:
            l += 1
    print("IMPOSSIBLE")
solve()
"#
    .to_string()
}
fn get_checker() -> Checker {
    Checker {
        checker: r#"
#include "testlib.h"
#include <vector>

using namespace std;

const double EPS = 1.5E-5;
//inf ouf ans
int main(int argc, char *argv[]) {
    //setName("compare two sequences of doubles, maximal absolute error = %.10f", EPS);
    registerTestlibCmd(argc, argv);
    int i_n = inf.readInt(1, 2 * (int) 1e5);
    int target = inf.readInt(1, (int) 1e9);
    vector<int>arr;
    for(int idx = 0; idx < i_n; idx++) {
        int k = inf.readInt(1, (int) 1e9);
        arr.push_back(k);
    }
    auto possible = ans.readWord();
    if(possible == "IMPOSSIBLE") {
        auto res = ouf.readWord();
        if(res == "IMPOSSIBLE") quitf(_ok, "");
        else quitf(_wa,"");
    }
    int u_a = ouf.readInt(1,  i_n);
    int u_b = ouf.readInt(1,  i_n);
    if( arr[u_a - 1] + arr[u_b - 1] != target) {
        quitf(_wa,"");
    }
    quitf(_ok, "");
} "#
        .to_string(),
    }
}
