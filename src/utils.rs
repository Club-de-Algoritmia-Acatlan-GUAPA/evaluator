use std::fs;

use anyhow::Result;
use primitypes::problem::STestCase;
use regex::Regex;
use slice_group_by::GroupBy;

pub fn file_to_string(path: &str) -> String {
    let file = fs::read(path).unwrap();
    String::from_utf8_lossy(&file).to_string()
}
pub fn file_to_bytes(path: &str) -> Result<Vec<u8>> {
    let file = fs::read(path)?;
    Ok(String::from_utf8_lossy(&file).as_bytes().to_vec())
}
pub fn bytes_to_str(path: String) -> String {
    let file = fs::read(path).unwrap();
    String::from_utf8_lossy(&file).to_string()
}
pub fn get_testcases_names(path: String) -> Vec<Vec<String>> {
    let paths = fs::read_dir(path).unwrap();

    let mut name_files = vec![];
    for path in paths {
        let file_name = path.unwrap().path().display().to_string();
        name_files.push(file_name);
    }
    let re = Regex::new(r"\d+").unwrap();

    name_files.sort_by(|a, b| {
        let (num, num2);
        if let Some(cap) = re.find(a) {
            num = Some(cap.as_str().parse::<i32>().unwrap());
        } else {
            num = None;
        }
        if let Some(cap) = re.find(b) {
            num2 = Some(cap.as_str().parse::<i32>().unwrap());
        } else {
            num2 = None;
        }
        num.cmp(&num2)
    });
    let mut res = name_files
        .as_slice()
        .linear_group_by(|a, b| {
            let (num, num2);
            if let Some(cap) = re.find(a) {
                num = cap.as_str();
            } else {
                num = "";
            }
            if let Some(cap) = re.find(b) {
                num2 = cap.as_str();
            } else {
                num2 = "";
            }
            num == num2
        })
        .map(Vec::from)
        .collect::<Vec<_>>();

    for i in res.iter_mut() {
        i.sort();
    }
    res
}

pub fn get_testcases(path: String) -> Vec<STestCase> {
    let files = get_testcases_names(path);
    let mut test_cases = vec![];
    files.iter().enumerate().for_each(|(_idx, elem)| {
        if elem.len() <= 1 {
            return;
        }
        test_cases.push(STestCase {
            input_case: file_to_string(&elem[0].clone()), // input testcase
            output_case: file_to_string(&elem[1].clone()), // input testcas
            id: uuid::Uuid::new_v4(),
        });
    });

    test_cases
}
