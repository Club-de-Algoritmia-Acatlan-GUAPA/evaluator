use primitypes::problem::{Checker, PolicyExecution, Problem, ProblemID, TestCase, ValidatorType};
use std::collections::HashMap;

pub struct ProblemStore {
    pub store: HashMap<ProblemID, Problem>,
}

impl Default for ProblemStore {
    fn default() -> Self {
        Self::new()
    }
}
impl ProblemStore {
    pub fn new() -> Self {
        let problems = test_problems();
        Self {
            store: problems.into_iter().collect(),
        }
    }

    pub async fn get_problem(&mut self, problem_id: &ProblemID) -> Option<Problem> {
        self.store.get(problem_id).cloned()
    }

    pub async fn download_problem() -> Option<Problem> {
        todo!()
    }
}

pub fn test_problems() -> Vec<(ProblemID, Problem)> {
    let problem = Problem {
        problem_id: ProblemID(2),
        name: Some("Sum of Two Values".to_string()),
        policy_execution: PolicyExecution::Checker,
        system_policy: None,
        test_cases: (0..=23)
            .map(|s| TestCase {
                id: s,
                ..Default::default()
            })
            .collect::<Vec<_>>(),
        checker: Some(get_checker_problem_0()),
        validation_type: ValidatorType::TestLibChecker,
    };
    vec![
        (ProblemID(2), problem),
        (
            ProblemID(1),
            Problem {
                problem_id: ProblemID(1),
                name: Some("Missing Number".to_string()),
                policy_execution: PolicyExecution::AnswerFile,
                system_policy: None,
                test_cases: (0..=13)
                    .map(|s| TestCase {
                        id: s,
                        ..Default::default()
                    })
                    .collect::<Vec<_>>(),
                checker: Some(get_checker_problem_0()),
                validation_type: ValidatorType::LiteralChecker,
            },
        ),
    ]
}

fn get_checker_problem_0() -> Checker {
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
