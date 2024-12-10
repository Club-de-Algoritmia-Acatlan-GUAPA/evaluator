use std::{any::Any, collections::HashMap, path::Path};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use primitypes::problem::{
    Checker, Problem, ProblemId, TestCaseConfig, TestCaseIdInfo, TestCaseInfo, ValidationType,
};
use reqwest::Client;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    consts::{CONFIGURATION, RESOURCES},
    types::{EvaluatorError, TestCaseError, TestCaseErrorExternalError},
};

#[derive(Debug)]
pub struct FileSystemStore<'pg> {
    client: Client,
    pg_pool: &'pg PgPool,
    problem_cache: HashMap<ProblemId, Problem>,
}
#[async_trait]
pub trait ProblemStore: std::fmt::Debug + Send + Sync {
    type Error;
    fn load_testcase(
        &self,
        problem_id: &ProblemId,
        test_case_id_info: &Uuid,
    ) -> Result<TestCaseInfo, Self::Error>;
    async fn get_problem_by_id(&self, problem_id: &ProblemId) -> Result<Problem, Self::Error>;
    //async fn get_test_case_config(&self, id: &ProblemId) ->
    // Result<TestCaseConfig, Self::Error>; fn get_full_path_test_case(&self,
    // problem_id: &ProblemId, test_case_id: &Uuid) -> TestCaseInfo;
}

impl<'pg> FileSystemStore<'pg> {
    pub async fn from(pg_pool: &'pg PgPool) -> Self {
        let client = reqwest::Client::new();
        //let problem = problems[problem_id.as_u32() as usize].1.clone();
        //
        let problem_cache: HashMap<ProblemId, Problem> = HashMap::new();

        Self {
            client,
            pg_pool,
            problem_cache,
        }
    }
}

#[async_trait]
impl ProblemStore for FileSystemStore<'_> {
    type Error = TestCaseError;

    //async fn get_test_case_config(&self, id: &ProblemId) ->
    // Result<TestCaseConfig, Self::Error> {    let client =
    // self.client.clone();    let url = format!("{}/{}",
    // CONFIGURATION.upstream.uri, id.as_u32());    let res =
    // client.get(url).send().await.map_err(|_| {
    //        TestCaseError::ExternalError(anyhow!("Unable to fetch test case
    // config"))    })?;
    //    let bytes = res
    //        .bytes()
    //        .await
    //        .map_err(|_| anyhow!("Unable to obtain bytes response"))?;
    //
    //    match bincode::deserialize(&bytes) {
    //        Ok(Some(config)) => Ok(config),
    //        _ => Err(TestCaseError::ExternalError(anyhow!(
    //            "Unable to fetch test case config"
    //        ))),
    //    }
    //}

    fn load_testcase(
        &self,
        problem_id: &ProblemId,
        test_case_id: &Uuid,
    ) -> Result<TestCaseInfo, Self::Error> {
        let stdin_path = format!("{}/{}/{}.in", *RESOURCES, problem_id.as_u32(), test_case_id);

        let stdout_path = format!(
            "{}/{}/{}.out",
            *RESOURCES,
            problem_id.as_u32(),
            test_case_id
        );
        let _ = Path::new(&stdin_path)
            .is_file()
            .then_some(|| ())
            .ok_or_else(|| {
                TestCaseError::ExternalError(EvaluatorError::GenericError(anyhow!(
                    "File: {} not found",
                    stdin_path
                )))
            })?;

        let _ = Path::new(&stdout_path)
            .is_file()
            .then_some(|| ())
            .ok_or_else(|| {
                TestCaseError::ExternalError(EvaluatorError::GenericError(anyhow!(
                    "File: {} not found",
                    stdin_path
                )))
            })?;

        Ok(TestCaseInfo {
            id: *test_case_id,
            problem_id: problem_id.clone(),
            stdin_path,
            stdout_path,
        })
    }

    async fn get_problem_by_id(&self, problem_id: &ProblemId) -> Result<Problem, Self::Error> {
        // TODO implement implentation of IntoRow
        match sqlx::query!(
            r#"
            SELECT 
                id,
                created_at,
                submitted_by,
                checker,
                body,
                memory_limit,
                time_limit,
                is_public,
                validation as "validation: ValidationType",
                testcases
            FROM problem
            WHERE id = $1
            "#,
            problem_id.as_u32() as i32
        )
        .fetch_optional(self.pg_pool)
        .await?
        {
            Some(row) => {
                let body = serde_json::from_str(&row.body.to_string()).map_err(|e| {
                    TestCaseError::ExternalError(EvaluatorError::GenericError(e.into()))
                })?;

                Ok(Problem {
                    id: ProblemId(row.id as u32),
                    checker: row.checker.map(|s| Checker { checker: s }),
                    created_at: row.created_at,
                    submitted_by: row.submitted_by,
                    body,
                    memory_limit: row.memory_limit as u16,
                    time_limit: row.time_limit as u16,
                    is_public: row.is_public,
                    validation: row.validation as ValidationType,
                    test_cases: row.testcases,
                })
            },
            None => Err(TestCaseError::ExternalError(
                EvaluatorError::ProblemNotFound,
            )),
        }
        //.map(|row| -> Result<Problem, Self::Error> {
        //    let body = serde_json::from_str(&row.body.to_string())
        //        .map_err(|e| TestCaseError::ExternalError(e.into()))?;
        //
        //    Ok(Problem {
        //        id: ProblemId(row.id as u32),
        //        checker: row.checker.map(|s| Checker { checker: s }),
        //        created_at: row.created_at,
        //        submitted_by: row.submitted_by,
        //        body,
        //        memory_limit: row.memory_limit as u16,
        //        time_limit: row.time_limit as u16,
        //        is_public: row.is_public,
        //        validation: row.validation as ValidationType,
        //        test_cases: row.testcases,
        //    })
        //})
        //.map_err(|e| TestCaseError::ExternalError(e.into()))?
    }
}

fn test_problems() -> Vec<(ProblemId, Problem)> {
    let problem = Problem {
        id: ProblemId(2),
        checker: Some(get_checker_problem_0()),
        validation: ValidationType::TestlibChecker,
        ..Default::default()
    };
    vec![
        (ProblemId(2), problem),
        (
            ProblemId(1),
            Problem {
                id: ProblemId(1),
                checker: None,
                validation: ValidationType::LiteralChecker,
                ..Default::default()
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
