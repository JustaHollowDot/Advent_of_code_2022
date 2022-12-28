use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn day_4_task_1() {
        let content = read_to_string(r"input\day_4\day_4_input").unwrap();
        let mut amount = 0;

        for line in content.lines() {
            let elves: Vec<&str> = line.split(",").collect();

            let ranges: Vec<Vec<i32>> = elves.iter().map(|x| {
                x.split("-").map(|x|x.parse::<i32>().unwrap()).collect()
            }).collect();


            let mut range = ranges[0][0]..=ranges[0][1];
            let mut range_2 = ranges[1][0]..=ranges[1][1];

            if range.contains(&range_2.clone().next().unwrap()) && range.contains(&range_2.clone().last().unwrap()) {
                amount += 1;
            } else if range_2.contains(&range.clone().next().unwrap()) && range_2.contains(&range.clone().last().unwrap()) {
                amount += 1;
            }

        }

        println!("{}", amount);


    }

    pub fn day_4_task_2() {
        let content = read_to_string(r"input\day_4\day_4_input").unwrap();
        let mut amount = 0;

        for line in content.lines() {
            let elves: Vec<&str> = line.split(",").collect();

            let ranges: Vec<Vec<i32>> = elves.iter().map(|x| {
                x.split("-").map(|x|x.parse::<i32>().unwrap()).collect()
            }).collect();


            let mut range = ranges[0][0]..=ranges[0][1];
            let mut range_2 = ranges[1][0]..=ranges[1][1];

            if range.contains(&range_2.clone().next().unwrap()) || range.contains(&range_2.clone().last().unwrap()) {
                amount += 1;
            } else if range_2.contains(&range.clone().next().unwrap()) || range_2.contains(&range.clone().last().unwrap()) {
                amount += 1;
            }

        }

        println!("{}", amount);


    }
}