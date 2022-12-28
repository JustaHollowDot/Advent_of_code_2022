use std::fs::read_to_string;
use crate::{InputReader, Solution};

impl InputReader {
    pub fn read_day_14_task() -> String {
        let contents = read_to_string(r"input\day_14\day_14_input").unwrap();
        return contents
    }

    pub fn read_day_14_task_test() -> String {
        let contents = read_to_string(r"input\day_14\day_14_input_test").unwrap();
        return contents
    }
}

impl Solution {
    pub fn day_14_task_1() {
        let contents = InputReader::read_day_14_task_test();
        println!("contents: {:?}", contents);

        let lines = contents.lines();

        let mut lines = lines.map(|line| {
            let split = line.split(" -> ").collect::<Vec<&str>>();

            return split.iter().map(|s| s.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        }).collect::<Vec<Vec<Vec<&str>>>>();


        println!("lines:\n{:#?}", lines);

    }
}