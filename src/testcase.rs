use anyhow::Result;
use futures::{stream, StreamExt, TryStreamExt};
use primitypes::problem::Problem;
use primitypes::problem::TestCase;
use tokio::fs::{create_dir, File};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub async fn load_testcases<'a>(problem: &Problem) -> Result<()> {
    let dir = format!("./resources/{}", problem.problem_id.as_u32());
    if let Err(e) = create_dir(dir.as_str()).await {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => return Err(e.into()),
        };
    };

    let stream = stream::iter(problem.test_cases.clone());

    let results: Result<Vec<_>> = stream
        .then(|t| {
            let dir = dir.clone();
            async move { create_test_file(&t, &dir).await }
        })
        .try_collect()
        .await;
    results.map(|_| ())
}

pub async fn create_test_file<'a>(test_case: &TestCase, dir: &str) -> Result<()> {
    let input = format!("{}/input_{}.in", dir, test_case.id);
    let output = format!("{}/output_{}.out", dir, test_case.id);

    let mut input_file = create_file(input.as_str()).await?;
    input_file
        .write_all(test_case.input_case.as_bytes())
        .await?;

    let mut output_file = create_file(output.as_str()).await?;
    output_file
        .write_all(test_case.output_case.as_bytes())
        .await?;

    Ok(())
}
pub async fn create_file(path: &str) -> Result<File> {
    Ok(OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .await?)
}
