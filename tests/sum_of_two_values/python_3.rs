use crate::sum_of_two_values::expected_response;
use evaluator::problem_executor::ProblemExecutor;
use evaluator::types::{Checker, PolicyExecution, Problem, ProblemExecutorResult, TestCaseResult};
use evaluator::utils::get_testcases;
use evaluator::validator::ValidatorType;
use expected_response::{
    get_expected_accepted, get_expected_partial_runtime_error, get_expected_runtime_error,
    get_expected_time_limit,
};
use pretty_assertions::assert_eq;
use primitypes::contest::{Language, Submission};
use primitypes::problem::{ContestId, ProblemID, SubmissionID};
use uuid::Uuid;
#[test]
fn test_runtime_error() {
    assert_eq!(
        evaluate_code(get_code_runtime_error()),
        get_expected_runtime_error()
    );
}

#[test]
fn test_partial_runtime_error() {
    assert_eq!(
        evaluate_code(get_code_runtime_error_in_some_cases()),
        get_expected_partial_runtime_error()
    );
}

#[test]
fn test_time_limit_exceeded() {
    assert_eq!(
        evaluate_code(get_code_time_limit()),
        get_expected_time_limit()
    );
}
#[ignore]
#[test]
fn test_accepted() {
    //assert_eq!(evaluate_code(get_code_accepted()), get_expected_accepted());
}

fn evaluate_code(code: String) -> ProblemExecutorResult {
    let executor = ProblemExecutor::new();
    let test_cases = get_testcases("./tests/sum_of_two_values/stdio".to_string());
    let user_id = Uuid::new_v4();
    let submission = Submission {
        language: Language::Python3,
        code,
        problem_id: ProblemID(1234),
        contest_id: Some(ContestId(1234)),
        id: SubmissionID::new(
            90,
            &ProblemID(1234),
            Some(ContestId(1234)).as_ref(),
            &user_id,
        ),
        user_id,
    };
    let problem = Problem {
        problem_id: "123123".to_string(),
        name: Some("Sum of Two Values".to_string()),
        policy_execution: PolicyExecution::Checker,
        system_policy: None,
        test_cases: test_cases.clone(),
        checker: Some(get_checker()),
        validation_type: ValidatorType::TestLibChecker,
    };
    let mut res = executor.execute(&submission, problem).unwrap();
    res.test_cases_results = sort_by_id(res.test_cases_results);
    res.test_cases_results = output_to_none(sort_by_id(res.test_cases_results));

    res
}
fn sort_by_id(mut arr: Vec<TestCaseResult>) -> Vec<TestCaseResult> {
    arr.sort_by(|a, b| a.id.cmp(&b.id));
    arr
}

fn output_to_none(mut arr: Vec<TestCaseResult>) -> Vec<TestCaseResult> {
    arr.iter_mut().for_each(|elem| elem.output = None);
    arr
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
