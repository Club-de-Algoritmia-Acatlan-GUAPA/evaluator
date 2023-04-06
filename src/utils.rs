use regex::Regex;
use slice_group_by::GroupBy;

use std::fs;

use crate::types::TestCase;

pub fn file_to_string(path: String) -> String {
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

pub fn get_testcases(path: String) -> Vec<TestCase> {
    let files = get_testcases_names(path);
    let mut test_cases = vec![];
    files.iter().enumerate().for_each(|(idx, elem)| {
        if elem.len() <= 1 {
            return;
        }
        test_cases.push(TestCase {
            input_case: file_to_string(elem[0].clone()), // input testcase
            output_case: file_to_string(elem[1].clone()), // input testcas
            id: idx as i32,
        });
    });

    test_cases
}