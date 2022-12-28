use std::cmp::max;
use std::fs::read_to_string;
use crate::Solution;

impl Solution{
    pub fn day_1_task_1() {
        let contents: String = read_to_string("../input/day_1/day_1_input").unwrap();

        let mut elves: Vec<i32> = Vec::new();

        let mut new_elf: i32 = 0;

        for line in contents.lines() {
            if line == "" {
                elves.push(new_elf);
                new_elf = 0;

            } else {
                new_elf += line.parse::<i32>().unwrap();
            }
        }

        elves.push(new_elf);

        println!("{:?}", elves);

        println!("{}", elves.iter().max().unwrap());
    }

    pub fn day_1_task_2() {
        let contents: String = read_to_string("../input/day_1/day_1_input").unwrap();

        let mut elves: Vec<i32> = Vec::new();

        let mut new_elf: i32 = 0;

        for line in contents.lines() {
            if line == "" {
                elves.push(new_elf);
                new_elf = 0;

            } else {
                new_elf += line.parse::<i32>().unwrap();
            }
        }

        elves.push(new_elf);

        println!("{:?}", elves);

        elves.sort();
        elves.reverse();

        println!("{:?}", elves);

        let sum = elves[0] + elves[1] + elves[2];
        println!("{}", sum)
    }
}