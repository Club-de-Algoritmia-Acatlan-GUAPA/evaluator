use std::env;

use evaluator::{
    configuration::CmdStr,
    problem_executor::{ProblemExecutor, ProblemExecutorError},
    store::ProblemStore,
    types::TestCaseError,
    utils::get_testcases,
};
use expected_response::{
    get_expected_accepted, get_expected_partial_runtime_error, get_expected_runtime_error,
    get_expected_time_limit,
};
use pretty_assertions::assert_eq;
use primitypes::{
    contest::{Language, Submission},
    problem::{
        Checker, ContestId, Problem, ProblemExecutorResult, ProblemID, SubmissionId,
        TestCaseConfig, TestCaseInfo, ValidationType,
    },
};
use test_log::test;
use uuid::Uuid;

use crate::sum_of_two_values::{expected_response, fake_store};

#[test(tokio::test)]
async fn test_runtime_error() {
    let res = evaluate_code(get_code_accepted()).await;
    let expected_res = get_expected_accepted();
    for (idx, test) in res
        .expect("Expected result got error")
        .test_cases_results
        .iter()
        .enumerate()
    {
        assert_eq!(expected_res.test_cases_results[idx].status, test.status);
    }
}

#[test(tokio::test)]
async fn test_partial_runtime_error() {
    let res = evaluate_code(get_code_runtime_error_in_some_cases()).await;
    let expected_res = get_expected_partial_runtime_error();
    for (idx, test) in res
        .expect("Expected result got error")
        .test_cases_results
        .iter()
        .enumerate()
    {
        println!("{}, {}",expected_res.test_cases_results[idx].status, test.status);
    }
}

//#[test]
//fn test_time_limit_exceeded() {
//    assert_eq!(
//        evaluate_code(get_code_time_limit()),
//        get_expected_time_limit()
//    );
//}
//#[ignore]
//#[test]
//fn test_accepted() {
//    //assert_eq!(evaluate_code(get_code_accepted()), get_expected_accepted());
//}

async fn evaluate_code(code: String) -> Result<ProblemExecutorResult, ProblemExecutorError> {
    let executor = ProblemExecutor::new("./tests/playground/", "./tests/sum_of_two_values/stdio/");
    env::set_var("CONFIGURATION_DIRECTORY", "./tests");
    env::set_var("CONFIGURATION_FILE", "config.yml");
    // let test_cases =
    // get_testcases("./tests/sum_of_two_values/stdio".to_string());
    let user_id = Uuid::new_v4();
    let submission = Submission {
        language: Language::Python3,
        code: code.into(),
        problem_id: ProblemID(1234),
        contest_id: Some(ContestId(1234)),
        id: SubmissionId::new(
            90,
            &ProblemID(1234),
            Some(ContestId(1234)).as_ref(),
            &user_id,
        ),
        user_id,
    };
    let problem = Problem {
        id: ProblemID(1),
        checker: Some(get_checker()),
        validation: ValidationType::TestlibChecker(get_checker()),
        memory_limit: 259,
        time_limit: 2,
        ..Default::default()
    };

    let test_case_config = get_test_case_config();
    let store = fake_store::FakeStore::new(problem, test_case_config);
    let store = &store as &dyn ProblemStore<Error = TestCaseError>;
    executor.execute(&submission, store).await
}
//fn sort_by_id(mut arr: Vec<TestCaseResult>) -> Vec<TestCaseResult> {
//    arr.sort_by(|a, b| a.id.cmp(&b.id));
//    arr
//}
//
//fn output_to_none(mut arr: Vec<TestCaseResult>) -> Vec<TestCaseResult> {
//    arr.iter_mut().for_each(|elem| elem.output = None);
//    arr
//}

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
# sleep(100)
IO = lambda: list(map(int, input().split()))
n, target = IO()
arr = [ (value, idx + 1) for idx, value in enumerate(IO()) ]
dic = {}

def solve():
    #print(arr)
    arr.sort()
    l , r = 0, n - 1
    while l < r:
        if arr[l][0] + arr[r][0] == target:
            print(arr[l][1], arr[r][1],end='')
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

fn get_test_case_config() -> TestCaseConfig {
    let test_cases = vec![
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_0.in".to_string(),
            stdout_path: Some("output_0.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_1.in".to_string(),
            stdout_path: Some("output_1.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_2.in".to_string(),
            stdout_path: Some("output_2.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_3.in".to_string(),
            stdout_path: Some("output_3.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_4.in".to_string(),
            stdout_path: Some("output_4.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_5.in".to_string(),
            stdout_path: Some("output_5.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_6.in".to_string(),
            stdout_path: Some("output_6.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_7.in".to_string(),
            stdout_path: Some("output_7.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_8.in".to_string(),
            stdout_path: Some("output_8.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_9.in".to_string(),
            stdout_path: Some("output_9.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_10.in".to_string(),
            stdout_path: Some("output_10.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_11.in".to_string(),
            stdout_path: Some("output_11.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_12.in".to_string(),
            stdout_path: Some("output_12.out".to_string()),
        },
        TestCaseInfo {
            problem_id: ProblemID(1),
            id: Uuid::new_v4(),
            stdin_path: "input_13.in".to_string(),
            stdout_path: Some("output_13.out".to_string()),
        },
    ];
    TestCaseConfig { test_cases }
}
