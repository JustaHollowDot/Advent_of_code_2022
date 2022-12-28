use std::fs::read_to_string;
use crate::{InputReader, Solution};

impl InputReader {
    pub fn read_day_13_task() -> String {
        let contents = read_to_string(r"input\day_13\day_13_input").unwrap();
        return contents
    }

    pub fn read_day_13_task_test() -> String {
        let contents = read_to_string(r"input\day_13\day_13_input_test").unwrap();
        return contents
    }
}

impl Solution {
    pub fn day_13_task_1() {
        let contents = InputReader::read_day_13_task_test();

        let pairs = contents.split("\r\n\r\n")
            .map(|pair| {
                let split = pair.split("\r\n").collect::<Vec<&str>>();
                return vec![split[0], split[1]];
            })
            .collect::<Vec<Vec<&str>>>();

        for pair in &pairs {
            println!("pair: {:?}", pair);
        }

        println!();

        let mut formated_pairs = Vec::new();

        for pair in &pairs {
            let pair_0 = pair[0];
            let pair_1 = pair[1];

            let test = pair_0;

            let left_bracket: Vec<_> = test.match_indices("[").collect();
            let right_bracket: Vec<_> = test.match_indices("]").collect();

            println!("left_bracket: {:?}", left_bracket);
            println!("right_bracket: {:?}", right_bracket);




            let pair_0 = pair_0.split(",").collect::<Vec<&str>>();
            let pair_1 = pair_1.split(",").collect::<Vec<&str>>();

            println!("pair_0: {:?}", pair_0);
            println!("pair_1: {:?}", pair_1);


            formated_pairs.push(vec![pair_0, pair_1]);
        }


        println!();

        for pair in &formated_pairs {
            println!("pair: {:?}", pair);
        }


    }
}