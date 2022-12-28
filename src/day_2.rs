use std::fs::read_to_string;
use std::io::{Read, stdin};
use crate::Solution;

impl Solution {
    pub fn day_2_task_1() {
        let contents = read_to_string(r"input\day_3\day_3_input").unwrap();
        println!("{:?}", contents);

        let mut matches: Vec<Vec<&str>> = Vec::new();

        for line in contents.lines() {
            let new_match = line.split(" ")
                .map(|x|
                    return match x {
                        "X" => { "A" }
                        "Y" => { "B" }
                        "Z" => { "C" }
                        _ => { x }
                    }
                )
                .collect();
            matches.push(new_match);
        }

        println!("{:?}", matches);

        let mut score = 0;

        for kamp in matches {
            match kamp[0] {
                "A" => match kamp[1] {
                    "A" => score += 1 + 3,
                    "B" => score += 2 + 6,
                    "C" => score += 3 + 0,
                    _ => {}
                },
                "B" => match kamp[1] {
                    "A" => score += 1 + 0,
                    "B" => score += 2 + 3,
                    "C" => score += 3 + 6,
                    _ => {}
                },
                "C" => match kamp[1] {
                    "A" => score += 1 + 6,
                    "B" => score += 2 + 0,
                    "C" => score += 3 + 3,
                    _ => {}
                },
                _ => {}
            }
        }

        println!("{}", score);

    }

    pub fn day_2_task_2() {
        let contents = read_to_string(r"input\day_3\day_3_input").unwrap();
        println!("{:?}", contents);

        let mut matches: Vec<Vec<&str>> = Vec::new();

        for line in contents.lines() {
            let new_match = line.split(" ").collect();
            matches.push(new_match);
        }

        println!("{:?}", matches);

        let mut score = 0;

        for kamp in matches {
            match kamp[0] {
                "A" => match kamp[1] {
                    "X" => score += 3 + 0,
                    "Y" => score += 1 + 3,
                    "Z" => score += 2 + 6,
                    _ => {}
                },
                "B" => match kamp[1] {
                    "X" => score += 1 + 0,
                    "Y" => score += 2 + 3,
                    "Z" => score += 3 + 6,
                    _ => {}
                },
                "C" => match kamp[1] {
                    "X" => score += 2 + 0,
                    "Y" => score += 3 + 3,
                    "Z" => score += 1 + 6,
                    _ => {}
                },
                _ => {}
            }
        }

        println!("{}", score);

    }
}