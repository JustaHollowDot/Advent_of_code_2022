use std::fs::read_to_string;
use crate::{InputReader, Solution};

impl InputReader {
    pub fn read_input_day_11() -> String {
        let contents = read_to_string(r"input\day_11\day_11_input").unwrap();
        return contents;
    }

    pub fn read_input_day_11_test() -> String {
        let contents = read_to_string(r"input\day_11\day_11_input_test").unwrap();
        return contents;
    }
}

impl Solution {
    pub fn day_11_task_1() -> i32 {
        let contents = InputReader::read_input_day_11();
        println!("{}", contents);

        let apes = contents.split("\r\n\r\n").collect::<Vec<&str>>();

        println!("{:#?}", apes);

        let apes = apes.iter()
            .map(|ape| ape.lines()
                .map(|line| line.chars()
                    .filter(|c| c != &',' && c != &':')
                    .collect::<String>())
                .collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();

        let mut apes = apes.iter()
            .map(|ape| ape.iter()
                .map(|line| line.split_whitespace()
                    .filter(|s| s.parse::<i32>().is_ok() || s == &"+" || s == &"*" || s == &"old")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>())
            .collect::<Vec<Vec<Vec<String>>>>();

        for ape in &apes {
            println!("{:?}", ape);
        }

        let mut inspections = vec![0; apes.len()];

        for i in 0..20 {
            for j in 0..apes.len() {
                // println!("apes -> {:?}: {:?}", j, apes[j]);
                for item in apes[j][1].clone() {
                    let mut item = item.parse::<i128>().unwrap();
                    let action = &apes[j][2][1];
                    let value = apes[j][2][2].parse::<i128>().unwrap_or(item);
                    let divisible_by = apes[j][3][0].parse::<i128>().unwrap();
                    let new_ape_1 = apes[j][4][0].parse::<i128>().unwrap();
                    let new_ape_2 = apes[j][5][0].parse::<i128>().unwrap();

                    // println!("item: {}, action: {}, value: {}, divisible_by: {}, new_ape_1: {}, new_ape_2: {}", item, action, value, divisible_by, new_ape_1, new_ape_2);

                    match action.as_str() {
                        "+" => {
                            item += value;

                            item /= 3
                        },
                        "*" => {
                            item *= value;

                            item /= 3
                        },
                        _ => panic!("Invalid action"),
                    }

                    if item % divisible_by == 0 {
                        apes[new_ape_1 as usize][1].push(item.to_string());
                    } else {
                        apes[new_ape_2 as usize][1].push(item.to_string());
                    }

                    apes[j][1].remove(0);
                    inspections[j] += 1;

                }
            }
        }


        inspections.sort();

        let inspections = inspections.iter().rev().take(2).collect::<Vec<&i32>>();
        println!("{:?}", inspections);

        println!("{:?}", inspections[0] * inspections[1]);


        return 0;
    }

    pub fn day_11_task_2() -> i32 {
        let contents = InputReader::read_input_day_11();
        println!("{}", contents);

        let apes = contents.split("\r\n\r\n").collect::<Vec<&str>>();

        println!("{:#?}", apes);

        let apes = apes.iter()
            .map(|ape| ape.lines()
                .map(|line| line.chars()
                    .filter(|c| c != &',' && c != &':')
                    .collect::<String>())
                .collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();

        let mut apes = apes.iter()
            .map(|ape| ape.iter()
                .map(|line| line.split_whitespace()
                    .filter(|s| s.parse::<i32>().is_ok() || s == &"+" || s == &"*" || s == &"old")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>())
            .collect::<Vec<Vec<Vec<String>>>>();

        for ape in &apes {
            println!("{:?}", ape);
        }

        let mut inspections = vec![0; apes.len()];

        let shorten_number = apes.iter().map(|ape| ape[3][0].parse::<i128>().unwrap()).product::<i128>();
        println!("{}", shorten_number);

        for i in 0..10_000 {
            for j in 0..apes.len() {
                // println!("apes -> {:?}: {:?}", j, apes[j]);
                for item in apes[j][1].clone() {
                    let mut item = item.parse::<i128>().unwrap();
                    let action = &apes[j][2][1];
                    let value = apes[j][2][2].parse::<i128>().unwrap_or(item);
                    let divisible_by = apes[j][3][0].parse::<i128>().unwrap();
                    let new_ape_1 = apes[j][4][0].parse::<i128>().unwrap();
                    let new_ape_2 = apes[j][5][0].parse::<i128>().unwrap();

                    // println!("item: {}, action: {}, value: {}, divisible_by: {}, new_ape_1: {}, new_ape_2: {}", item, action, value, divisible_by, new_ape_1, new_ape_2);

                    match action.as_str() {
                        "+" => {
                            item += value;

                            item %= shorten_number
                        },
                        "*" => {
                            item *= value;

                            item %= shorten_number
                        },
                        _ => panic!("Invalid action"),
                    }

                    if item % divisible_by == 0 {
                        apes[new_ape_1 as usize][1].push(item.to_string());
                    } else {
                        apes[new_ape_2 as usize][1].push(item.to_string());
                    }

                    apes[j][1].remove(0);
                    inspections[j] += 1;

                }
            }
        }

        println!("{:?}", inspections);


        inspections.sort();

        let inspections = inspections.iter().rev().take(2).collect::<Vec<&i128>>();
        println!("{:?}", inspections);

        println!("{:?}", inspections[0] * inspections[1]);


        return 0;
    }
}